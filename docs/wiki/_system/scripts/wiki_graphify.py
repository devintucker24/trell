#!/usr/bin/env python3
"""RepoBrain wrapper around Graphify — the machine code graph.

We do not rebuild a code AST graph. Graphify owns `graphify-out/graph.json`.
RepoBrain's generated claim graph is compiled from corpus frontmatter only.

  ./repobrain graph sync
  ./repobrain graph query "who calls TypeChecker"
  ./repobrain graph path Parser TypeChecker
  ./repobrain graph explain TypeChecker
  ./repobrain graph god-nodes
  ./repobrain graph status
  ./repobrain graph export-wiki
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import ROOT, load_host

STD_SKIP = {
    "string", "option", "result", "vec", "hashmap", "box", "vecdeque",
    "error", "ok", "none", "some", "true", "false", "self", "super",
}


def find_graphify() -> Path | None:
    which = shutil.which("graphify")
    if which:
        return Path(which)
    home = Path.home() / ".local/bin/graphify"
    if home.is_file():
        return home
    return None


def graphify_cfg(host: dict | None = None) -> dict:
    host = host if host is not None else load_host()
    cfg = dict(host.get("graphify") or {})
    cfg.setdefault("enabled", True)
    cfg.setdefault("code_only", True)
    cfg.setdefault("out", "graphify-out")
    targets = cfg.get("targets")
    if not targets:
        targets = [p.rstrip("/") for p in (host.get("code_roots") or ["src"]) if p]
    cfg["targets"] = [t.rstrip("/") for t in targets]
    return cfg


def graph_dir(cfg: dict | None = None) -> Path:
    cfg = cfg or graphify_cfg()
    out = Path(cfg.get("out") or "graphify-out")
    if not out.is_absolute():
        out = ROOT / out
    # Graphify --out DIR writes DIR/graphify-out/ when DIR is the repo.
    # If cfg.out is already "graphify-out", the json lives there.
    if out.name == "graphify-out":
        return out
    return out / "graphify-out"


def graph_json_path(cfg: dict | None = None) -> Path:
    return graph_dir(cfg) / "graph.json"


def load_code_graph(cfg: dict | None = None) -> dict:
    path = graph_json_path(cfg)
    if not path.exists():
        return {}
    g = json.loads(path.read_text(encoding="utf-8"))
    # Raw extract uses "edges"; clustered NetworkX node-link uses "links".
    if not g.get("edges"):
        g["edges"] = g.get("links") or []
    return g


def run_graphify(
    args: list[str],
    check: bool = True,
    capture: bool = False,
) -> subprocess.CompletedProcess:
    bin_path = find_graphify()
    if not bin_path:
        raise SystemExit(
            "graphify CLI not found. Install: pip install graphifyy\n"
            "Then ensure ~/.local/bin is on PATH."
        )
    env = os.environ.copy()
    local_bin = str(Path.home() / ".local/bin")
    env["PATH"] = local_bin + os.pathsep + env.get("PATH", "")
    return subprocess.run(
        [str(bin_path), *args],
        cwd=ROOT,
        env=env,
        check=check,
        text=True,
        capture_output=capture,
    )


def cmd_sync(force: bool = False) -> Path:
    cfg = graphify_cfg()
    if not cfg.get("enabled", True):
        raise SystemExit("graphify.enabled is false in HOST.yaml")
    targets = []
    for t in cfg["targets"]:
        p = ROOT / t
        if p.exists():
            targets.append(str(p))
        else:
            print(f"skip missing target {t}")
    if not targets:
        raise SystemExit("No graphify targets exist on disk (see HOST.yaml graphify.targets / code_roots)")

    extract_out = str(ROOT)
    cmd = ["extract", targets[0], "--out", extract_out]
    if cfg.get("code_only", True):
        cmd.append("--code-only")
    if force:
        cmd.append("--force")
    print(" ".join(["graphify", *cmd]))
    run_graphify(cmd)

    # Merge extra code roots into the same graph.json
    extra = targets[1:]
    if extra:
        merged = [str(graph_json_path(cfg))]
        for t in extra:
            tmp = ROOT / ".graphify-tmp" / Path(t).name
            tmp.mkdir(parents=True, exist_ok=True)
            extra_cmd = ["extract", t, "--out", str(tmp)]
            if cfg.get("code_only", True):
                extra_cmd.append("--code-only")
            run_graphify(extra_cmd)
            extra_graph = tmp / "graphify-out" / "graph.json"
            if extra_graph.exists():
                merged.append(str(extra_graph))
        if len(merged) > 1:
            out = str(graph_json_path(cfg))
            run_graphify(["merge-graphs", *merged, "--out", out])
        shutil.rmtree(ROOT / ".graphify-tmp", ignore_errors=True)

    path = graph_json_path(cfg)
    g = load_code_graph(cfg)
    n, e = len(g.get("nodes") or []), len(g.get("edges") or [])
    print(f"code graph → {path} ({n} nodes, {e} edges)")
    return path


def cmd_status() -> int:
    cfg = graphify_cfg()
    bin_path = find_graphify()
    path = graph_json_path(cfg)
    print(f"graphify CLI: {bin_path or 'NOT INSTALLED (pip install graphifyy)'}")
    print(f"enabled: {cfg.get('enabled', True)}")
    print(f"code_only: {cfg.get('code_only', True)}")
    print(f"targets: {cfg.get('targets')}")
    print(f"graph.json: {path} {'OK' if path.exists() else 'MISSING'}")
    if path.exists():
        g = load_code_graph(cfg)
        edges = g.get("edges") or []
        conf = Counter(e.get("confidence") for e in edges)
        rels = Counter(e.get("relation") for e in edges)
        print(f"nodes: {len(g.get('nodes') or [])}")
        print(f"edges: {len(edges)} {dict(conf)}")
        print(f"relations: {rels.most_common(8)}")
        gods = god_nodes(g, top=8)
        print("god nodes:")
        for n in gods:
            print(f"  {n.get('label') or n.get('id')} ({n['degree']} edges) ← {n.get('source_file')}")
    return 0 if path.exists() and bin_path else 1


def god_nodes(graph: dict, top: int = 10) -> list[dict]:
    deg: Counter[str] = Counter()
    for e in graph.get("edges") or []:
        if e.get("source"):
            deg[e["source"]] += 1
        if e.get("target"):
            deg[e["target"]] += 1
    by_id = {n.get("id"): n for n in (graph.get("nodes") or []) if n.get("id")}
    out = []
    for nid, d in deg.most_common(top):
        n = dict(by_id.get(nid) or {"id": nid, "label": nid})
        n["degree"] = d
        out.append(n)
    return out


def seedable_god_nodes(graph: dict, top: int = 12) -> list[dict]:
    """God nodes that can become draft wiki stubs (skip methods and std types)."""
    seeds = []
    for n in god_nodes(graph, top=top * 2):
        label = str(n.get("label") or n.get("id") or "")
        if not label or label.startswith("."):
            continue
        if "(" in label or "'" in label:
            continue
        slug = _kebab(label)
        if slug in STD_SKIP or len(slug) < 3:
            continue
        if not n.get("source_file"):
            continue
        n["slug"] = slug
        seeds.append(n)
        if len(seeds) >= top:
            break
    return seeds


def _kebab(label: str) -> str:
    s = []
    for ch in label.strip():
        if ch.isalnum():
            s.append(ch.lower())
        elif ch in " _-/":
            s.append("-")
    slug = "".join(s).strip("-")
    while "--" in slug:
        slug = slug.replace("--", "-")
    return slug


def cmd_passthrough(verb: str, rest: list[str]) -> None:
    cfg = graphify_cfg()
    path = graph_json_path(cfg)
    if not path.exists():
        raise SystemExit(
            f"No {path}. Run: "
            "./repobrain graph sync"
        )
    args = [verb, *rest]
    if "--graph" not in rest:
        args.extend(["--graph", str(path)])
    run_graphify(args)


def cmd_export_wiki() -> None:
    cfg = graphify_cfg()
    path = graph_json_path(cfg)
    if not path.exists():
        raise SystemExit("sync the code graph first")
    run_graphify(["export", "wiki", "--graph", str(path)])
    wiki = graph_dir(cfg) / "wiki"
    print(f"Graphify structural wiki → {wiki}")
    print("These articles are regenerated AST/community pages, not compiled doctrine.")


def main() -> None:
    ap = argparse.ArgumentParser(description="Graphify code-graph wrapper for RepoBrain")
    sub = ap.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("sync", help="extract/rebuild graph.json from HOST.yaml targets")
    s.add_argument("--force", action="store_true")
    sub.add_parser("status")
    sub.add_parser("god-nodes")
    q = sub.add_parser("query")
    q.add_argument("question")
    q.add_argument("--budget", type=int, default=1200)
    p = sub.add_parser("path")
    p.add_argument("a")
    p.add_argument("b")
    e = sub.add_parser("explain")
    e.add_argument("node")
    sub.add_parser("export-wiki")
    args = ap.parse_args()

    if args.cmd == "sync":
        cmd_sync(force=args.force)
    elif args.cmd == "status":
        raise SystemExit(cmd_status())
    elif args.cmd == "god-nodes":
        g = load_code_graph()
        if not g:
            raise SystemExit("no graph.json — run sync")
        for n in god_nodes(g, 12):
            print(f"{n.get('label')}  degree={n['degree']}  {n.get('source_file')} {n.get('source_location')}")
    elif args.cmd == "query":
        cmd_passthrough("query", [args.question, "--budget", str(args.budget)])
    elif args.cmd == "path":
        cmd_passthrough("path", [args.a, args.b])
    elif args.cmd == "explain":
        cmd_passthrough("explain", [args.node])
    elif args.cmd == "export-wiki":
        cmd_export_wiki()


if __name__ == "__main__":
    main()
