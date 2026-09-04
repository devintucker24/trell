#!/usr/bin/env python3
"""Stand up a wiki-brain in this repo with minimal human steps.

Idempotent: will not overwrite a filled HOST.yaml, existing doctrine pages,
or AGENTS.md that already knows about retrieve.

  python3 docs/wiki/scripts/wiki_setup.py
  python3 docs/wiki/scripts/wiki_setup.py --dry-run
  python3 docs/wiki/scripts/wiki_setup.py --no-graphify
  python3 docs/wiki/scripts/wiki_setup.py --seed-pages
"""

from __future__ import annotations

import argparse
import datetime
import subprocess
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parent))
from wiki_paths import ROOT, WIKI, HOST_PATH, load_host

TODAY = datetime.date.today().isoformat()

CODE_ROOT_CANDIDATES = (
    "src", "lib", "app", "apps", "crates", "packages", "backend", "frontend",
    "examples", "tests",
)
RAW_CANDIDATES = ("THESIS.md", "README.md")

GRAPHIFY_BLOCK = """
graphify:
  enabled: true
  code_only: true
  out: graphify-out
  # AST extract only (no LLM). Override targets if code_roots includes non-code.
  targets:
    - src
"""

INDEX_STUB = """---
id: wiki-index
title: {name} Knowledge Base Index
type: index
status: active
created: '{today}'
updated: '{today}'
tags: [index, navigation]
domain: meta
summary: Master catalog of the {name} wiki brain for agent navigation.
nodes:
  - id: wiki-index
    kind: concept
    label: Wiki Index
edges: []
related:
  - "[[SCHEMA]]"
  - "[[ROUTER]]"
  - "[[FRAMEWORK]]"
agent:
  priority: critical
  read_when:
    - starting any wiki session
    - finding a page
  maintain:
    - update on every structural page add/remove
---

# {name} wiki index

Agent bootstrap: `AGENTS.md` → `docs/wiki/ROUTER.md` → retrieve. Do not dump this INDEX.

```bash
python3 docs/wiki/scripts/wiki_retrieve.py "<question>" --budget-tokens 3500
python3 docs/wiki/scripts/wiki_graphify.py query "<code question>"
```

Fill `HOST.yaml` `anchor`, then ingest real pages via inbox → triage → ingest.
Seed/draft pages (if any) are **not** compiled doctrine until reviewed.
"""

LOG_STUB = """# Wiki-brain operations log (append-only)

## [{today}] schema | wiki-setup bootstrap

- Ran `wiki_setup.py` in this repo
- Fill `HOST.yaml` anchor; review any graphify-seed draft pages
"""

SEED_PAGE = """---
id: {id}
title: "{title} (graphify seed)"
type: concept
status: draft
created: '{today}'
updated: '{today}'
tags: [generated, graphify-seed]
domain: {domain}
summary: "Auto-seeded from Graphify god node {label}; not compiled doctrine until filled from source."
generated_from: graphify
nodes:
  - id: {id}
    kind: concept
    label: "{label}"
edges: []
related:
  - "[[INDEX]]"
implements_code:
  - {code_path}
agent:
  priority: low
  read_when:
    - "filling setup seed pages"
  maintain:
    - "replace this stub with compiled claims; drop graphify-seed tag"
---

# {title}

**Status:** draft seed from Graphify (`{label}` in `{source_file}` {loc}).

This page is a placeholder so a portable install has somewhere to start.
It is **not** wiki truth. An agent should:

1. Read `{code_path}`
2. Write what this symbol *means* for the host project (invariants, contracts)
3. Add typed `edges` (`depends_on`, `implements`, …) to other claim pages
4. Set `status: active` and remove the `graphify-seed` tag

Do not paste the source file into this page. Point at it.
"""


def detect() -> dict:
    name = ROOT.name
    try:
        remote = subprocess.check_output(
            ["git", "remote", "get-url", "origin"],
            cwd=ROOT,
            text=True,
            stderr=subprocess.DEVNULL,
        ).strip()
        # .../foo.git or .../foo
        base = remote.rstrip("/").split("/")[-1]
        if base.endswith(".git"):
            base = base[:-4]
        if base:
            name = base
    except Exception:  # noqa: BLE001
        pass

    code_roots = [c + "/" for c in CODE_ROOT_CANDIDATES if (ROOT / c).exists()]
    raw = [p for p in RAW_CANDIDATES if (ROOT / p).exists()]
    languages = []
    src = ROOT / "src"
    if src.exists():
        exts = {p.suffix for p in src.rglob("*") if p.is_file() and p.suffix}
        languages = sorted(exts)
    return {
        "name": name,
        "code_roots": code_roots or ["src/"],
        "raw": raw,
        "languages": languages,
    }


def ensure_dirs(host: dict, dry: bool) -> list[str]:
    created = []
    dirs = [
        WIKI / "inbox" / "archive",
        WIKI / "episodic",
        WIKI / "temporal",
        WIKI / "raw",
        WIKI / "host",
        WIKI / "_meta",
    ]
    for d in host.get("semantic_dirs") or []:
        dirs.append(WIKI / d)
    for d in host.get("domains") or []:
        if d in ("meta", "episodic", "temporal"):
            continue
        dirs.append(WIKI / d)
    for d in dirs:
        if not d.exists():
            created.append(str(d.relative_to(ROOT)))
            if not dry:
                d.mkdir(parents=True, exist_ok=True)
    return created


