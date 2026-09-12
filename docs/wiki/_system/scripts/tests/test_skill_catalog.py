from __future__ import annotations

import unittest
from pathlib import Path
import sys

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

import wiki_pack
from repobrain_catalog import (
    CLI_COMMANDS,
    CLI_VERBS,
    PLAYBOOK_ONLY,
    SKILL_CLI,
    SKILL_PROMPTS,
    SKILL_SUFFIXES,
)
from repobrain_cli import COMMAND_HELP
import repobrain_dashboard as dashboard
from repobrain_paths import PATHS, ROOT


class SkillCatalogConsistencyTests(unittest.TestCase):
    def test_suffix_lists_match_disk_and_pack(self) -> None:
        on_disk = tuple(
            sorted(
                path.name.removeprefix("repobrain-")
                for path in PATHS.skills.iterdir()
                if path.is_dir() and path.name.startswith("repobrain-")
            )
        )
        self.assertEqual(tuple(sorted(SKILL_SUFFIXES)), on_disk)
        self.assertEqual(tuple(SKILL_SUFFIXES), tuple(wiki_pack.SKILL_SUFFIXES))
        self.assertEqual(set(SKILL_SUFFIXES), set(wiki_pack.DESCRIPTIONS))
        self.assertEqual(set(SKILL_SUFFIXES), set(SKILL_PROMPTS))

    def test_playbooks_are_not_cli_verbs(self) -> None:
        self.assertTrue(PLAYBOOK_ONLY.isdisjoint(CLI_VERBS))
        self.assertEqual(set(COMMAND_HELP), set(CLI_VERBS))
        self.assertTrue(set(SKILL_CLI).isdisjoint(PLAYBOOK_ONLY))
        for suffix, command in SKILL_CLI.items():
            verb = command.split()[1]
            self.assertIn(verb, CLI_VERBS)
            self.assertEqual(suffix, verb)

    def test_dashboard_catalog_names_every_skill(self) -> None:
        by_id = {item["id"]: item for item in dashboard.command_catalog()}
        for suffix in SKILL_SUFFIXES:
            item = by_id[f"repobrain-{suffix}"]
            self.assertIn(f"/repobrain-{suffix}", item["prompt"])
            if suffix in PLAYBOOK_ONLY:
                self.assertEqual(item["command"], "")
                self.assertIn(f"/repobrain-{suffix}", item["note"])
                self.assertIn(f"not a ./repobrain {suffix} command", item["note"])
                self.assertIn("CLI command", item["prompt"])
            else:
                self.assertTrue(item["command"].startswith("./repobrain "))

    def test_cli_command_ids_are_public_verbs(self) -> None:
        names = {item["name"] for item in CLI_COMMANDS}
        self.assertTrue(names <= set(CLI_VERBS))
        graph = next(item for item in CLI_COMMANDS if item["id"] == "graph")
        self.assertIn("./repobrain graph query", graph["command"])
        self.assertIn("/repobrain-query", graph["prompt"])

    def test_launchers_and_docs_name_each_skill(self) -> None:
        cheatsheet = (PATHS.system / "docs" / "CHEATSHEET.md").read_text(
            encoding="utf-8"
        )
        index = (PATHS.corpus / "INDEX.md").read_text(encoding="utf-8")
        operator = (PATHS.system / "docs" / "OPERATOR.md").read_text(encoding="utf-8")
        inbox = (PATHS.corpus / "inbox" / "README.md").read_text(encoding="utf-8")
        self.assertNotIn("wiki-triage", inbox)
        self.assertNotIn("wiki-ingest", inbox)
        for suffix in SKILL_SUFFIXES:
            name = f"repobrain-{suffix}"
            self.assertTrue((PATHS.skills / name / "SKILL.md").exists(), name)
            self.assertIn(f"/repobrain-{suffix}", cheatsheet)
            self.assertIn(name, index)
            self.assertIn(name, operator)
            for harness in (".cursor", ".claude", ".agents"):
                launcher = ROOT / harness / "skills" / name / "SKILL.md"
                self.assertTrue(launcher.is_file(), str(launcher))
                text = launcher.read_text(encoding="utf-8")
                self.assertIn(f"docs/wiki/_system/skills/{name}/SKILL.md", text)

    def test_engine_docs_require_graph_query_protocol(self) -> None:
        agents_fragment = (PATHS.templates / "AGENTS.fragment.md").read_text(
            encoding="utf-8"
        )
        router = (PATHS.system / "docs" / "ROUTER.md").read_text(encoding="utf-8")
        cheatsheet = (PATHS.system / "docs" / "CHEATSHEET.md").read_text(
            encoding="utf-8"
        )
        retrieve = (PATHS.skills / "repobrain-retrieve" / "SKILL.md").read_text(
            encoding="utf-8"
        )
        query = (PATHS.skills / "repobrain-query" / "SKILL.md").read_text(
            encoding="utf-8"
        )
        self.assertIn("./repobrain graph query", agents_fragment)
        self.assertIn("./repobrain graph sync", router)
        self.assertIn("graphify-out/graph.json", cheatsheet)
        self.assertIn("./repobrain graph query", retrieve)
        self.assertIn("graph sync", retrieve)
        self.assertIn("./repobrain graph query", query)
        self.assertIn("graph sync", query)

    def test_compiler_router_seed_requires_graph_query(self) -> None:
        import yaml

        host = yaml.safe_load((PATHS.config / "HOST.yaml").read_text(encoding="utf-8"))
        if (host or {}).get("name") != "Trell":
            self.skipTest("Trell host overlay")
        seeds = (PATHS.config / "router-seeds.md").read_text(encoding="utf-8")
        agents = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
        claude = (ROOT / "CLAUDE.md").read_text(encoding="utf-8")
        self.assertIn("./repobrain graph query", seeds)
        self.assertIn("graph sync", seeds)
        self.assertIn("Natural Trell", seeds)
        self.assertIn("core/natural-syntax-specification.md", seeds)
        self.assertGreaterEqual(seeds.count("./repobrain graph query"), 2)
        self.assertNotIn("`src/` (code is ground truth)", seeds)
        self.assertIn("./repobrain graph query", agents)
        self.assertIn("graph sync", agents)
        self.assertIn("Do not skip Graphify", agents)
        self.assertIn("./repobrain graph query", claude)
        self.assertIn("graph sync", claude)


if __name__ == "__main__":
    unittest.main()
