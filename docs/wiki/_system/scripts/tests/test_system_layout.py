from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

import yaml

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

from repobrain_catalog import SKILL_SUFFIXES
from repobrain_paths import PATHS, ROOT, is_wiki_content_page


class RepoBrainSystemLayoutTests(unittest.TestCase):
    def test_resolver_owns_every_engine_location(self) -> None:
        self.assertEqual(PATHS.corpus, ROOT / "docs" / "wiki")
        self.assertEqual(PATHS.system, PATHS.corpus / "_system")
        self.assertEqual(PATHS.config, PATHS.system / "config")
        self.assertEqual(PATHS.skills, PATHS.system / "skills")
        self.assertEqual(PATHS.scripts, PATHS.system / "scripts")
        self.assertEqual(PATHS.templates, PATHS.system / "templates")
        self.assertEqual(PATHS.logs, PATHS.system / "logs")
        self.assertEqual(PATHS.generated, PATHS.system / "generated")
        self.assertEqual(PATHS.graphify, ROOT / "graphify-out")
        self.assertEqual(
            PATHS.source_manifest,
            PATHS.generated / "sources" / "manifest.json",
        )
        self.assertEqual(
            PATHS.dashboard_dir,
            PATHS.generated / "dashboard",
        )

    def test_corpus_filter_excludes_engine_and_compatibility_pointers(self) -> None:
        self.assertTrue(is_wiki_content_page("INDEX.md", "INDEX.md"))
        self.assertTrue(
            is_wiki_content_page(
                "inbox/README.md",
                "README.md",
            )
        )
        self.assertFalse(
            is_wiki_content_page(
                "_system/docs/ROUTER.md",
                "ROUTER.md",
            )
        )
        self.assertFalse(is_wiki_content_page("ROUTER.md", "ROUTER.md"))
        self.assertFalse(
            is_wiki_content_page("_meta/GRAPH.md", "GRAPH.md")
        )

    def test_old_script_shim_directory_is_gone(self) -> None:
        self.assertFalse((ROOT / "docs" / "wiki" / "scripts").exists())

    def test_harness_launchers_point_to_canonical_skills(self) -> None:
        launcher = ROOT / ".cursor" / "skills" / "repobrain-brain" / "SKILL.md"
        if not launcher.is_file():
            self.skipTest("host harness launchers not installed")
        for harness in (".cursor", ".claude", ".agents"):
            canonical = (
                ROOT / harness / "skills" / "repobrain-brain" / "SKILL.md"
            ).read_text(encoding="utf-8")
            self.assertIn(
                "docs/wiki/_system/skills/repobrain-brain/SKILL.md",
                canonical,
            )
            self.assertNotIn("Deprecated", canonical)
            self.assertFalse(
                (ROOT / harness / "skills" / "wiki-brain" / "SKILL.md").exists()
            )

        for suffix in SKILL_SUFFIXES:
            canonical = PATHS.skills / f"repobrain-{suffix}" / "SKILL.md"
            self.assertTrue(canonical.exists())
            self.assertFalse((PATHS.skills / f"wiki-{suffix}" / "SKILL.md").exists())

    def test_retrieve_and_query_skills_cap_misses_at_two_strikes(self) -> None:
        retrieve = (PATHS.skills / "repobrain-retrieve" / "SKILL.md").read_text(
            encoding="utf-8"
        )
        query = (PATHS.skills / "repobrain-query" / "SKILL.md").read_text(
            encoding="utf-8"
        )
        for body in (retrieve, query):
            self.assertIn("Two-strike miss", body)
            self.assertIn("Cap is two", body)
            self.assertIn("Second miss", body)

    def test_export_resolves_from_arbitrary_destination(self) -> None:
        with tempfile.TemporaryDirectory(prefix="repobrain-layout-") as tmp:
            destination = Path(tmp) / "different-repository-name"
            destination.mkdir()
            subprocess.run(
                [
                    sys.executable,
                    str(PATHS.scripts / "wiki_pack.py"),
                    "export",
                    str(destination),
                ],
                cwd=ROOT,
                check=True,
                capture_output=True,
                text=True,
            )
            exported_scripts = (
                destination / "docs" / "wiki" / "_system" / "scripts"
            )
            probe = subprocess.run(
                [
                    sys.executable,
                    "-c",
                    (
                        "import sys;"
                        f"sys.path.insert(0,{str(exported_scripts)!r});"
                        "from repobrain_paths import PATHS;"
                        "print(PATHS.repository)"
                    ),
                ],
                cwd=destination,
                check=True,
                capture_output=True,
                text=True,
            )
            self.assertEqual(Path(probe.stdout.strip()), destination)
            self.assertTrue(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "config"
                    / "HOST.yaml"
                ).exists()
            )
            exported_cli = destination / "repobrain"
            self.assertTrue(exported_cli.exists())
            help_proc = subprocess.run(
                [str(exported_cli), "--help"],
                cwd=destination,
                check=True,
                capture_output=True,
                text=True,
            )
            self.assertIn("RepoBrain technical CLI", help_proc.stdout)
            self.assertFalse(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "core"
                    / "epistemic-foundations.md"
                ).exists()
            )
            self.assertTrue(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "scripts"
                    / "tests"
                    / "test_system_layout.py"
                ).exists()
            )
            self.assertFalse(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "scripts"
                    / "apply_frontmatter_and_sync_graph.py"
                ).exists()
            )
            self.assertFalse(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "docs"
                    / "brain-gap-analysis-2026-09-04.md"
                ).exists()
            )
            self.assertTrue(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "inbox"
                    / "_TEMPLATE.md"
                ).exists()
            )
            self.assertTrue(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "config"
                    / "eval-queries.yaml"
                ).exists()
            )
            episodic = (
                destination / "docs" / "wiki" / "episodic" / "INDEX.md"
            ).read_text(encoding="utf-8")
            self.assertNotIn("2026-09-04-brain-memory-upgrade", episodic)
            self.assertIn("compiled wiki truth", episodic)
            operator = (
                destination / "docs" / "wiki" / "_system" / "docs" / "OPERATOR.md"
            ).read_text(encoding="utf-8")
            self.assertNotIn("examples/*.trell", operator)
            self.assertNotIn("COLREGs", operator)
            framework = (
                destination / "docs" / "wiki" / "_system" / "docs" / "FRAMEWORK.md"
            ).read_text(encoding="utf-8")
            self.assertIn("./repobrain bootstrap", framework)
            self.assertNotIn("(or Trell)", framework)
            self.assertNotIn("Plug-in checklist", framework)
            self.assertIn("## Install checklist", framework)
            schema = (
                destination / "docs" / "wiki" / "_system" / "docs" / "SCHEMA.md"
            ).read_text(encoding="utf-8")
            self.assertNotIn("typecheck.rs", schema)
            self.assertNotIn("belief reduces to certain", schema)
            self.assertTrue(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "docs"
                    / "INSTALL.md"
                ).exists()
            )
            quickstart = (
                destination / "docs" / "wiki" / "_system" / "docs" / "QUICKSTART.md"
            ).read_text(encoding="utf-8")
            self.assertIn("./repobrain bootstrap", quickstart)
            how = (
                destination / "docs" / "wiki" / "_system" / "docs" / "HOW-IT-WORKS.md"
            ).read_text(encoding="utf-8")
            self.assertIn("https://github.com/Graphify-Labs/graphify", how)
            self.assertIn("You do not have to install or use Graphify", how)
            self.assertIn("./repobrain retrieve", how)
            self.assertIn("/repobrain-query", how)
            self.assertIn("/repobrain-retrieve", how)
            self.assertIn("Do not collapse", how)
            self.assertIn("does **not** expand synonyms", how)
            self.assertIn("# miss: no-lexical-match", how)
            self.assertIn("second miss", how.lower())
            using = (
                destination / "docs" / "wiki" / "_system" / "docs" / "USING.md"
            ).read_text(encoding="utf-8")
            self.assertIn("[0.833] inbox/README.md", using)
            self.assertIn("Do **not** rename this to `/repobrain-retrieve`", using)
            self.assertIn("does not know synonyms", using.replace("\n", " "))
            inbox = (
                destination / "docs" / "wiki" / "inbox" / "README.md"
            ).read_text(encoding="utf-8")
            self.assertNotIn("2026-09-04-brain-memory-upgrade", inbox)

    def test_bootstrap_copies_engine_without_host_corpus(self) -> None:
        with tempfile.TemporaryDirectory(prefix="repobrain-bootstrap-") as tmp:
            destination = Path(tmp) / "host-app"
            destination.mkdir()
            (destination / "README.md").write_text("# Host App\n", encoding="utf-8")
            proc = subprocess.run(
                [
                    str(ROOT / "repobrain"),
                    "bootstrap",
                    str(destination),
                    "--no-graphify",
                    "--no-sources",
                ],
                cwd=ROOT,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)
            self.assertTrue((destination / "repobrain").exists())
            self.assertTrue(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "_system"
                    / "docs"
                    / "SCHEMA.md"
                ).exists()
            )
            self.assertFalse(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "core"
                    / "epistemic-foundations.md"
                ).exists()
            )
            host = (
                destination / "docs" / "wiki" / "_system" / "config" / "HOST.yaml"
            ).read_text(encoding="utf-8")
            self.assertNotIn("Trell", host)
            loaded = yaml.safe_load(host)
            self.assertEqual(loaded.get("semantic_dirs") or [], [])
            self.assertNotIn("core", loaded.get("domains") or [])
            self.assertFalse((loaded.get("graphify") or {}).get("enabled"))
            self.assertFalse(
                (destination / "docs" / "wiki" / "core").exists()
            )
            self.assertTrue((destination / "AGENTS.md").exists())
            agents = (destination / "AGENTS.md").read_text(encoding="utf-8")
            self.assertIn("./repobrain retrieve", agents)
            self.assertNotIn("Paste into the host project's AGENTS.md", agents)
            self.assertTrue(
                (
                    destination
                    / ".cursor"
                    / "skills"
                    / "repobrain-setup"
                    / "SKILL.md"
                ).exists()
            )


if __name__ == "__main__":
    unittest.main()
