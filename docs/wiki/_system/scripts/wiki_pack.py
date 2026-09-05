#!/usr/bin/env python3
"""Export the portable RepoBrain engine or install compatibility launchers."""

from __future__ import annotations

import argparse
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from repobrain_paths import PATHS, ROOT, WIKI

SKILL_SUFFIXES = [
    "brain",
    "retrieve",
    "query",
    "navigate",
    "triage",
    "ingest",
    "doctor",
    "heal",
    "lint",
    "label",
    "maintain",
    "usage",
    "setup",
]

CANONICAL_LAUNCHER = """---
name: {name}
description: {description}
---

# {name}

Canonical playbook:

```text
docs/wiki/_system/skills/{name}/SKILL.md
```

Scripts: `docs/wiki/_system/scripts/`
Operator: `docs/wiki/_system/docs/OPERATOR.md`
Router: `docs/wiki/_system/docs/ROUTER.md`
Host overlay: `docs/wiki/_system/config/HOST.yaml`
"""

DEPRECATED_LAUNCHER = """---
name: {alias}
description: Deprecated alias for {name}; use the canonical RepoBrain skill.
---

# {alias}

Deprecated compatibility alias. Use `{name}`.

Canonical playbook:

```text
docs/wiki/_system/skills/{name}/SKILL.md
```
"""

DESCRIPTIONS = {
    "brain": "Operate the RepoBrain repository knowledge engine.",
    "retrieve": "Retrieve ranked evidence from the RepoBrain corpus.",
    "query": "Answer repository questions from cited RepoBrain evidence.",
    "navigate": "Navigate RepoBrain corpus and graph relationships.",
    "triage": "Classify RepoBrain inbox material before ingestion.",
    "ingest": "Promote reviewed material into the RepoBrain corpus.",
    "doctor": "Audit RepoBrain corpus structure and health.",
    "heal": "Repair findings from a RepoBrain doctor report.",
    "lint": "Run the RepoBrain doctor, heal, and recheck workflow.",
    "label": "Normalize RepoBrain page frontmatter.",
    "maintain": "Synchronize code and RepoBrain knowledge.",
    "usage": "Measure RepoBrain retrieval and context usefulness.",
    "setup": "Install or export RepoBrain in a repository.",
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
    _copy_tree(ROOT / "repobrain", dest_repo / "repobrain")

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
    print("  ./repobrain setup")
    print("(fills host config, launchers, and Graphify; optional --seed-pages)")


def harness_roots() -> tuple[Path, ...]:
    return tuple(
        ROOT / harness / "skills" for harness in (".cursor", ".claude", ".agents")
    )


def cmd_install_launchers() -> None:
    for harness in harness_roots():
        harness.mkdir(parents=True, exist_ok=True)
        for suffix in SKILL_SUFFIXES:
            name = f"repobrain-{suffix}"
            skill_dir = harness / name
            skill_dir.mkdir(parents=True, exist_ok=True)
            text = CANONICAL_LAUNCHER.format(
                name=name,
                description=DESCRIPTIONS[suffix],
            )
            (skill_dir / "SKILL.md").write_text(text, encoding="utf-8")
            print(f"wrote {skill_dir / 'SKILL.md'}")

            alias = f"wiki-{suffix}"
            alias_dir = harness / alias
            alias_dir.mkdir(parents=True, exist_ok=True)
            alias_text = DEPRECATED_LAUNCHER.format(alias=alias, name=name)
            (alias_dir / "SKILL.md").write_text(alias_text, encoding="utf-8")
            print(f"wrote {alias_dir / 'SKILL.md'}")

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
