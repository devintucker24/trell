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

    def test_graph_status_json_uses_public_cli_seam(self) -> None:
        proc = run_cli("graph", "status", "--json")

        self.assertEqual(proc.returncode, 0, proc.stderr)
        payload = json.loads(proc.stdout)
        self.assertTrue(payload["cli"]["compatible"])
        self.assertEqual(payload["artifact"]["state"], "ready")
        self.assertIn("EXTRACTED", payload["artifact"]["confidence"])
        self.assertIn(payload["freshness"]["source"], ("fresh", "unknown"))

    def test_graph_affected_and_recovery_help_are_exposed(self) -> None:
        affected = run_cli("graph", "affected", "--help")
        sync = run_cli("graph", "sync", "--help")

        self.assertEqual(affected.returncode, 0, affected.stderr)
        self.assertIn("--depth", affected.stdout)
        self.assertIn("--relation", affected.stdout)
        self.assertEqual(sync.returncode, 0, sync.stderr)
        self.assertIn("--force", sync.stdout)
        self.assertIn("--html", sync.stdout)

    def test_source_status_and_scan_are_installed(self) -> None:
        status = run_cli("source", "status")
        help_scan = run_cli("source", "scan", "--help")

        self.assertEqual(status.returncode, 0, status.stderr)
        self.assertIn("manifest.json", status.stdout)
        self.assertEqual(help_scan.returncode, 0, help_scan.stderr)
        self.assertIn("usage:", help_scan.stdout.lower())
        self.assertIn("--dry-run", help_scan.stdout)

    def test_sources_alias_is_deprecated_but_still_works(self) -> None:
        proc = run_cli("sources", "status")

        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("DEPRECATED:", proc.stderr)
        self.assertIn("manifest.json", proc.stdout)

    def test_dashboard_html_is_health_overview_not_graph_export(self) -> None:
        help_dash = run_cli("dashboard", "--help")
        generated = run_cli("dashboard", "html")

        self.assertEqual(help_dash.returncode, 0, help_dash.stderr)
        self.assertIn("health overview", help_dash.stdout.lower())
        self.assertIn("html", help_dash.stdout.lower())
        self.assertEqual(generated.returncode, 0, generated.stderr)
        self.assertIn("generated/dashboard/index.html", generated.stdout.replace("\\", "/"))

    def test_dashboard_status_reports_owned_locations(self) -> None:
        proc = run_cli("dashboard", "status")

        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("generated/usage/dashboard.md", proc.stdout)
        self.assertIn("generated/dashboard", proc.stdout)


if __name__ == "__main__":
    unittest.main()
