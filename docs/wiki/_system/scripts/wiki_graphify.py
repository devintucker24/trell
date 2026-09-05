#!/usr/bin/env python3
"""Thin argparse shell for RepoBrain's Graphify adapter."""

from __future__ import annotations

import argparse
import subprocess
import sys

from graphify_adapter import (
    GraphifyAdapterError,
    cmd_operation,
    cmd_status,
    cmd_sync,
    find_graphify,
    graph_json_path,
    graphify_cfg,
    load_code_graph,
    run_graphify,
    seedable_god_nodes,
)

# Compatibility exports used by setup, retrieve, doctor, and eval.
__all__ = [
    "graphify_cfg",
    "graph_json_path",
    "find_graphify",
    "run_graphify",
    "load_code_graph",
    "seedable_god_nodes",
    "cmd_sync",
]


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Versioned Graphify code-graph adapter for RepoBrain"
    )
    commands = parser.add_subparsers(dest="cmd", required=True)

    sync = commands.add_parser("sync", help="build Graphify-owned graph artifacts")
    sync.add_argument("--force", action="store_true")
    sync.add_argument("--html", action="store_true")

    status = commands.add_parser("status", help="report compatibility and freshness")
    status.add_argument("--json", action="store_true")

    query = commands.add_parser("query", help="delegate a Graphify graph query")
    query.add_argument("question")
    query.add_argument("--budget", type=int, default=1200)

    path = commands.add_parser("path", help="delegate a shortest-path query")
    path.add_argument("a")
    path.add_argument("b")

    explain = commands.add_parser("explain", help="delegate a node explanation")
    explain.add_argument("node")

    affected = commands.add_parser("affected", help="delegate reverse impact analysis")
    affected.add_argument("node")
    affected.add_argument("--relation", action="append", default=[])
    affected.add_argument("--depth", type=int, default=2)

    gods = commands.add_parser("god-nodes", help="delegate hub analysis")
    gods.add_argument("--top", type=int, default=10)
    gods.add_argument("--json", action="store_true")

    commands.add_parser("export-html", help="delegate Graphify HTML rendering")
    commands.add_parser("export-wiki", help="delegate Graphify structural wiki export")
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        if args.cmd == "sync":
            cmd_sync(force=args.force, html=args.html)
            return 0
        if args.cmd == "status":
            return cmd_status(as_json=args.json)
        if args.cmd == "query":
            return cmd_operation(
                "query", [args.question, "--budget", str(args.budget)]
            )
        if args.cmd == "path":
            return cmd_operation("path", [args.a, args.b])
        if args.cmd == "explain":
            return cmd_operation("explain", [args.node])
        if args.cmd == "affected":
            rest = [args.node, "--depth", str(args.depth)]
            for relation in args.relation:
                rest.extend(["--relation", relation])
            return cmd_operation("affected", rest)
        if args.cmd == "god-nodes":
            rest = ["--top", str(args.top)]
            if args.json:
                rest.append("--json")
            return cmd_operation("god-nodes", rest)
        if args.cmd in ("export-html", "export-wiki"):
            return cmd_operation(args.cmd, [])
    except GraphifyAdapterError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 1
    except subprocess.CalledProcessError as exc:
        return exc.returncode
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
