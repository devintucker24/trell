from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import yaml

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

from repobrain_paths import ROOT
from repobrain_eval import (
    SCORE_FLOORS,
    evaluate_answer_fidelity,
    evaluate_setup_fixture,
)
from repobrain_paths import is_wiki_content_page
from wiki_retrieve import lexical_score
from wiki_usage import STRONG_HIT, WEAK_HIT


class RepoBrainEvalTests(unittest.TestCase):
    def test_score_classes_reuse_usage_thresholds(self) -> None:
        self.assertEqual(SCORE_FLOORS["relevant"], WEAK_HIT)
        self.assertEqual(SCORE_FLOORS["strong"], STRONG_HIT)

    def test_read_when_terms_contribute_to_lexical_relevance(self) -> None:
        score = lexical_score(
            ["maritime", "colregs"],
            {"agent": {"read_when": ["maritime safety under COLREGs"]}},
            {"heading": "", "text": ""},
            "applications/safety-pattern.md",
        )

        self.assertGreaterEqual(score, 1.0)

    def test_generated_eval_reports_are_not_corpus_pages(self) -> None:
        self.assertFalse(
            is_wiki_content_page(
                "_system/generated/eval/repobrain-eval-20260905T120000Z.md",
                "repobrain-eval-20260905T120000Z.md",
            )
        )

    def test_retrieve_json_reports_provenance_and_packed_budget(self) -> None:
        proc = subprocess.run(
            [
                sys.executable,
                str(SCRIPTS / "wiki_retrieve.py"),
                "belief certain verify guard",
                "--budget-tokens",
                "600",
                "--json",
                "--no-log",
            ],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        payload = json.loads(proc.stdout)

        self.assertEqual(payload["budget_tokens"], 600)
        self.assertLessEqual(payload["packed_tokens"], 600)
        self.assertTrue(payload["hits"])
        self.assertEqual(payload["hits"][0]["provenance"]["kind"], "compiled")
        self.assertEqual(
            payload["hits"][0]["provenance"]["path"],
            payload["hits"][0]["path"],
        )

    def test_code_graph_evidence_stays_separate_from_claim_hits(self) -> None:
        proc = subprocess.run(
            [
                str(ROOT / "repobrain"),
                "retrieve",
                "TypeChecker",
                "--k",
                "1",
                "--json",
                "--no-log",
                "--code",
            ],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        payload = json.loads(proc.stdout)

        self.assertTrue(payload["hits"])
        self.assertTrue(
            all(hit["provenance"]["kind"] != "code" for hit in payload["hits"])
        )
        self.assertIn("code_graph", payload)
        self.assertIn("query_output", payload["code_graph"])
        self.assertIn("[EXTRACTED", payload["code_graph"]["query_output"])

    def test_answer_fidelity_cannot_use_forbidden_index_evidence(self) -> None:
        config = {
            "tier0": {
                "paths": ["AGENTS.md", "docs/wiki/_system/docs/ROUTER.md"]
            },
            "answer_fidelity": [
                {
                    "id": "guarded-answer",
                    "query_id": "query",
                    "allow_provenance": ["compiled", "meta"],
                    "forbid_paths": ["INDEX.md"],
                    "required_terms": ["only-in-index"],
                    "required_citations": ["core/claim.md"],
                }
            ],
        }
        payloads = {
            "query": {
                "hits": [
                    {
                        "path": "INDEX.md",
                        "anchor": "all",
                        "excerpt": "only-in-index",
                        "provenance": {"kind": "meta"},
                    },
                    {
                        "path": "core/claim.md",
                        "anchor": "claim",
                        "excerpt": "bounded evidence",
                        "provenance": {"kind": "compiled"},
                    },
                ]
            }
        }

        result = evaluate_answer_fidelity(config, payloads)

        self.assertFalse(result.passed)
        item = result.evidence[0]
        self.assertEqual(item["full_corpus_reads"], 0)
        self.assertNotIn(
            "INDEX.md",
            [
                entry["path"]
                for entry in item["context_manifest"]
                if entry["kind"] == "retrieved-excerpt"
            ],
        )
        self.assertIn("required evidence", item["failures"][0])

    def test_cli_propagates_required_retrieval_failure(self) -> None:
        config = {
            "version": 2,
            "budgets": {
                "top_k": 3,
                "minimum_score_class": "strong",
                "retrieved_tokens": 300,
            },
            "queries": [
                {
                    "id": "intentional-failure",
                    "q": "belief certain",
                    "expect_sources": [
                        {
                            "path": "core/does-not-exist.md",
                            "max_rank": 1,
                            "provenance": "compiled",
                        }
                    ],
                }
            ],
        }
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            config_path = tmp_path / "failing.yaml"
            config_path.write_text(yaml.safe_dump(config), encoding="utf-8")
            proc = subprocess.run(
                [
                    sys.executable,
                    str(ROOT / "repobrain"),
                    "eval",
                    "--only",
                    "golden-retrieval",
                    "--config",
                    str(config_path),
                    "--output-dir",
                    str(tmp_path / "reports"),
                ],
                cwd=ROOT,
                check=False,
                capture_output=True,
                text=True,
            )

            self.assertEqual(proc.returncode, 1, proc.stdout + proc.stderr)
            self.assertIn("[FAIL] golden-retrieval", proc.stdout)
            reports = list((tmp_path / "reports").glob("*.json"))
            self.assertEqual(len(reports), 1)
            report = json.loads(reports[0].read_text(encoding="utf-8"))
            self.assertEqual(report["status"], "fail")
            self.assertEqual(report["required_failures"], ["golden-retrieval"])
            self.assertTrue(report["categories"][0]["remediation"])

    def test_busy_docs_fixture_preserves_sources_and_existing_corpus(self) -> None:
        result = evaluate_setup_fixture(ROOT)

        self.assertTrue(result.passed, result.evidence)
        checks = result.evidence[0]["fixture_checks"]
        self.assertTrue(all(checks.values()), checks)
        safety = result.evidence[0]["safety_checks"]
        self.assertTrue(all(safety.values()), safety)


if __name__ == "__main__":
    unittest.main()
