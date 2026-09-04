#!/usr/bin/env python3
"""Export the portable wiki-brain pack or install harness launchers.

  python3 docs/wiki/scripts/wiki_pack.py export /path/to/other-repo
  python3 docs/wiki/scripts/wiki_pack.py install-launchers
"""

from __future__ import annotations

import argparse
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from wiki_paths import ROOT, WIKI

SKILL_NAMES = [
    "wiki-brain",
    "wiki-retrieve",
    "wiki-query",
    "wiki-navigate",
    "wiki-triage",
    "wiki-ingest",
    "wiki-doctor",
    "wiki-heal",
    "wiki-lint",
    "wiki-label",
    "wiki-maintain",
    "wiki-usage",
]

LAUNCHER = """---
name: {name}
description: {description}
---

# {name}

Canonical playbook (portable wiki-brain pack):

```text
docs/wiki/skills/{name}/SKILL.md
```

Scripts: `docs/wiki/scripts/`
Operator: `docs/wiki/OPERATOR.md`
Router: `docs/wiki/ROUTER.md`
Host overlay: `docs/wiki/HOST.yaml`
"""

DESCRIPTIONS = {
    "wiki-brain": "Operate the portable wiki brain over docs/wiki. Canonical: docs/wiki/skills/wiki-brain/SKILL.md",
    "wiki-retrieve": "File-RAG retrieve over docs/wiki. Canonical: docs/wiki/skills/wiki-retrieve/SKILL.md",
    "wiki-query": "Answer from docs/wiki with citations. Canonical: docs/wiki/skills/wiki-query/SKILL.md",
    "wiki-navigate": "Find wiki pages via INDEX/GRAPH. Canonical: docs/wiki/skills/wiki-navigate/SKILL.md",
    "wiki-triage": "Classify docs/wiki/inbox items. Canonical: docs/wiki/skills/wiki-triage/SKILL.md",
    "wiki-ingest": "Ingest triaged wiki inbox items. Canonical: docs/wiki/skills/wiki-ingest/SKILL.md",
    "wiki-doctor": "Diagnose wiki health (no edits). Canonical: docs/wiki/skills/wiki-doctor/SKILL.md",
    "wiki-heal": "Apply safe wiki doctor fixes. Canonical: docs/wiki/skills/wiki-heal/SKILL.md",
    "wiki-lint": "Wiki doctor then heal. Canonical: docs/wiki/skills/wiki-lint/SKILL.md",
    "wiki-label": "Normalize wiki frontmatter. Canonical: docs/wiki/skills/wiki-label/SKILL.md",
    "wiki-maintain": "Sync code and wiki GRAPH. Canonical: docs/wiki/skills/wiki-maintain/SKILL.md",
    "wiki-usage": "Wiki usage telemetry and context cost. Canonical: docs/wiki/skills/wiki-usage/SKILL.md",
}


def _copy_tree(src: Path, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if src.is_dir():
        shutil.copytree(src, dest, dirs_exist_ok=True)
    else:
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)


def cmd_export(dest_repo: Path) -> None:
    dest_wiki = dest_repo / "docs" / "wiki"
    dest_wiki.mkdir(parents=True, exist_ok=True)
    for rel in [
        "skills",
        "scripts",
        "pack",
        "SCHEMA.md",
        "OPERATOR.md",
        "ROUTER.md",
        "FRAMEWORK.md",
        "inbox/_TEMPLATE.md",
        "inbox/README.md",
        "episodic/_TEMPLATE.md",
        "_meta/CONTEXT_PROTOCOL.md",
    ]:
        src = WIKI / rel
        if not src.exists():
            print(f"skip missing {rel}")
            continue
        _copy_tree(src, dest_wiki / rel)

    stubs = [
        ("pack/HOST.template.yaml", "HOST.yaml"),
        ("pack/router-seeds.template.md", "host/router-seeds.md"),
    ]
    for src_rel, dest_rel in stubs:
        dest = dest_wiki / dest_rel
        if dest.exists():
            print(f"keep existing {dest_rel}")
            continue
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(WIKI / src_rel, dest)
        print(f"wrote stub {dest_rel}")

    print(f"Exported pack → {dest_wiki}")
    print("Next: fill HOST.yaml, host/router-seeds.md, then wiki_pack.py install-launchers")


def cmd_install_launchers() -> None:
    for harness in (ROOT / ".cursor" / "skills", ROOT / ".claude" / "skills"):
        harness.mkdir(parents=True, exist_ok=True)
        for name in SKILL_NAMES:
            d = harness / name
            d.mkdir(parents=True, exist_ok=True)
            text = LAUNCHER.format(name=name, description=DESCRIPTIONS[name])
            (d / "SKILL.md").write_text(text, encoding="utf-8")
            print(f"wrote {d / 'SKILL.md'}")
    # drop old Trell-branded parent if present
    for harness in (ROOT / ".cursor" / "skills", ROOT / ".claude" / "skills"):
        old = harness / "trell-wiki"
        if old.exists():
            shutil.rmtree(old)
            print(f"removed {old}")


def main() -> None:
    ap = argparse.ArgumentParser(description="Wiki-brain pack export / launchers")
    sub = ap.add_subparsers(dest="cmd", required=True)
    ex = sub.add_parser("export")
    ex.add_argument("dest_repo", type=Path)
    sub.add_parser("install-launchers")
    args = ap.parse_args()
    if args.cmd == "export":
        cmd_export(args.dest_repo.resolve())
    else:
        cmd_install_launchers()


if __name__ == "__main__":
    main()
