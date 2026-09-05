"""Single catalog of RepoBrain skills vs ./repobrain CLI verbs.

Dashboard, pack launchers, and consistency tests import this module so
/repobrain-query cannot be described as missing when only the CLI verb is absent.
"""

from __future__ import annotations

SKILL_SUFFIXES = (
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
)

# Skills with no matching `./repobrain <suffix>` command.
PLAYBOOK_ONLY = frozenset(
    {
        "brain",
        "query",
        "navigate",
        "triage",
        "ingest",
        "heal",
        "lint",
        "label",
        "maintain",
    }
)

# Public CLI verbs from repobrain_cli.COMMAND_HELP (plus source/dashboard subcommands).
CLI_VERBS = (
    "setup",
    "retrieve",
    "graph",
    "source",
    "doctor",
    "eval",
    "usage",
    "dashboard",
)

SKILL_CLI = {
    "retrieve": './repobrain retrieve "<question>" --budget-tokens 3500',
    "doctor": "./repobrain doctor",
    "usage": "./repobrain usage report",
    "setup": "./repobrain setup",
}

SKILL_PROMPTS = {
    "brain": (
        "Follow /repobrain-brain. Operate RepoBrain with the public CLI. "
        "Do not dump the wiki. There is no ./repobrain brain CLI command."
    ),
    "retrieve": (
        "Follow /repobrain-retrieve. Run ./repobrain retrieve \"…\" within "
        "Router budgets. Cite paths."
    ),
    "query": (
        "Follow the /repobrain-query skill. Lookup with ./repobrain retrieve "
        "(there is no ./repobrain query CLI command). Answer from cited hits; "
        "do not invent compiled claims."
    ),
    "navigate": (
        "Follow the /repobrain-navigate skill. Lookup with ./repobrain retrieve "
        "(there is no ./repobrain navigate CLI command). Return wikilinks and "
        "one-line summaries. For code wiring use ./repobrain graph query."
    ),
    "triage": (
        "Follow /repobrain-triage. Classify inbox material. Do not ingest until "
        "triage says to. There is no ./repobrain triage CLI command."
    ),
    "ingest": (
        "Follow /repobrain-ingest. Promote reviewed inbox pages into the compiled "
        "corpus without inventing taxonomy. There is no ./repobrain ingest CLI command."
    ),
    "doctor": (
        "Follow /repobrain-doctor. Run ./repobrain doctor and remediate "
        "critical/high findings."
    ),
    "heal": (
        "Follow /repobrain-heal. Repair the latest doctor findings without "
        "rewriting authority rules. There is no ./repobrain heal CLI command."
    ),
    "lint": (
        "Follow /repobrain-lint. Run doctor, heal if needed, then doctor again. "
        "There is no ./repobrain lint CLI command."
    ),
    "label": (
        "Follow /repobrain-label. Normalize page frontmatter to SCHEMA.md. "
        "There is no ./repobrain label CLI command."
    ),
    "maintain": (
        "Follow /repobrain-maintain. Synchronize code and RepoBrain knowledge "
        "after semantic changes. There is no ./repobrain maintain CLI command."
    ),
    "usage": (
        "Follow /repobrain-usage. Run ./repobrain usage report and say if "
        "retrieve is expensive or weakly hitting."
    ),
    "setup": (
        "Follow /repobrain-setup. Install or refresh RepoBrain with "
        "./repobrain setup. Do not dump the wiki."
    ),
}

CLI_COMMANDS = (
    {
        "id": "cli-setup",
        "name": "setup",
        "description": "Initialize or refresh RepoBrain in this repository.",
        "command": "./repobrain setup",
        "prompt": SKILL_PROMPTS["setup"],
    },
    {
        "id": "cli-retrieve",
        "name": "retrieve",
        "description": "Rank compiled wiki evidence. This is the only corpus lookup verb.",
        "command": SKILL_CLI["retrieve"],
        "prompt": SKILL_PROMPTS["retrieve"],
    },
    {
        "id": "cli-doctor",
        "name": "doctor",
        "description": "Audit corpus structure and knowledge health.",
        "command": "./repobrain doctor",
        "prompt": SKILL_PROMPTS["doctor"],
    },
    {
        "id": "cli-usage",
        "name": "usage",
        "description": "Report retrieval usefulness and token cost.",
        "command": "./repobrain usage report",
        "prompt": SKILL_PROMPTS["usage"],
    },
    {
        "id": "graph",
        "name": "graph",
        "description": "Sync and query the Graphify code graph (not wiki claims).",
        "command": './repobrain graph query "<symbol>"',
        "prompt": (
            "Query Graphify for how … is wired. Do not treat graph HTML as "
            "compiled claims. This is ./repobrain graph query, not /repobrain-query."
        ),
    },
    {
        "id": "source",
        "name": "source",
        "description": "Scan Git-tracked sources and convert configured local formats.",
        "command": "./repobrain source convert",
        "prompt": "Scan and convert local sources. Keep derived Markdown non-authoritative.",
    },
    {
        "id": "eval",
        "name": "eval",
        "description": "Run the end-to-end RepoBrain baseline.",
        "command": "./repobrain eval",
        "prompt": "Run ./repobrain eval and explain any failed category.",
    },
    {
        "id": "dashboard",
        "name": "dashboard",
        "description": "Generate the local read-only HTML dashboard.",
        "command": "./repobrain dashboard html --serve",
        "prompt": (
            "Serve the local HTML dashboard and open the printed "
            "http://127.0.0.1 URL in the browser."
        ),
    },
)
