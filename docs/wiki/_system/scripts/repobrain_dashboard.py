"""Generate a local read-only RepoBrain health dashboard."""

from __future__ import annotations

import html
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from repobrain_paths import PATHS, ROOT, load_host


def _escape(value: Any) -> str:
    return html.escape("" if value is None else str(value), quote=True)


def _load_json(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {}
    try:
        loaded = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}
    return loaded if isinstance(loaded, dict) else {}


def _latest_eval() -> dict[str, Any]:
    files = sorted(PATHS.eval_dir.glob("repobrain-eval-*.json")) if PATHS.eval_dir.exists() else []
    return _load_json(files[-1]) if files else {}


def collect_dashboard_data() -> dict[str, Any]:
    doctor = _load_json(PATHS.doctor_latest)
    evaluation = _latest_eval()
    usage = {}
    try:
        from wiki_usage import compute_stats, load_events

        usage = compute_stats(load_events(30))
    except Exception:  # noqa: BLE001
        usage = {}
    graphify = {}
    try:
        from graphify_adapter import status_data

        graphify = status_data()
    except Exception:  # noqa: BLE001
        graphify = {}
    sources = {}
    try:
        from source_pipeline import status_data as source_status

        sources = source_status()
    except Exception:  # noqa: BLE001
        sources = {}
    categories = evaluation.get("categories") or []
    passed = sum(1 for item in categories if item.get("passed"))
    return {
        "generated_at": datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "doctor_score": doctor.get("score"),
        "doctor_counts": doctor.get("counts") or {},
        "eval_status": evaluation.get("status") or "missing",
        "eval_passed": passed,
        "eval_total": len(categories),
        "usage": usage,
        "graphify": graphify,
        "sources": sources,
        "host": (load_host() or {}).get("name") or ROOT.name,
    }


def _compact_mapping(value: Any) -> str:
    if not isinstance(value, dict) or not value:
        return "none"
    return ", ".join(f"{key}={value[key]}" for key in sorted(value))


def _metric_card(label: str, value: Any, note: str) -> str:
    return (
        f'<article class="card"><h2>{_escape(label)}</h2>'
        f'<p class="value">{_escape(value)}</p>'
        f'<p class="note">{_escape(note)}</p></article>'
    )


def _warnings(data: dict[str, Any]) -> list[tuple[str, str, str]]:
    warnings: list[tuple[str, str, str]] = []
    if data["doctor_score"] is None:
        warnings.append(
            (
                "Doctor report is missing.",
                "./repobrain doctor",
                "Run RepoBrain doctor and fix any critical or high findings.",
            )
        )
    if data["eval_status"] != "pass":
        warnings.append(
            (
                "Baseline evaluation is missing or failing.",
                "./repobrain eval",
                "Run the RepoBrain baseline evaluation and inspect the latest report.",
            )
        )
    artifact = (data["graphify"] or {}).get("artifact") or {}
    freshness = (data["graphify"] or {}).get("freshness") or {}
    if artifact.get("state") not in {None, "ready"}:
        warnings.append(
            (
                "Graphify graph is not ready.",
                "./repobrain graph sync --force",
                "Sync the Graphify code graph and re-check status.",
            )
        )
    elif freshness.get("source") == "stale":
        warnings.append(
            (
                "Graphify graph is stale relative to code roots.",
                "./repobrain graph sync",
                "Refresh the Graphify adapter from the current source tree.",
            )
        )
    conversion = (data["sources"] or {}).get("conversion") or {}
    if int(conversion.get("failed") or 0):
        warnings.append(
            (
                "Source conversion failures are recorded.",
                "./repobrain source convert",
                "Retry local document conversion and inspect the source manifest.",
            )
        )
    if not (data["sources"] or {}).get("manifest", {}).get("present"):
        warnings.append(
            (
                "Source inventory manifest has not been generated.",
                "./repobrain source scan",
                "Scan Git-tracked project sources without promoting them.",
            )
        )
    return warnings


