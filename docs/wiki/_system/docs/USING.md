# Using RepoBrain

After [Quick Start](QUICKSTART.md), stay on these four loops. Do not dump the
wiki into chat.

## Ask a question

```bash
./repobrain retrieve "<question>" --budget-tokens 3500
```

Cite the hit paths. There is no `./repobrain query` command. The
`/repobrain-query` skill means: retrieve, then answer from those hits.

Code wiring (optional Graphify):

```bash
./repobrain graph sync          # if graphify-out/graph.json is missing
./repobrain graph query "<symbol or question>"
```

Open only the `source_file`s Graphify names. Skip Graphify if you are not
asking about code structure. See [HOW-IT-WORKS.md](HOW-IT-WORKS.md#graphify-optional).

## Add a note

Drop messy material in `docs/wiki/inbox/`, or tell an agent:

> Inbox this: \<paste\>

Then **triage** (classify) → **ingest** (write a real page). Inbox items are
not citable until ingested. Playbooks:
[`repobrain-triage`](../skills/repobrain-triage/SKILL.md),
[`repobrain-ingest`](../skills/repobrain-ingest/SKILL.md).

Do not invent a new top-level folder because a note feels important. Add the
domain in `HOST.yaml` first, or leave the item as `needs-human`.

## Check health

```bash
./repobrain doctor
```

Fix critical/high findings. Do not invent taxonomy to silence doctor.
Heal playbook: [`repobrain-heal`](../skills/repobrain-heal/SKILL.md).

## Fill HOST.yaml

The only host overlay you must write by hand:

- `name` — this repository
- `anchor` — one paragraph the wiki must not dilute
- `router-seeds.md` — your keywords → your pages
- `domains` / `semantic_dirs` — folders you actually have

Optional: Graphify roots (only if you installed Graphify), MarkItDown
conversion (PDFs/Office).

## Commands you will actually run

```bash
./repobrain retrieve "<question>" --budget-tokens 3500
./repobrain doctor
./repobrain graph query "<symbol>"    # optional; needs Graphify
```

Full verb list: [`CHEATSHEET.md`](CHEATSHEET.md) (humans). Agents: start at
[`ROUTER.md`](ROUTER.md).