def _host_is_template(text: str) -> bool:
    return (
        "name: My Project" in text
        or "One paragraph: what this repo is" in text
    )


def write_host_if_missing(detected: dict, dry: bool) -> str:
    if HOST_PATH.exists() and not _host_is_template(HOST_PATH.read_text(encoding="utf-8")):
        text = HOST_PATH.read_text(encoding="utf-8")
        if "graphify:" not in text:
            if not dry:
                targets = "\n".join(
                    f"    - {t.rstrip('/')}" for t in (detected["code_roots"][:3] or ["src"])
                )
                block = GRAPHIFY_BLOCK.replace("    - src", targets or "    - src")
                HOST_PATH.write_text(text.rstrip() + "\n" + block, encoding="utf-8")
            return "appended graphify block"
        return "kept existing HOST.yaml"
    template = WIKI / "pack" / "HOST.template.yaml"
    data = yaml.safe_load(template.read_text(encoding="utf-8")) if template.exists() else {}
    data = data or {}
    data["name"] = detected["name"]
    data["code_roots"] = detected["code_roots"]
    data["raw"] = detected["raw"]
    data.setdefault("domains", ["core", "meta", "episodic", "temporal"])
    data["semantic_dirs"] = [d for d in data.get("domains", []) if d not in ("meta", "episodic", "temporal")]
    if not data["semantic_dirs"]:
        data["semantic_dirs"] = ["core"]
        if "core" not in data["domains"]:
            data["domains"] = ["core"] + list(data["domains"])
    data["graphify"] = {
        "enabled": True,
        "code_only": True,
        "out": "graphify-out",
        "targets": [t.rstrip("/") for t in detected["code_roots"] if t.rstrip("/") not in ("examples", "tests")][:3]
        or ["src"],
    }
    if not dry:
        HOST_PATH.parent.mkdir(parents=True, exist_ok=True)
        HOST_PATH.write_text(
            "# Host overlay — generated by wiki_setup.py; edit the anchor.\n"
            + yaml.safe_dump(data, sort_keys=False, allow_unicode=True),
            encoding="utf-8",
        )
    return "wrote HOST.yaml from detection"


def write_stubs(host: dict, dry: bool) -> list[str]:
    wrote = []
    index = WIKI / "INDEX.md"
    if not index.exists():
        if not dry:
            index.write_text(
                INDEX_STUB.format(name=host.get("name") or ROOT.name, today=TODAY),
                encoding="utf-8",
            )
        wrote.append("INDEX.md")
    log = WIKI / "log.md"
    if not log.exists():
        if not dry:
            log.write_text(LOG_STUB.format(today=TODAY), encoding="utf-8")
        wrote.append("log.md")
    seeds = WIKI / "host" / "router-seeds.md"
    tmpl = WIKI / "pack" / "router-seeds.template.md"
    if not seeds.exists() and tmpl.exists():
        if not dry:
            seeds.parent.mkdir(parents=True, exist_ok=True)
            seeds.write_text(tmpl.read_text(encoding="utf-8"), encoding="utf-8")
        wrote.append("host/router-seeds.md")
    timeline = WIKI / "temporal" / "TIMELINE.md"
    if not timeline.exists() and not dry:
        timeline.parent.mkdir(parents=True, exist_ok=True)
        timeline.write_text(
            f"# Timeline\n\n## {TODAY}\n\n- — | schema | wiki-setup | pack bootstrap | [[FRAMEWORK]]\n",
            encoding="utf-8",
        )
        wrote.append("temporal/TIMELINE.md")
    return wrote


def semantic_page_count(host: dict) -> int:
    n = 0
    for d in host.get("semantic_dirs") or ["core"]:
        folder = WIKI / d
        if not folder.is_dir():
            continue
        n += len([p for p in folder.glob("*.md") if p.name != "_TEMPLATE.md"])
    return n


def seed_pages(host: dict, dry: bool, force: bool) -> list[str]:
    from wiki_graphify import load_code_graph, seedable_god_nodes, graph_json_path

    wrote = []
    if semantic_page_count(host) > 2 and not force:
        return wrote
    if not graph_json_path().exists():
        return wrote
    graph = load_code_graph()
    domain = (host.get("semantic_dirs") or ["core"])[0]
    dest = WIKI / domain
    if not dry:
        dest.mkdir(parents=True, exist_ok=True)
    code_root = (host.get("graphify") or {}).get("targets") or ["src"]
    prefix = str(code_root[0]).rstrip("/") + "/"
    for n in seedable_god_nodes(graph, top=8):
        path = dest / f"{n['slug']}.md"
        if path.exists():
            continue
        source = n.get("source_file") or ""
        code_path = prefix + source if source and not source.startswith(prefix) else source
        body = SEED_PAGE.format(
            id=n["slug"],
            title=n.get("label") or n["slug"],
            label=n.get("label") or n["slug"],
            today=TODAY,
            domain=domain,
            code_path=code_path or source,
            source_file=source,
            loc=n.get("source_location") or "",
        )
        if not dry:
            path.write_text(body, encoding="utf-8")
        wrote.append(str(path.relative_to(ROOT)))
    return wrote


