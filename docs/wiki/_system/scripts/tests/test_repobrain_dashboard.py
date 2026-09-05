from __future__ import annotations

import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1]
import sys

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
                },
                "sources": {
                    "manifest": {"present": True, "entries": 4},
                    "conversion": {"cached": 2, "native": 2},
                },
                "host": '<script>alert("xss")</script>',
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
        self.assertNotIn("action=", html)

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
                },
                "sources": {"manifest": {"present": False}, "conversion": {"failed": 1}},
                "host": "Trell",
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

    def test_cli_html_writes_local_dashboard(self) -> None:
        code = main(["dashboard", "html"])
        self.assertEqual(code, 0)
        path = dashboard.PATHS.dashboard_dir / "index.html"
        self.assertTrue(path.is_file())
        text = path.read_text(encoding="utf-8")
        self.assertIn("Health overview", text)
        self.assertIn("Doctor", text)


if __name__ == "__main__":
    unittest.main()
