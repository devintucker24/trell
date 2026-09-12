---
id: repobrain-install
title: Install RepoBrain into a host repository
type: meta
status: active
created: 2026-09-12
updated: 2026-09-12
tags: [repobrain, setup, install, bootstrap]
domain: meta
summary: Agent pointer to Quick Start — clone the empty engine and stand it up in the current repo.
nodes:
  - id: repobrain-install
    kind: concept
    label: RepoBrain install
edges:
  - from: repobrain-install
    to: wiki-setup
    rel: implements
  - from: repobrain-install
    to: wiki-brain-pack
    rel: depends_on
related:
  - "[[QUICKSTART]]"
  - "[[HOW-IT-WORKS]]"
  - "[[FRAMEWORK]]"
  - "[[_system/skills/repobrain-setup/SKILL]]"
agent:
  priority: critical
  read_when:
    - installing RepoBrain
    - user pastes the bootstrap prompt
    - the host repo has no ./repobrain yet
  maintain:
    - keep the paste prompt only in QUICKSTART.md
---

# Install RepoBrain

Follow **[QUICKSTART.md](QUICKSTART.md)**. That page is the install recipe and
the copy-paste agent prompt.

From a RepoBrain clone:

```bash
./repobrain bootstrap /path/to/your-project
```

Equivalent: `./bootstrap.sh /path/to/your-project`.

Graphify is optional. Wiki retrieve works without it. See
[HOW-IT-WORKS.md](HOW-IT-WORKS.md#graphify-optional) and
[GRAPHIFY.md](GRAPHIFY.md).