def install_launchers(dry: bool) -> None:
    if dry:
        return
    script = WIKI / "scripts" / "wiki_pack.py"
    subprocess.run([sys.executable, str(script), "install-launchers"], cwd=ROOT, check=False)


def maybe_patch_agents(dry: bool) -> str:
    agents = ROOT / "AGENTS.md"
    fragment = WIKI / "pack" / "AGENTS.fragment.md"
    if not agents.exists() or not fragment.exists():
        return "no AGENTS.md"
    text = agents.read_text(encoding="utf-8")
    if "wiki_retrieve.py" in text or "Wiki brain (portable pack)" in text:
        return "AGENTS.md already wired"
    if not dry:
        agents.write_text(text.rstrip() + "\n\n" + fragment.read_text(encoding="utf-8"), encoding="utf-8")
    return "appended AGENTS.fragment.md"


def gitignore_graphify(dry: bool) -> str:
    gi = ROOT / ".gitignore"
    marker = "graphify-out/"
    if gi.exists() and marker in gi.read_text(encoding="utf-8"):
        return "gitignore already has graphify-out/"
    if not dry:
        prev = gi.read_text(encoding="utf-8") if gi.exists() else ""
        gi.write_text(prev.rstrip() + "\n\n# Graphify machine code graph (regenerated)\ngraphify-out/\n.graphify-tmp/\n", encoding="utf-8")
    return "gitignore graphify-out/"


def main() -> None:
    ap = argparse.ArgumentParser(description="Stand up wiki-brain in this repo")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--no-graphify", action="store_true")
    ap.add_argument("--seed-pages", action="store_true", help="write draft concept pages from Graphify god nodes if the corpus is empty")
    ap.add_argument("--force-seed", action="store_true")
    args = ap.parse_args()
    dry = args.dry_run

    if not (WIKI / "SCHEMA.md").exists():
        raise SystemExit(
            "Pack files missing. From a repo that already has wiki-brain:\n"
            "  python3 docs/wiki/scripts/wiki_pack.py export /path/to/this-repo\n"
            "then re-run wiki_setup.py"
        )

    detected = detect()
    print(f"repo: {ROOT}")
    print(f"detected name={detected['name']} code_roots={detected['code_roots']} raw={detected['raw']}")

    host_action = write_host_if_missing(detected, dry)
    print(f"HOST.yaml: {host_action}")
    host = load_host() if not dry else {**detected, "semantic_dirs": ["core"], "name": detected["name"]}
    if not host.get("name"):
        host["name"] = detected["name"]

    created = ensure_dirs(host, dry)
    if created:
        print("mkdir: " + ", ".join(created))
    stubs = write_stubs(host, dry)
    if stubs:
        print("stubs: " + ", ".join(stubs))
    print("gitignore: " + gitignore_graphify(dry))
    print("AGENTS.md: " + maybe_patch_agents(dry))
    if not dry:
        install_launchers(dry)
        print("launchers: installed .cursor/skills/wiki-* and .claude/skills/wiki-*")

    graphify_ok = False
    if not args.no_graphify and not dry:
        try:
            from wiki_graphify import cmd_sync, find_graphify
            if not find_graphify():
                print("graphify: NOT INSTALLED — pip install graphifyy  (code graph skipped)")
            else:
                cmd_sync()
                graphify_ok = True
        except SystemExit as e:
            print(f"graphify: {e}")
        except Exception as e:  # noqa: BLE001
            print(f"graphify: failed ({e})")
    elif args.no_graphify:
        print("graphify: skipped (--no-graphify)")

    seeded = []
    if args.seed_pages and not dry and graphify_ok:
        seeded = seed_pages(host, dry, force=args.force_seed)
        print("seed pages: " + (", ".join(seeded) if seeded else "(none — corpus already present or no god nodes)"))
    elif args.seed_pages and dry:
        print("seed pages: (dry-run — would seed from graph.json if corpus empty)")

    if not dry:
        sync = WIKI / "scripts" / "sync_graph.py"
        if sync.exists():
            subprocess.run([sys.executable, str(sync)], cwd=ROOT, check=False)

    print()
    print("Setup complete. Remaining human/agent steps (minimal):")
    print("  1. Edit docs/wiki/HOST.yaml `anchor` — one paragraph the wiki must not dilute")
    print("  2. Fill docs/wiki/host/router-seeds.md with your keywords → pages")
    print("  3. If seed drafts exist, fill them from source (do not paste code into wiki pages)")
    print("  4. pip install graphifyy   # if code graph was skipped")
    print("  5. python3 docs/wiki/scripts/wiki_doctor.py")
    print("  6. python3 docs/wiki/scripts/wiki_retrieve.py \"what is this repo\" --budget-tokens 1500")


if __name__ == "__main__":
    main()
