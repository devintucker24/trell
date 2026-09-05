from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[5]
CLI = ROOT / "repobrain"


def run_cli(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [str(CLI), *args],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )


class RepoBrainCliTests(unittest.TestCase):
    def test_top_level_help_lists_technical_groups(self) -> None:
        proc = run_cli("--help")

        self.assertEqual(proc.returncode, 0)
        self.assertIn("RepoBrain technical CLI", proc.stdout)
        for command in (
            "setup",
            "retrieve",
            "graph",
            "source",
            "doctor",
            "eval",
            "usage",
            "dashboard",
        ):
            self.assertIn(command, proc.stdout)

    def test_operator_help_dispatches_through_public_cli(self) -> None:
        for command in ("setup", "retrieve", "graph", "doctor", "eval", "usage"):
            with self.subTest(command=command):
                proc = run_cli(command, "--help")
                self.assertEqual(proc.returncode, 0, proc.stderr)
                self.assertIn("usage:", proc.stdout.lower())

        for command in ("source", "dashboard"):
            with self.subTest(command=command):
                proc = run_cli(command, "--help")
                self.assertEqual(proc.returncode, 0, proc.stderr)
                self.assertIn(f"repobrain {command}", proc.stdout)

    def test_retrieve_delegates_json_output(self) -> None:
        proc = run_cli(
            "retrieve",
            "belief certain verify",
            "--k",
            "1",
            "--json",
            "--no-log",
        )

        self.assertEqual(proc.returncode, 0, proc.stderr)
        payload = json.loads(proc.stdout)
        self.assertTrue(payload["hits"])
        self.assertLessEqual(len(payload["hits"]), 1)

    def test_graph_operator_exit_status_is_propagated(self) -> None:
        proc = run_cli("graph", "not-a-command")

        self.assertEqual(proc.returncode, 2)
        self.assertIn("invalid choice", proc.stderr)

    def test_source_status_and_future_operator_statuses_are_explicit(self) -> None:
        status = run_cli("source", "status")
        unavailable = run_cli("source", "scan")

        self.assertEqual(status.returncode, 0, status.stderr)
        self.assertIn("manifest.json", status.stdout)
        self.assertEqual(unavailable.returncode, 2)
        self.assertIn("operator not installed", unavailable.stderr)

    def test_sources_alias_is_deprecated_but_still_works(self) -> None:
        proc = run_cli("sources", "status")

        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("DEPRECATED:", proc.stderr)
        self.assertIn("manifest.json", proc.stdout)

    def test_dashboard_status_reports_owned_locations(self) -> None:
        proc = run_cli("dashboard", "status")

        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("generated/usage/dashboard.md", proc.stdout)
        self.assertIn("generated/dashboard", proc.stdout)


if __name__ == "__main__":
    unittest.main()
