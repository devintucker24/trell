# Quick Start

Install RepoBrain into **your** git repository. This copies the engine
(`./repobrain` and `docs/wiki/_system/`). It does not copy another project's
wiki pages.

You need Python 3.10+ and PyYAML. Graphify is optional (see
[HOW-IT-WORKS.md](HOW-IT-WORKS.md#graphify-optional)).

## Humans

From a RepoBrain clone:

```bash
python3 -m pip install --user pyyaml
chmod +x repobrain bootstrap.sh
./repobrain bootstrap /path/to/your-project
```

Equivalent: `./bootstrap.sh /path/to/your-project`.

Then in **your** project:

1. Edit `docs/wiki/_system/config/HOST.yaml`: set `name` and `anchor` from
   this repo's README. `anchor` is one paragraph agents must not dilute.
2. Map a few keywords in `docs/wiki/_system/config/router-seeds.md`.
3. `./repobrain doctor`
4. `./repobrain retrieve "what is this repo" --budget-tokens 1500`

Do not invent wiki folders. Add domains in `HOST.yaml` first.

## Paste this into an agent (in the host repo)

```text
Install RepoBrain into this repository.

RepoBrain is a portable file-native knowledge engine. Clone it, copy the
engine into this repo, and run setup. Do not copy any other project's wiki
pages.

1. Require Python 3.10+ and `python3 -m pip install --user pyyaml`.
2. If ./repobrain is missing:
   git clone --depth 1 https://github.com/devintucker24/RepoBrain.git /tmp/RepoBrain
   chmod +x /tmp/RepoBrain/repobrain /tmp/RepoBrain/bootstrap.sh
   /tmp/RepoBrain/repobrain bootstrap "$PWD"
   (equivalent: /tmp/RepoBrain/bootstrap.sh "$PWD")
3. If ./repobrain already exists: ./repobrain setup
4. Edit docs/wiki/_system/config/HOST.yaml: set `name` and `anchor` from this
   repo's README. Map keywords in docs/wiki/_system/config/router-seeds.md.
   Do not invent new wiki folders.
5. ./repobrain doctor
6. ./repobrain retrieve "what is this repo" --budget-tokens 1500
7. Stop. Remaining pages are written via inbox → triage → ingest.

Done when: ./repobrain --help works, HOST.yaml has a real anchor, doctor has
no critical/high findings you introduced, and this repo has no other project's
compiled wiki pages.
```

The clone URL is private today: the agent needs GitHub auth.

## Done when

| Step | Done when |
|------|-----------|
| Bootstrap | host has `./repobrain` and `docs/wiki/_system/docs/SCHEMA.md` |
| Overlay | `HOST.yaml` `name` is this repo; `anchor` is not the template sentence |
| Verify | `./repobrain retrieve "what is this repo"` returns host hits |

Next: [USING.md](USING.md). Concepts: [HOW-IT-WORKS.md](HOW-IT-WORKS.md).
