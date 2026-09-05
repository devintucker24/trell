from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

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
                "core/epistemic-foundations.md",
                "epistemic-foundations.md",
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

    def test_old_script_path_delegates_with_deprecation_message(self) -> None:
        proc = subprocess.run(
            [
                sys.executable,
                "docs/wiki/scripts/wiki_retrieve.py",
                "belief certain",
                "--k",
                "1",
                "--json",
                "--no-log",
            ],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        )
        payload = json.loads(proc.stdout)
        self.assertTrue(payload["hits"])
        self.assertIn("DEPRECATED:", proc.stderr)
        self.assertIn("docs/wiki/_system/scripts/wiki_retrieve.py", proc.stderr)

    def test_harness_launchers_point_to_canonical_skills(self) -> None:
        for harness in (".cursor", ".claude", ".agents"):
            launcher = (
                ROOT / harness / "skills" / "wiki-brain" / "SKILL.md"
            ).read_text(encoding="utf-8")
            self.assertIn(
                "docs/wiki/_system/skills/wiki-brain/SKILL.md",
                launcher,
            )
            self.assertIn("Deprecated", launcher)

        for skill_dir in PATHS.skills.iterdir():
            if not (skill_dir / "SKILL.md").exists():
                continue
            compatibility = (
                PATHS.corpus / "skills" / skill_dir.name / "SKILL.md"
            ).read_text(encoding="utf-8")
            self.assertIn(
                f"docs/wiki/_system/skills/{skill_dir.name}/SKILL.md",
                compatibility,
            )

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
            self.assertFalse(
                (
                    destination
                    / "docs"
                    / "wiki"
                    / "core"
                    / "epistemic-foundations.md"
                ).exists()
            )


if __name__ == "__main__":
    unittest.main()
