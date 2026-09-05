# RepoBrain engine

`_system/` contains the portable RepoBrain implementation. The surrounding
`docs/wiki/` tree is the host repository's knowledge corpus.

| Path | Owner | Contents |
|---|---|---|
| `config/` | host + engine | Host overlay, router seeds, evaluation contract |
| `skills/repobrain-*` | engine | Canonical agent playbooks |
| `skills/wiki-*` | compatibility | Deprecated aliases |
| `scripts/` | engine | Deterministic operators and path resolver |
| `templates/` | engine | Export/setup templates |
| `docs/` | engine | Schema, operator, cheat sheet, context, and graph protocols |
| `logs/` | engine | Append-only operator history |
| `generated/` | engine | Claim graph, doctor/eval/usage machine artifacts |

Graphify owns `../../../graphify-out/`; it is deliberately outside this tree.
Compatibility entry points under the former paths delegate here during the
expand–migrate–contract transition.
