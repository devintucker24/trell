from __future__ import annotations

import contextlib
import io
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

import graphify_adapter as adapter
import wiki_graphify


def raw_graph(*, edge_key: str = "edges", commit: str | None = None) -> dict:
    graph = {
        "directed": True,
        "nodes": [
            {"id": "parser", "label": "Parser", "source_file": "src/parser.rs"},
            {"id": "ast", "label": "AST", "source_file": "src/ast.rs"},
        ],
        edge_key: [
            {
                "source": "parser",
                "target": "ast",
                "relation": "calls",
                "confidence": "EXTRACTED",
            },
            {
                "source": "ast",
                "target": "parser",
                "relation": "related_to",
                "confidence": "INFERRED",
            },
            {
                "source": "parser",
                "target": "ast",
                "relation": "may_use",
                "confidence": "AMBIGUOUS",
            },
            {"source": "ast", "target": "parser", "relation": "contains"},
        ],
    }
    if commit:
        graph["graph"] = {"built_commit": commit}
    return graph


class AdapterTempTest(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.src = self.root / "src"
        self.src.mkdir()
        self.graph_dir = self.root / "graphify-out"
        self.graph_dir.mkdir()
        self.host = {
            "code_roots": ["src"],
            "graphify": {
                "enabled": True,
                "roots": ["src"],
                "out": "graphify-out",
            },
        }
        self.root_patch = mock.patch.object(adapter, "ROOT", self.root)
        self.root_patch.start()
        self.load_host_patch = mock.patch.object(
            adapter, "load_host", side_effect=lambda: self.host
        )
        self.load_host_patch.start()

    def tearDown(self) -> None:
        self.load_host_patch.stop()
        self.root_patch.stop()
        self.temp.cleanup()

    def write_graph(self, value: object) -> Path:
        path = self.graph_dir / "graph.json"
        path.write_text(json.dumps(value), encoding="utf-8")
        return path


class ConfigurationTests(unittest.TestCase):
    def test_multiple_roots_and_default_excludes_are_normalized(self) -> None:
        cfg = adapter.graphify_cfg(
            {"code_roots": ["src/", "packages/api/"], "graphify": {}}
        )

        self.assertEqual(cfg["roots"], ["src", "packages/api"])
        self.assertEqual(cfg["targets"], cfg["roots"])
        self.assertEqual(cfg["excludes"], list(adapter.DEFAULT_EXCLUDES))
        self.assertEqual(cfg["requirement"], "graphifyy>=0.9.54,<0.10")

    def test_legacy_targets_and_explicit_excludes_remain_supported(self) -> None:
        cfg = adapter.graphify_cfg(
            {
                "graphify": {
                    "targets": ["lib/"],
                    "excludes": ["**/snapshots/**", "**/snapshots/**"],
                }
            }
        )

        self.assertEqual(cfg["roots"], ["lib"])
        self.assertEqual(cfg["targets"], ["lib"])
        self.assertEqual(cfg["excludes"], ["**/snapshots/**"])

    def test_absolute_root_is_not_stripped(self) -> None:
        cfg = adapter.graphify_cfg({"graphify": {"roots": ["/"]}})

        self.assertEqual(cfg["roots"], ["/"])


class CliCompatibilityTests(unittest.TestCase):
    def test_absent_cli_has_exact_requirement_and_install_command(self) -> None:
        with (
            mock.patch.object(adapter, "find_graphify", return_value=None),
            mock.patch.object(adapter, "_distribution_version", return_value=None),
        ):
            info = adapter.cli_info()

        self.assertFalse(info.compatible)
        self.assertIn("graphifyy>=0.9.54,<0.10", info.diagnostic)
        self.assertIn(adapter.GRAPHIFY_INSTALL_COMMAND, info.diagnostic)
        self.assertEqual(
            adapter.GRAPHIFY_INSTALL_COMMAND,
            "python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'",
        )

    def test_installed_package_without_cli_explains_path_problem(self) -> None:
        with (
            mock.patch.object(adapter, "find_graphify", return_value=None),
            mock.patch.object(
                adapter, "_distribution_version", return_value="0.9.54"
            ),
        ):
            info = adapter.cli_info()

        self.assertIn("0.9.54", info.diagnostic)
        self.assertIn("not on PATH", info.diagnostic)

    def test_incompatible_cli_reports_detected_version(self) -> None:
        proc = subprocess.CompletedProcess([], 0, "graphify 0.10.0\n", "")
        with (
            mock.patch.object(
                adapter, "find_graphify", return_value=Path("/bin/graphify")
            ),
            mock.patch.object(adapter.subprocess, "run", return_value=proc),
        ):
            info = adapter.cli_info()

        self.assertEqual(info.version, "0.10.0")
        self.assertFalse(info.compatible)
        self.assertIn(adapter.GRAPHIFY_REQUIREMENT, info.diagnostic)

    def test_supported_cli_is_compatible(self) -> None:
        proc = subprocess.CompletedProcess([], 0, "graphify 0.9.54\n", "")
        with (
            mock.patch.object(
                adapter, "find_graphify", return_value=Path("/bin/graphify")
            ),
            mock.patch.object(adapter.subprocess, "run", return_value=proc),
        ):
            info = adapter.cli_info()

        self.assertTrue(info.compatible)
        self.assertEqual(info.version, "0.9.54")

    def test_unparseable_cli_version_is_not_masked_by_package_version(self) -> None:
        proc = subprocess.CompletedProcess([], 0, "graphify development build\n", "")
        with (
            mock.patch.object(
                adapter, "find_graphify", return_value=Path("/bin/graphify")
            ),
            mock.patch.object(
                adapter, "_distribution_version", return_value="0.9.54"
            ),
            mock.patch.object(adapter.subprocess, "run", return_value=proc),
        ):
            info = adapter.cli_info()

        self.assertFalse(info.compatible)
        self.assertIsNone(info.version)
        self.assertIn("development build", info.diagnostic)

    def test_checked_native_failure_becomes_actionable_adapter_error(self) -> None:
        info = adapter.CliInfo(
            Path("/bin/graphify"), "0.9.54", True, "compatible"
        )
        proc = subprocess.CompletedProcess([], 7, "", "native failure")
        with (
            mock.patch.object(adapter, "cli_info", return_value=info),
            mock.patch.object(adapter.subprocess, "run", return_value=proc),
            self.assertRaisesRegex(
                adapter.GraphifyAdapterError, "exited 7.*native failure"
            ),
        ):
            adapter.run_graphify(["extract", "src"])


class NormalizationTests(AdapterTempTest):
    def test_raw_edges_are_normalized_read_only_with_confidence(self) -> None:
        self.write_graph(raw_graph(edge_key="edges"))

        graph = adapter.load_code_graph()

        self.assertEqual(graph.schema, "edges")
        self.assertEqual(len(graph["nodes"]), 2)
        self.assertEqual(graph["edges"][1]["confidence"], "INFERRED")
        self.assertEqual(
            graph.confidence_counts,
            {
                "EXTRACTED": 1,
                "INFERRED": 1,
                "AMBIGUOUS": 1,
                "UNQUALIFIED": 1,
            },
        )
        with self.assertRaises(TypeError):
            graph["edges"][0]["confidence"] = "EXTRACTED"

    def test_networkx_links_are_exposed_as_normalized_edges(self) -> None:
        self.write_graph(raw_graph(edge_key="links"))

        graph = adapter.load_code_graph()

        self.assertEqual(graph.schema, "links")
        self.assertEqual(graph["edges"], graph["links"])

    def test_edges_and_links_choose_identical_or_only_populated_shape(self) -> None:
        value = raw_graph()
        value["links"] = list(value["edges"])
        graph = adapter.normalize_graph(value)
        self.assertEqual(graph.schema, "edges+links")

        value["links"] = []
        graph = adapter.normalize_graph(value, source="fixture.json")
        self.assertEqual(graph.schema, "edges")

        value["links"] = [{"source": "different", "target": "edge"}]
        with self.assertRaisesRegex(adapter.GraphArtifactError, "Conflicting"):
            adapter.normalize_graph(value, source="fixture.json")

    def test_malformed_json_is_actionable(self) -> None:
        (self.graph_dir / "graph.json").write_text("{broken", encoding="utf-8")

        with self.assertRaisesRegex(
            adapter.GraphArtifactError, r"sync --force"
        ):
            adapter.load_code_graph()

    def test_partial_and_malformed_shapes_have_field_diagnostics(self) -> None:
        cases = (
            ({}, "nodes"),
            ({"nodes": []}, "edges.*links"),
            ({"nodes": [], "edges": []}, "nodes.*empty"),
            ({"nodes": {}, "edges": []}, "nodes.*list"),
            ({"nodes": [], "edges": {}}, "edges.*list"),
            ({"nodes": [{}], "edges": []}, r"nodes\[0\].*`id`"),
            (
                {"nodes": [{"id": "a"}], "edges": [{"source": "a"}]},
                r"edges\[0\].*`target`",
            ),
        )
        for value, message in cases:
            with self.subTest(value=value):
                with self.assertRaisesRegex(adapter.GraphArtifactError, message):
                    adapter.normalize_graph(value, source="partial.json")

    def test_missing_artifact_is_not_silently_empty(self) -> None:
        with self.assertRaisesRegex(adapter.GraphArtifactError, "missing"):
            adapter.load_code_graph()

    def test_duplicate_nodes_and_dangling_edges_are_rejected(self) -> None:
        duplicate = raw_graph()
        duplicate["nodes"].append(dict(duplicate["nodes"][0]))
        with self.assertRaisesRegex(adapter.GraphArtifactError, "duplicate node"):
            adapter.normalize_graph(duplicate)

        dangling = raw_graph()
        dangling["edges"][0]["target"] = "missing"
        with self.assertRaisesRegex(adapter.GraphArtifactError, "missing nodes"):
            adapter.normalize_graph(dangling)


class StatusTests(AdapterTempTest):
    def test_json_status_reports_cli_schema_counts_confidence_and_html(self) -> None:
        source = self.src / "lib.rs"
        source.write_text("fn main() {}", encoding="utf-8")
        os.utime(source, (100, 100))
        graph_path = self.write_graph(raw_graph(commit="abc123"))
        os.utime(graph_path, (200, 200))
        (self.graph_dir / "graph.html").write_text("<html></html>", encoding="utf-8")
        cli = adapter.CliInfo(
            Path("/bin/graphify"), "0.9.54", True, "compatible"
        )

        with (
            mock.patch.object(adapter, "cli_info", return_value=cli),
            mock.patch.object(adapter, "_git_head", return_value="abc123"),
        ):
            data = adapter.status_data()

        self.assertEqual(data["artifact"]["state"], "ready")
        self.assertEqual(data["artifact"]["schema"], "edges")
        self.assertEqual(data["artifact"]["nodes"], 2)
        self.assertEqual(data["artifact"]["edges"], 4)
        self.assertEqual(data["artifact"]["confidence"]["INFERRED"], 1)
        self.assertEqual(data["freshness"]["built_commit"], "abc123")
        self.assertEqual(data["freshness"]["commit"], "fresh")
        self.assertEqual(data["freshness"]["source"], "fresh")
        self.assertTrue(data["html"]["available"])
        self.assertTrue(data["html"]["fresh"])

    def test_stale_commit_and_newer_source_are_reported(self) -> None:
        source = self.src / "new.rs"
        source.write_text("fn changed() {}", encoding="utf-8")
        graph_path = self.write_graph(raw_graph(commit="old"))
        os.utime(graph_path, (100, 100))
        os.utime(source, (200, 200))
        cli = adapter.CliInfo(
            Path("/bin/graphify"), "0.9.54", True, "compatible"
        )

        with (
            mock.patch.object(adapter, "cli_info", return_value=cli),
            mock.patch.object(adapter, "_git_head", return_value="new"),
        ):
            data = adapter.status_data()

        self.assertEqual(data["freshness"]["commit"], "stale")
        self.assertEqual(data["freshness"]["source"], "stale")

    def test_new_head_without_code_changes_is_source_fresh(self) -> None:
        self.write_graph(raw_graph(commit="old"))
        cli = adapter.CliInfo(
            Path("/bin/graphify"), "0.9.54", True, "compatible"
        )
        with (
            mock.patch.object(adapter, "cli_info", return_value=cli),
            mock.patch.object(adapter, "_git_head", return_value="new"),
            mock.patch.object(
                adapter, "_changed_sources_since", return_value=[]
            ),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            data = adapter.status_data()
            result = adapter.cmd_status()

        self.assertEqual(data["freshness"]["commit"], "stale")
        self.assertEqual(data["freshness"]["source"], "fresh")
        self.assertEqual(data["freshness"]["method"], "git-diff")
        self.assertEqual(result, 0)

    def test_corrupt_status_is_diagnostic_and_nonzero(self) -> None:
        self.write_graph({"nodes": []})
        cli = adapter.CliInfo(
            Path("/bin/graphify"), "0.9.54", True, "compatible"
        )
        with (
            mock.patch.object(adapter, "cli_info", return_value=cli),
            mock.patch.object(adapter, "_git_head", return_value="abc"),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            data = adapter.status_data()
            result = adapter.cmd_status(as_json=True)

        self.assertEqual(data["artifact"]["state"], "corrupt")
        self.assertIn("expected an `edges` or `links`", data["artifact"]["diagnostic"])
        self.assertEqual(result, 1)


class SyncDelegationTests(AdapterTempTest):
    def test_multi_root_excludes_force_and_html_are_native_argv(self) -> None:
        second = self.root / "packages" / "api"
        second.mkdir(parents=True)
        self.host["graphify"].update(
            roots=["src", "packages/api"],
            excludes=["**/vendor/**", "**/generated/**"],
        )
        calls: list[list[str]] = []

        def fake_run(args: list[str], **_: object) -> subprocess.CompletedProcess[str]:
            calls.append(args)
            if args[0] == "extract":
                out_root = Path(args[args.index("--out") + 1])
                output = out_root / "graphify-out"
                output.mkdir(parents=True, exist_ok=True)
                (output / "graph.json").write_text(
                    json.dumps(raw_graph()), encoding="utf-8"
                )
            elif args[0] == "merge-graphs":
                output = Path(args[args.index("--out") + 1])
                output.write_text(json.dumps(raw_graph()), encoding="utf-8")
            elif args[:2] == ["export", "html"]:
                (self.graph_dir / "graph.html").write_text(
                    "<html></html>", encoding="utf-8"
                )
            return subprocess.CompletedProcess(args, 0)

        with (
            mock.patch.object(adapter, "require_compatible_cli"),
            mock.patch.object(adapter, "run_graphify", side_effect=fake_run),
            mock.patch.object(adapter, "_git_head", return_value="abc"),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            result = adapter.cmd_sync(force=True, html=True)

        self.assertEqual(result, self.graph_dir / "graph.json")
        extracts = [call for call in calls if call[0] == "extract"]
        self.assertEqual(len(extracts), 2)
        self.assertEqual(
            [extracts[0][index + 1] for index, value in enumerate(extracts[0]) if value == "--exclude"],
            ["**/vendor/**", "**/generated/**"],
        )
        for call in extracts:
            self.assertIn("--code-only", call)
            self.assertIn("--force", call)
        merge = next(call for call in calls if call[0] == "merge-graphs")
        self.assertEqual(merge[-2], "--out")
        self.assertTrue(merge[-1].endswith("merged-graph.json"))
        self.assertTrue((self.graph_dir / "graph.json").exists())
        self.assertEqual(
            calls[-1],
            ["export", "html", "--graph", str(self.graph_dir / "graph.json")],
        )

    def test_missing_root_fails_before_extraction(self) -> None:
        self.host["graphify"]["roots"] = ["does-not-exist"]
        with (
            mock.patch.object(adapter, "require_compatible_cli"),
            mock.patch.object(adapter, "run_graphify") as run,
            self.assertRaisesRegex(adapter.GraphifyAdapterError, "do not exist"),
        ):
            adapter.cmd_sync()
        run.assert_not_called()

    def test_failed_rebuild_preserves_previous_valid_graph(self) -> None:
        previous = raw_graph()
        self.write_graph(previous)

        def failed_extract(
            args: list[str], **_: object
        ) -> subprocess.CompletedProcess[str]:
            raise adapter.GraphifyAdapterError("extract failed")

        with (
            mock.patch.object(adapter, "require_compatible_cli"),
            mock.patch.object(
                adapter, "run_graphify", side_effect=failed_extract
            ),
            self.assertRaisesRegex(adapter.GraphifyAdapterError, "extract failed"),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            adapter.cmd_sync(force=True)

        self.assertEqual(
            json.loads((self.graph_dir / "graph.json").read_text()),
            previous,
        )

    def test_refactor_reduction_requires_explicit_force(self) -> None:
        self.write_graph(raw_graph())
        reduced = {"nodes": [{"id": "parser"}], "edges": []}

        def write_reduced(
            args: list[str], **_: object
        ) -> subprocess.CompletedProcess[str]:
            out_root = Path(args[args.index("--out") + 1])
            output = out_root / "graphify-out"
            output.mkdir(parents=True, exist_ok=True)
            (output / "graph.json").write_text(
                json.dumps(reduced), encoding="utf-8"
            )
            return subprocess.CompletedProcess(args, 0)

        with (
            mock.patch.object(adapter, "require_compatible_cli"),
            mock.patch.object(
                adapter, "run_graphify", side_effect=write_reduced
            ),
            self.assertRaisesRegex(adapter.GraphifyAdapterError, "fewer nodes"),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            adapter.cmd_sync()

        with (
            mock.patch.object(adapter, "require_compatible_cli"),
            mock.patch.object(
                adapter, "run_graphify", side_effect=write_reduced
            ),
            mock.patch.object(adapter, "_git_head", return_value="abc"),
            contextlib.redirect_stdout(io.StringIO()),
        ):
            adapter.cmd_sync(force=True)

        self.assertEqual(len(adapter.load_code_graph().nodes), 1)


class OperationTests(AdapterTempTest):
    def setUp(self) -> None:
        super().setUp()
        self.write_graph(raw_graph())

    def test_all_operations_delegate_native_argv_and_return_status(self) -> None:
        cases = (
            ("query", ["who calls Parser"], ["query", "who calls Parser"]),
            ("path", ["Parser", "AST"], ["path", "Parser", "AST"]),
            ("explain", ["Parser"], ["explain", "Parser"]),
            (
                "affected",
                ["Parser", "--depth", "3"],
                ["affected", "Parser", "--depth", "3"],
            ),
            ("god-nodes", ["--top", "5"], ["god-nodes", "--top", "5"]),
            ("export-html", [], ["export", "html"]),
            ("export-wiki", [], ["export", "wiki"]),
        )
        for verb, rest, prefix in cases:
            with self.subTest(verb=verb):
                proc = subprocess.CompletedProcess([], 7)
                with mock.patch.object(
                    adapter, "run_graphify", return_value=proc
                ) as run:
                    result = adapter.cmd_operation(verb, rest)
                self.assertEqual(result, 7)
                args = run.call_args.args[0]
                self.assertEqual(args[: len(prefix)], prefix)
                self.assertEqual(
                    args[-2:],
                    ["--graph", str(self.graph_dir / "graph.json")],
                )
                self.assertFalse(run.call_args.kwargs["check"])
                self.assertFalse(run.call_args.kwargs["capture"])

    def test_missing_and_corrupt_artifacts_fail_before_delegation(self) -> None:
        (self.graph_dir / "graph.json").unlink()
        with (
            mock.patch.object(adapter, "run_graphify") as run,
            self.assertRaisesRegex(adapter.GraphArtifactError, "missing"),
        ):
            adapter.cmd_operation("query", ["x"])
        run.assert_not_called()

        self.write_graph({"nodes": []})
        with (
            mock.patch.object(adapter, "run_graphify") as run,
            self.assertRaisesRegex(adapter.GraphArtifactError, "Partial"),
        ):
            adapter.cmd_operation("query", ["x"])
        run.assert_not_called()


class ThinShellTests(unittest.TestCase):
    def test_compatibility_symbols_are_reexported(self) -> None:
        for name in (
            "graphify_cfg",
            "graph_json_path",
            "find_graphify",
            "run_graphify",
            "load_code_graph",
            "seedable_god_nodes",
            "cmd_sync",
        ):
            self.assertTrue(hasattr(wiki_graphify, name), name)

    def test_forbidden_commands_are_absent(self) -> None:
        help_text = wiki_graphify.build_parser().format_help()
        for command in ("hook", "watch", "semantic", "enterprise"):
            self.assertNotIn(command, help_text.lower())

        for command in ("hook", "watch", "semantic", "enterprise"):
            with self.subTest(command=command):
                with (
                    contextlib.redirect_stderr(io.StringIO()),
                    self.assertRaises(SystemExit) as raised,
                ):
                    wiki_graphify.build_parser().parse_args([command])
                self.assertEqual(raised.exception.code, 2)

    def test_shell_propagates_native_operation_status(self) -> None:
        with mock.patch.object(
            wiki_graphify, "cmd_operation", return_value=9
        ) as operation:
            result = wiki_graphify.main(["path", "A", "B"])

        self.assertEqual(result, 9)
        operation.assert_called_once_with("path", ["A", "B"])

    def test_shell_propagates_sync_process_failure_without_traceback(self) -> None:
        failure = subprocess.CalledProcessError(6, ["graphify", "extract"])
        with mock.patch.object(wiki_graphify, "cmd_sync", side_effect=failure):
            result = wiki_graphify.main(["sync", "--force"])

        self.assertEqual(result, 6)


if __name__ == "__main__":
    unittest.main()