def render_html(data: dict[str, Any]) -> str:
    usage = data.get("usage") or {}
    graphify = data.get("graphify") or {}
    sources = data.get("sources") or {}
    warnings = _warnings(data)
    warning_html = "".join(
        (
            "<li>"
            f"<p>{_escape(message)}</p>"
            f'<pre><code data-copy="{_escape(command)}">{_escape(command)}</code></pre>'
            f'<button type="button" class="copy" data-copy="{_escape(command)}">Copy command</button> '
            f'<button type="button" class="copy" data-copy="{_escape(prompt)}">Copy agent prompt</button>'
            "</li>"
        )
        for message, command, prompt in warnings
    ) or "<li>No remediation warnings.</li>"
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>RepoBrain health overview</title>
  <style>
    :root {{ color-scheme: light dark; font-family: ui-sans-serif, system-ui, sans-serif; }}
    body {{ margin: 0 auto; max-width: 960px; padding: 1.25rem; }}
    h1 {{ font-size: 1.6rem; }}
    .grid {{ display: grid; gap: 1rem; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); }}
    .card, .warn {{ border: 1px solid color-mix(in srgb, currentColor 20%, transparent); border-radius: 12px; padding: 1rem; }}
    .value {{ font-size: 1.8rem; margin: 0.2rem 0; }}
    .note, .meta {{ color: color-mix(in srgb, currentColor 65%, transparent); }}
    pre {{ overflow: auto; }}
    button.copy {{ margin: 0.25rem 0.25rem 0 0; }}
    @media (max-width: 640px) {{ body {{ padding: 0.75rem; }} .value {{ font-size: 1.4rem; }} }}
  </style>
</head>
<body>
  <header>
    <p class="meta">Local read-only RepoBrain dashboard · {_escape(data.get("host"))}</p>
    <h1>Health overview</h1>
    <p class="meta">Generated {_escape(data.get("generated_at"))}. No mutation API, queue, or credentials.</p>
  </header>
  <section class="grid">
    {_metric_card("Doctor", data.get("doctor_score") if data.get("doctor_score") is not None else "missing", "Structural health from the latest doctor report")}
    {_metric_card("Evaluation", f"{data.get('eval_passed')}/{data.get('eval_total')} {data.get('eval_status')}", "Pass rate from the latest baseline snapshot")}
    {_metric_card("Retrieval quality", usage.get("usefulness_index") if usage.get("usefulness_index") is not None else "n/a", "Usefulness heuristic from usage telemetry")}
    {_metric_card("Retrieve cost", usage.get("tokens_total_est") if usage.get("tokens_total_est") is not None else "n/a", "Estimated retrieve tokens in the usage window")}
    {_metric_card("Usage events", usage.get("events") if usage.get("events") is not None else "n/a", "Telemetry events in the usage window")}
    {_metric_card("Graphify", ((graphify.get("freshness") or {}).get("source") or (graphify.get("artifact") or {}).get("state") or "missing"), "Adapter freshness, not a second code index")}
    {_metric_card("Sources", (sources.get("manifest") or {}).get("entries") or 0, "Conversion " + _compact_mapping(sources.get("conversion")))}
  </section>
  <section class="warn">
    <h2>Warnings and remediation</h2>
    <ul>{warning_html}</ul>
  </section>
  <script>
    document.querySelectorAll("button.copy").forEach((button) => {{
      button.addEventListener("click", async () => {{
        const value = button.getAttribute("data-copy") || "";
        if (navigator.clipboard && navigator.clipboard.writeText) {{
          await navigator.clipboard.writeText(value);
        }}
      }});
    }});
  </script>
</body>
</html>
"""


def write_dashboard(data: dict[str, Any] | None = None) -> Path:
    payload = data or collect_dashboard_data()
    PATHS.dashboard_dir.mkdir(parents=True, exist_ok=True)
    destination = PATHS.dashboard_dir / "index.html"
    destination.write_text(render_html(payload), encoding="utf-8")
    return destination


def cmd_html(argv: list[str] | None = None) -> int:
    del argv
    path = write_dashboard()
    print(path)
    return 0
