# Local usage events

`events.jsonl` is appended by operators under `docs/wiki/_system/scripts/`.

It is **gitignored**. This file keeps the directory in git.

Regenerate the shareable snapshot:

```bash
python3 docs/wiki/_system/scripts/wiki_usage.py report --days 30
```

See `docs/wiki/_system/docs/usage-telemetry.md`.
