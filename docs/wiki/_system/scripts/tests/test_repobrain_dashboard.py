from __future__ import annotations

import unittest
from pathlib import Path
import sys

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

import repobrain_dashboard as dashboard
from repobrain_cli import main


class DashboardHtmlTests(unittest.TestCase):
    def test_render_html_shows_metrics_and_escapes_repo_text(self) -> None:
        html = dashboard.render_html(
            {
                "generated_at": "2026-09-05T00:00:00Z",
                "doctor_score": 97.5,
                "doctor_counts": {"medium": 1},
                "eval_status": "pass",
                "eval_passed": 8,
                "eval_total": 8,
                "usage": {"usefulness_index": 81.2, "tokens_total_est": 1200, "events": 12},
                "graphify": {
                    "artifact": {"state": "ready"},
                    "freshness": {"source": "fresh"},
                    "html": {"available": True, "path": "graphify-out/graph.html"},
                },
                "sources": {
                    "manifest": {"present": True, "entries": 4},
                    "conversion": {"cached": 2, "native": 2},
                    "classifications": {"docs": 3, "data": 1},
                    "failures": [],
                    "policy": {"formats": ["csv"], "commit_groups": [], "cache_gitignored": True},
                },
                "host": '<script>alert("xss")</script>',
                "commands": dashboard.command_catalog(),
                "hot_pages": [("INDEX.md", 4)],
                "unused_pages": ["core/example.md"],
                "graph_src": "../../graphify-out/graph.html",
                "graph_open": "graphify-out/graph.html",
            }
        )
        self.assertIn("97.5", html)
        self.assertIn("8/8 pass", html)
        self.assertIn("81.2", html)
        self.assertIn("1200", html)
        self.assertIn("fresh", html)
        self.assertIn("Conversion cached=2, native=2", html)
        self.assertIn("&lt;script&gt;", html)
        self.assertNotIn('<script>alert("xss")</script>', html)
        self.assertNotIn("<form", html.lower())
        self.assertNotIn("fetch(", html)
        self.assertNotIn("XMLHttpRequest", html)
        self.assertIn("tone-ok", html)
        self.assertIn("Open full graph", html)
        self.assertIn('id="graph-frame"', html)
        self.assertIn("core/example.md", html)
        self.assertIn("INDEX.md", html)
        for suffix in dashboard.SKILL_SUFFIXES:
            self.assertIn(f"repobrain-{suffix}", html)
        self.assertIn("./repobrain graph query", html)

    def test_missing_and_stale_states_include_copyable_remediation(self) -> None:
        html = dashboard.render_html(
            {
                "generated_at": "2026-09-05T00:00:00Z",
                "doctor_score": None,
                "eval_status": "fail",
                "eval_passed": 1,
                "eval_total": 3,
                "usage": {},
                "graphify": {
                    "artifact": {"state": "ready"},
                    "freshness": {"source": "stale"},
                    "html": {"available": False},
                    "cli": {"compatible": True},
                },
                "sources": {
                    "manifest": {"present": False},
                    "conversion": {"failed": 1},
                    "failures": [{"path": "docs/a.pdf", "diagnostic": "broken <tag>"}],
                    "policy": {},
                },
                "host": "Trell",
                "commands": [],
                "hot_pages": [],
                "unused_pages": [],
                "graph_src": None,
                "graph_open": "graphify-out/graph.html",
            }
        )
        self.assertIn("./repobrain doctor", html)
        self.assertIn("./repobrain eval", html)
        self.assertIn("./repobrain graph sync", html)
        self.assertIn("./repobrain source convert", html)
        self.assertIn("./repobrain source scan", html)
        self.assertIn("Copy command", html)
        self.assertIn("Copy agent prompt", html)
        self.assertIn('data-copy="./repobrain doctor"', html)
        self.assertIn("tone-missing", html)
        self.assertIn("tone-warn", html)
        self.assertIn("broken &lt;tag&gt;", html)
        self.assertNotIn("broken <tag>", html)
        self.assertIn("Open full graph", html)
        self.assertNotIn('id="graph-frame"', html)

    def test_corrupt_graphify_uses_adapter_diagnostic(self) -> None:
        html = dashboard.render_html(
            {
                "generated_at": "2026-09-05T00:00:00Z",
                "doctor_score": 100,
                "eval_status": "pass",
                "eval_passed": 1,
                "eval_total": 1,
                "usage": {},
                "graphify": {
                    "artifact": {
                        "state": "corrupt",
                        "diagnostic": "Graph artifact is unreadable",
                    },
                    "cli": {"compatible": False, "diagnostic": "wrong version"},
                },
                "sources": {"manifest": {"present": True, "entries": 1}, "conversion": {}},
                "host": "Trell",
                "commands": dashboard.command_catalog(),
                "graph_src": None,
            }
        )
        self.assertIn("Graph artifact is unreadable", html)
        self.assertIn("./repobrain graph sync --force", html)

    def test_command_catalog_covers_canonical_skills(self) -> None:
        names = {item["id"] for item in dashboard.command_catalog()}
        for suffix in dashboard.SKILL_SUFFIXES:
            self.assertIn(f"repobrain-{suffix}", names)

    def test_cli_html_writes_local_dashboard(self) -> None:
        code = main(["dashboard", "html"])
        self.assertEqual(code, 0)
        path = dashboard.PATHS.dashboard_dir / "index.html"
        self.assertTrue(path.is_file())
        text = path.read_text(encoding="utf-8")
        self.assertIn("Health and exploration", text)
        self.assertIn("Doctor", text)
        self.assertIn("data-tab=\"graph\"", text)


if __name__ == "__main__":
    unittest.main()
