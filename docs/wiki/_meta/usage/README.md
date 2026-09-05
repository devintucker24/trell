# Local usage events

`events.jsonl` is appended by `docs/wiki/scripts/wiki_retrieve.py`, `wiki_doctor.py`, and `wiki_usage.py log`.

It is **gitignored**. This file keeps the directory in git.

Regenerate the shareable snapshot:

```bash
python3 docs/wiki/scripts/wiki_usage.py report --days 30
```

See `docs/wiki/_meta/usage-telemetry.md`.
