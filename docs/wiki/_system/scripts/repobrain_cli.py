#!/usr/bin/env python3
"""Thin technical command surface for RepoBrain operators."""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

from repobrain_paths import PATHS


OPERATORS = {
    "setup": "wiki_setup.py",
    "retrieve": "wiki_retrieve.py",
    "graph": "wiki_graphify.py",
    "doctor": "wiki_doctor.py",
    "eval": "repobrain_eval.py",
    "usage": "wiki_usage.py",
}

COMMAND_HELP = {
    "setup": "initialize RepoBrain in the current repository",
    "retrieve": "retrieve evidence from the repository corpus",
    "graph": "sync and query the Graphify code graph",
    "source": "inspect source-inventory capabilities and artifacts",
    "doctor": "audit corpus structure and knowledge health",
    "eval": "run the end-to-end RepoBrain baseline",
    "usage": "record and report RepoBrain usage telemetry",
    "dashboard": "generate or locate RepoBrain dashboards",
}


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="repobrain",
        description=(
            "RepoBrain technical CLI for repository knowledge, code graphs, "
            "health, evaluation, and operations."
        ),
    )
    commands = parser.add_subparsers(dest="command", metavar="COMMAND")
    for name, help_text in COMMAND_HELP.items():
        commands.add_parser(name, add_help=False, help=help_text)
    return parser


def _delegate(operator: str, argv: list[str]) -> int:
    """Run one canonical operator with inherited stdio and its exact status."""
    script = PATHS.scripts / OPERATORS[operator]
    return subprocess.run([sys.executable, str(script), *argv]).returncode


def _doctor(argv: list[str]) -> int:
    if argv == ["--help"]:
        print(
            "usage: repobrain doctor [--no-log]\n\n"
            "Audit RepoBrain corpus structure and write doctor reports.\n\n"
            "options:\n"
            "  --no-log  do not append a usage telemetry event"
        )
        return 0
    return _delegate("doctor", argv)


def _source(argv: list[str]) -> int:
    from source_pipeline import cmd_convert, cmd_scan, cmd_status

    parser = argparse.ArgumentParser(
        prog="repobrain source",
        description=(
            "Scan Git-tracked repository sources, convert the local CSV tracer, "
            "and inspect the committed source manifest."
        ),
    )
    commands = parser.add_subparsers(dest="source_command", metavar="COMMAND")
    commands.add_parser("status", help="show the source inventory status", add_help=False)
    commands.add_parser("scan", help="scan Git-tracked sources", add_help=False)
    commands.add_parser("convert", help="convert the local CSV tracer", add_help=False)
    args = parser.parse_known_args(argv)

    command = args[0].source_command
    remainder = args[1] if command else argv
    if command is None:
        parser.print_help()
        return 0
    if command == "status":
        return cmd_status(remainder)
    if command == "scan":
        return cmd_scan(remainder)
    return cmd_convert(remainder)


def _dashboard(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        prog="repobrain dashboard",
        description=(
            "Generate the current usage dashboard or inspect reserved "
            "RepoBrain dashboard locations."
        ),
    )
    commands = parser.add_subparsers(dest="dashboard_command", metavar="COMMAND")
    usage = commands.add_parser("usage", help="generate the Markdown usage dashboard")
    usage.add_argument("--days", type=int, default=30)
    commands.add_parser("status", help="show dashboard artifact locations")
    commands.add_parser("html", help="generate Graphify's code-graph HTML")
    args = parser.parse_args(argv)

    if args.dashboard_command is None:
        parser.print_help()
        return 0
    if args.dashboard_command == "usage":
        return _delegate("usage", ["report", "--days", str(args.days)])
    if args.dashboard_command == "status":
        usage_state = "present" if PATHS.usage_dashboard.exists() else "not generated"
        graph_html = PATHS.graphify / "graph.html"
        graph_state = "present" if graph_html.exists() else "not generated"
        html_state = "present" if PATHS.dashboard_dir.exists() else "not generated"
        print(f"Usage dashboard: {PATHS.usage_dashboard} ({usage_state})")
        print(f"Code graph HTML: {graph_html} ({graph_state})")
        print(f"RepoBrain HTML dashboard: {PATHS.dashboard_dir} ({html_state})")
        return 0
    return _delegate("graph", ["export-html"])


def main(argv: list[str] | None = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    if argv and argv[0] == "sources":
        print("DEPRECATED: use `repobrain source`.", file=sys.stderr)
        argv[0] = "source"

    parser = _parser()
    args, remainder = parser.parse_known_args(argv)
    if args.command is None:
        parser.print_help()
        return 0
    if args.command == "source":
        return _source(remainder)
    if args.command == "dashboard":
        return _dashboard(remainder)
    if args.command == "doctor":
        return _doctor(remainder)
    return _delegate(args.command, remainder)


if __name__ == "__main__":
    raise SystemExit(main())
