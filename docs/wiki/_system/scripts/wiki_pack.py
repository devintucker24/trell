#!/usr/bin/env python3
"""Export the portable RepoBrain engine or install compatibility launchers."""

from __future__ import annotations

import argparse
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import PATHS, ROOT, WIKI

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
    "wiki-setup",
]

LAUNCHER = """---
name: {name}
description: {description}
---

# {name}

Deprecated `wiki-*` compatibility launcher.

Canonical playbook:

```text
docs/wiki/_system/skills/{name}/SKILL.md
```

Scripts: `docs/wiki/_system/scripts/`
Operator: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
Host overlay: `docs/wiki/_system/config/HOST.yaml`
"""

DESCRIPTIONS = {
    name: (
        "Deprecated compatibility alias; canonical playbook: "
        f"docs/wiki/_system/skills/{name}/SKILL.md"
    )
    for name in SKILL_NAMES
}


def _copy_tree(src: Path, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if src.is_dir():
        shutil.copytree(src, dest, dirs_exist_ok=True)
    else:
        shutil.copy2(src, dest)


def cmd_export(dest_repo: Path) -> None:
    dest_wiki = dest_repo / "docs" / "wiki"
    dest_system = dest_wiki / "_system"
    dest_system.mkdir(parents=True, exist_ok=True)

    for rel in [
        "README.md",
        "skills",
        "scripts",
        "templates",
        "docs",
        "config/README.md",
        "logs/README.md",
        "generated/README.md",
        "generated/doctor/README.md",
    ]:
        src = PATHS.system / rel
        if not src.exists():
            print(f"skip missing {rel}")
            continue
        if rel == "scripts":
            shutil.copytree(
                src,
                dest_system / rel,
                dirs_exist_ok=True,
                ignore=shutil.ignore_patterns("tests", "__pycache__"),
            )
        else:
            _copy_tree(src, dest_system / rel)

    for rel in ["inbox/README.md", "episodic/INDEX.md"]:
        src = WIKI / rel
        if src.exists():
            _copy_tree(src, dest_wiki / rel)

    for src_rel, dest_rel in [
        ("HOST.template.yaml", "config/HOST.yaml"),
        ("router-seeds.template.md", "config/router-seeds.md"),
    ]:
        dest = dest_system / dest_rel
        if dest.exists():
            print(f"keep existing {dest_rel}")
            continue
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(PATHS.templates / src_rel, dest)
        print(f"wrote stub {dest_rel}")

    print(f"Exported RepoBrain engine → {dest_system}")
    print("Next: in the destination repository run")
    print("  python3 docs/wiki/_system/scripts/wiki_setup.py")
    print("(fills host config, launchers, and Graphify; optional --seed-pages)")


def harness_roots() -> tuple[Path, ...]:
    return tuple(
        ROOT / harness / "skills" for harness in (".cursor", ".claude", ".agents")
    )


def cmd_install_launchers() -> None:
    for harness in harness_roots():
        harness.mkdir(parents=True, exist_ok=True)
        for name in SKILL_NAMES:
            skill_dir = harness / name
            skill_dir.mkdir(parents=True, exist_ok=True)
            text = LAUNCHER.format(name=name, description=DESCRIPTIONS[name])
            (skill_dir / "SKILL.md").write_text(text, encoding="utf-8")
            print(f"wrote {skill_dir / 'SKILL.md'}")

    for harness in harness_roots():
        old = harness / "trell-wiki"
        if old.exists():
            shutil.rmtree(old)
            print(f"removed {old}")


def main() -> None:
    if len(sys.argv) >= 2 and sys.argv[1] == "setup":
        from wiki_setup import main as setup_main

        sys.argv = [sys.argv[0], *sys.argv[2:]]
        setup_main()
        return

    parser = argparse.ArgumentParser(
        description="RepoBrain engine export and compatibility launchers"
    )
    sub = parser.add_subparsers(dest="cmd", required=True)
    export = sub.add_parser("export")
    export.add_argument("dest_repo", type=Path)
    sub.add_parser("install-launchers")
    args = parser.parse_args()
    if args.cmd == "export":
        cmd_export(args.dest_repo.resolve())
    else:
        cmd_install_launchers()


if __name__ == "__main__":
    main()
