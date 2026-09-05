"""Generate a local read-only RepoBrain health dashboard."""

from __future__ import annotations

import argparse
import html
import http.server
import json
import os
import sys
import threading
import webbrowser
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from repobrain_catalog import (
    CLI_COMMANDS,
    PLAYBOOK_ONLY,
    SKILL_CLI,
    SKILL_PROMPTS,
    SKILL_SUFFIXES,
)
from repobrain_paths import PATHS, ROOT, is_wiki_content_page, load_host


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


def _skill_description(suffix: str) -> str:
    path = PATHS.skills / f"repobrain-{suffix}" / "SKILL.md"
    if not path.exists():
        return SKILL_PROMPTS[suffix]
    text = path.read_text(encoding="utf-8")
    if text.startswith("---"):
        block = text.split("---", 2)
        if len(block) >= 3:
            for line in block[1].splitlines():
                if line.startswith("description:"):
                    return line.split(":", 1)[1].strip()
    return SKILL_PROMPTS[suffix]


def command_catalog() -> list[dict[str, str]]:
    catalog = []
    for item in CLI_COMMANDS:
        catalog.append({**item, "group": "command", "kind": "cli"})
    for suffix in SKILL_SUFFIXES:
        playbook = suffix in PLAYBOOK_ONLY
        related = SKILL_CLI.get(suffix, "")
        catalog.append(
            {
                "id": f"repobrain-{suffix}",
                "name": f"repobrain-{suffix}",
                "group": "skill",
                "kind": "playbook" if playbook else "wraps-cli",
                "description": _skill_description(suffix),
                "command": "" if playbook else related,
                "note": (
                    f"Skill /repobrain-{suffix} — playbook, not a ./repobrain {suffix} command."
                    if playbook
                    else f"Wraps {related}."
                ),
                "prompt": SKILL_PROMPTS[suffix],
            }
        )
    return catalog


def unused_and_hot_pages(usage: dict[str, Any]) -> tuple[list[tuple[str, int]], list[str]]:
    hot = [(str(path), int(count)) for path, count in (usage.get("hot_pages") or [])]
    seen = {path for path, _count in hot}
    unused: list[str] = []
    if PATHS.corpus.exists():
        for path in sorted(PATHS.corpus.rglob("*.md")):
            rel = path.relative_to(PATHS.corpus).as_posix()
            if not is_wiki_content_page(rel, path.name):
                continue
            if rel not in seen:
                unused.append(rel)
    return hot, unused[:24]


def graph_embed_src(graphify: dict[str, Any]) -> str | None:
    html_info = graphify.get("html") or {}
    path = Path(str(html_info.get("path") or PATHS.graphify / "graph.html"))
    if not path.exists():
        return None
    PATHS.dashboard_dir.mkdir(parents=True, exist_ok=True)
    relative = os.path.relpath(path, PATHS.dashboard_dir)
    return Path(relative).as_posix()


def collect_dashboard_data() -> dict[str, Any]:
    doctor = _load_json(PATHS.doctor_latest)
    evaluation = _latest_eval()
    usage: dict[str, Any] = {}
    try:
        from wiki_usage import compute_stats, load_events

        usage = compute_stats(load_events(30))
    except Exception:  # noqa: BLE001
        usage = {}
    graphify: dict[str, Any] = {}
    try:
        from graphify_adapter import status_data

        graphify = status_data()
    except Exception:  # noqa: BLE001
        graphify = {}
    sources: dict[str, Any] = {}
    try:
        from source_pipeline import status_data as source_status

        sources = source_status()
    except Exception:  # noqa: BLE001
        sources = {}
    categories = evaluation.get("categories") or []
    passed = sum(1 for item in categories if item.get("passed"))
    hot, unused = unused_and_hot_pages(usage)
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
        "commands": command_catalog(),
        "hot_pages": hot,
        "unused_pages": unused,
        "graph_src": graph_embed_src(graphify),
        "graph_open": str((graphify.get("html") or {}).get("path") or PATHS.graphify / "graph.html"),
    }


def _compact_mapping(value: Any) -> str:
    if not isinstance(value, dict) or not value:
        return "none"
    return ", ".join(f"{key}={value[key]}" for key in sorted(value))


def _tone(*parts: str) -> str:
    return " ".join(part for part in parts if part)


def _metric_card(label: str, value: Any, note: str, tone: str = "ok") -> str:
    return (
        f'<article class="card tone-{_escape(tone)}"><h2>{_escape(label)}</h2>'
        f'<p class="value">{_escape(value)}</p>'
        f'<p class="note">{_escape(note)}</p></article>'
    )


def _copy_pair(command: str, prompt: str) -> str:
    parts: list[str] = []
    if command:
        parts.append(f"<pre><code>{_escape(command)}</code></pre>")
        parts.append(
            f'<button type="button" class="copy" data-copy="{_escape(command)}">'
            "Copy command</button> "
        )
    if prompt:
        parts.append(
            f'<button type="button" class="copy" data-copy="{_escape(prompt)}">'
            "Copy agent prompt</button>"
        )
    return "".join(parts)


def _warnings(data: dict[str, Any]) -> list[tuple[str, str, str]]:
    warnings: list[tuple[str, str, str]] = []
    if data.get("doctor_score") is None:
        warnings.append(
            (
                "Doctor report is missing.",
                "./repobrain doctor",
                "Run RepoBrain doctor and fix any critical or high findings.",
            )
        )
    if data.get("eval_status") != "pass":
        warnings.append(
            (
                "Baseline evaluation is missing or failing.",
                "./repobrain eval",
                "Run the RepoBrain baseline evaluation and inspect the latest report.",
            )
        )
    graphify = data.get("graphify") or {}
    artifact = graphify.get("artifact") or {}
    freshness = graphify.get("freshness") or {}
    html_info = graphify.get("html") or {}
    cli = graphify.get("cli") or {}
    state = artifact.get("state")
    if state not in {None, "ready"}:
        warnings.append(
            (
                artifact.get("diagnostic")
                or f"Graphify graph is {state or 'unavailable'}.",
                "./repobrain graph sync --force",
                "Sync the Graphify code graph and re-check status.",
            )
        )
    elif cli.get("compatible") is False:
        warnings.append(
            (
                cli.get("diagnostic") or "Graphify CLI is incompatible.",
                cli.get("install_command") or "python3 -m pip install --user 'graphifyy>=0.9.54,<0.10'",
                "Install the pinned Graphify CLI and re-run graph status.",
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
    elif html_info and not html_info.get("available"):
        warnings.append(
            (
                "Graphify HTML has not been generated.",
                "./repobrain graph export-html",
                "Export Graphify HTML and reopen the Code Graph tab.",
            )
        )
    conversion = (data.get("sources") or {}).get("conversion") or {}
    if int(conversion.get("failed") or 0):
        warnings.append(
            (
                "Source conversion failures are recorded.",
                "./repobrain source convert",
                "Retry local document conversion and inspect the source manifest.",
            )
        )
    if not (data.get("sources") or {}).get("manifest", {}).get("present"):
        warnings.append(
            (
                "Source inventory manifest has not been generated.",
                "./repobrain source scan",
                "Scan Git-tracked project sources without promoting them.",
            )
        )
    return warnings


def _rows(items: list[tuple[str, Any]]) -> str:
    if not items:
        return "<tr><td colspan=\"2\">None</td></tr>"
    return "".join(
        f"<tr><td>{_escape(name)}</td><td>{_escape(value)}</td></tr>"
        for name, value in items
    )


def render_html(data: dict[str, Any]) -> str:
    usage = data.get("usage") or {}
    graphify = data.get("graphify") or {}
    sources = data.get("sources") or {}
    warnings = _warnings(data)
    warning_html = "".join(
        (
            '<article class="panel">'
            f"<p>{_escape(message)}</p>"
            + _copy_pair(command, prompt)
            + "</article>"
        )
        for message, command, prompt in warnings
    ) or '<p class="ok">No remediation warnings.</p>'
    doctor_tone = (
        "missing"
        if data.get("doctor_score") is None
        else "ok"
        if float(data.get("doctor_score") or 0) >= 95
        else "warn"
    )
    eval_tone = "ok" if data.get("eval_status") == "pass" else "warn"
    graph_tone = (
        "ok"
        if (graphify.get("artifact") or {}).get("state") == "ready"
        and (graphify.get("freshness") or {}).get("source") != "stale"
        else "warn"
    )
    source_tone = "warn" if int((sources.get("conversion") or {}).get("failed") or 0) else "ok"
    classifications = sources.get("classifications") or {}
    policy = sources.get("policy") or {}
    failures = sources.get("failures") or []
    failure_html = "".join(
        f"<li><code>{_escape(item.get('path'))}</code> — {_escape(item.get('diagnostic'))}</li>"
        for item in failures
    ) or "<li>No conversion failures.</li>"
    hot_html = _rows(list(data.get("hot_pages") or []))
    unused_html = "".join(
        f"<li><code>{_escape(path)}</code></li>" for path in (data.get("unused_pages") or [])
    ) or "<li>Usage data does not identify unused pages.</li>"
    commands = data.get("commands") or command_catalog()
    cli_items = [item for item in commands if item.get("group", "command") != "skill"]
    skill_items = [item for item in commands if item.get("group") == "skill"]

    def _catalog_articles(items: list[dict[str, str]]) -> str:
        return "".join(
            (
                '<article class="panel command">'
                f"<h3>{_escape(item.get('name'))}</h3>"
                f"<p>{_escape(item.get('description'))}</p>"
                + (
                    f'<p class="meta">{_escape(item.get("note"))}</p>'
                    if item.get("note")
                    else ""
                )
                + _copy_pair(str(item.get("command") or ""), str(item.get("prompt") or ""))
                + "</article>"
            )
            for item in items
        )

    command_html = (
        '<p class="meta">Commands are <code>./repobrain</code> verbs. '
        "Skills are agent playbooks such as /repobrain-query. "
        "Those names are not ./repobrain CLI commands.</p>"
        '<h3 id="dash-commands">Commands</h3>'
        + (_catalog_articles(cli_items) or '<p class="ok">No commands in this snapshot.</p>')
        + '<h3 id="dash-skills">Skills</h3>'
        + (_catalog_articles(skill_items) or '<p class="ok">No skills in this snapshot.</p>')
    )
    graph_src = data.get("graph_src")
    graph_open = data.get("graph_open") or ""
    iframe = (
        f'<iframe id="graph-frame" title="Graphify code graph" src="{_escape(graph_src)}"></iframe>'
        if graph_src
        else ""
    )
    graph_note = (
        "Graphify HTML is missing or stale. Use the command below instead of inventing a second renderer."
        if not graph_src
        else "If the iframe is blank (common for file:// pages), open the full Graphify HTML."
    )
    skill_ids = " ".join(f"repobrain-{suffix}" for suffix in SKILL_SUFFIXES)
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>RepoBrain dashboard</title>
  <style>
    :root {{
      color-scheme: dark light;
      --bg: #0f0f1a;
      --panel: #1a1a2e;
      --line: #2a2a4e;
      --text: #e8e8f0;
      --muted: #9aa0b5;
      --accent: #4e79a7;
      --ok: #3d9a6a;
      --warn: #c9862a;
      --missing: #8a6d8a;
      font-family: "Segoe UI", ui-sans-serif, system-ui, sans-serif;
    }}
    @media (prefers-color-scheme: light) {{
      :root {{
        --bg: #f4f6fb;
        --panel: #ffffff;
        --line: #d7dbe8;
        --text: #1b1f2a;
        --muted: #5c6478;
      }}
    }}
    * {{ box-sizing: border-box; }}
    body {{ margin: 0; background: var(--bg); color: var(--text); }}
    header {{ padding: 1.25rem 1.5rem 0.5rem; border-bottom: 1px solid var(--line); }}
    .brand {{ display: flex; gap: 0.75rem; align-items: baseline; flex-wrap: wrap; }}
    .badge {{ font-size: 0.75rem; border: 1px solid var(--line); border-radius: 999px; padding: 0.15rem 0.55rem; color: var(--muted); }}
    h1 {{ margin: 0.35rem 0; font-size: 1.7rem; }}
    .meta {{ color: var(--muted); margin: 0.2rem 0; }}
    nav {{ display: flex; gap: 0.4rem; flex-wrap: wrap; padding: 0.75rem 1.5rem; }}
    nav button {{ background: var(--panel); color: var(--text); border: 1px solid var(--line); border-radius: 999px; padding: 0.4rem 0.85rem; cursor: pointer; }}
    nav button[aria-selected="true"] {{ border-color: var(--accent); background: color-mix(in srgb, var(--accent) 18%, var(--panel)); }}
    main {{ padding: 1rem 1.5rem 2rem; max-width: 1100px; }}
    .grid {{ display: grid; gap: 0.85rem; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); }}
    .card, .panel, .warn {{ background: var(--panel); border: 1px solid var(--line); border-radius: 14px; padding: 1rem; }}
    .tone-ok {{ border-top: 3px solid var(--ok); }}
    .tone-warn {{ border-top: 3px solid var(--warn); }}
    .tone-missing {{ border-top: 3px solid var(--missing); }}
    .value {{ font-size: 1.75rem; margin: 0.25rem 0; font-variant-numeric: tabular-nums; }}
    .note, .ok {{ color: var(--muted); }}
    table {{ width: 100%; border-collapse: collapse; }}
    td {{ border-top: 1px solid var(--line); padding: 0.4rem 0.2rem; vertical-align: top; }}
    pre {{ overflow: auto; background: var(--bg); padding: 0.6rem; border-radius: 8px; }}
    button.copy {{ margin: 0.25rem 0.25rem 0 0; background: var(--accent); color: #fff; border: 0; border-radius: 8px; padding: 0.4rem 0.7rem; cursor: pointer; }}
    iframe {{ width: 100%; min-height: 70vh; border: 1px solid var(--line); border-radius: 12px; background: #0f0f1a; }}
    .hidden {{ display: none; }}
    .fallback {{ margin-top: 0.75rem; }}
    @media (max-width: 640px) {{
      header, nav, main {{ padding-left: 0.85rem; padding-right: 0.85rem; }}
      .value {{ font-size: 1.35rem; }}
      iframe {{ min-height: 50vh; }}
    }}
  </style>
</head>
<body>
  <header>
    <div class="brand">
      <strong>RepoBrain</strong>
      <span class="badge">local read-only</span>
      <span class="badge">{_escape(data.get("host"))}</span>
    </div>
    <h1>Health and exploration</h1>
    <p class="meta">Generated {_escape(data.get("generated_at"))}. No mutation API, queue, credentials, or command server.</p>
    <p class="meta">Clickable preview: ./repobrain dashboard html --serve then open the printed http://127.0.0.1 URL. Cursor Simple Browser cannot open file:// (/Users/... becomes https://users/... → ERR_NAME_NOT_RESOLVED).</p>
    <p class="meta">Cheat sheet: docs/wiki/_system/docs/CHEATSHEET.md</p>
  </header>
  <nav aria-label="Dashboard views">
    <button type="button" data-tab="overview" aria-selected="true">Overview</button>
    <button type="button" data-tab="sources">Sources</button>
    <button type="button" data-tab="graph">Code graph</button>
    <button type="button" data-tab="commands">Cheat sheet</button>
  </nav>
  <main>
    <section id="tab-overview">
      <div class="grid">
        {_metric_card("Doctor", data.get("doctor_score") if data.get("doctor_score") is not None else "missing", "Structural health from the latest doctor report", doctor_tone)}
        {_metric_card("Evaluation", f"{data.get('eval_passed')}/{data.get('eval_total')} {data.get('eval_status')}", "Pass rate from the latest baseline snapshot", eval_tone)}
        {_metric_card("Retrieval quality", usage.get("usefulness_index") if usage.get("usefulness_index") is not None else "n/a", "Usefulness heuristic from usage telemetry", "ok")}
        {_metric_card("Retrieve cost", usage.get("tokens_total_est") if usage.get("tokens_total_est") is not None else "n/a", "Estimated retrieve tokens in the usage window", "ok")}
        {_metric_card("Usage events", usage.get("events") if usage.get("events") is not None else "n/a", "Telemetry events in the usage window", "ok")}
        {_metric_card("Graphify", ((graphify.get("freshness") or {}).get("source") or (graphify.get("artifact") or {}).get("state") or "missing"), "Adapter freshness, not a second code index", graph_tone)}
        {_metric_card("Sources", (sources.get("manifest") or {}).get("entries") or 0, "Conversion " + _compact_mapping(sources.get("conversion")), source_tone)}
      </div>
      <section class="warn" style="margin-top:1rem">
        <h2>Warnings and remediation</h2>
        {warning_html}
      </section>
    </section>
    <section id="tab-sources" class="hidden">
      <h2>Sources and conversions</h2>
      <p class="meta">Cache remains gitignored. commit_groups={_escape(policy.get("commit_groups") or [])}. formats={_escape(policy.get("formats") or [])}.</p>
      <table>
        <thead><tr><th>Classification</th><th>Count</th></tr></thead>
        <tbody>{_rows(sorted((classifications or {}).items()))}</tbody>
      </table>
      <table>
        <thead><tr><th>Conversion state</th><th>Count</th></tr></thead>
        <tbody>{_rows(sorted((sources.get("conversion") or {}).items()))}</tbody>
      </table>
      <h3>Failed conversions</h3>
      <ul>{failure_html}</ul>
      <h3>Frequently retrieved pages</h3>
      <table><tbody>{hot_html}</tbody></table>
      <h3>Unused compiled pages</h3>
      <ul>{unused_html}</ul>
    </section>
    <section id="tab-graph" class="hidden">
      <h2>Code graph</h2>
      <p class="meta">{_escape(graph_note)}</p>
      <p><a id="open-full-graph" href="{_escape(graph_src or graph_open)}">Open full graph</a></p>
      {iframe}
      <div class="fallback panel" id="graph-fallback">
        <p>Use Graphify's generated HTML. RepoBrain does not draw a second graph from JSON.</p>
        {_copy_pair("./repobrain graph export-html", "Export Graphify HTML and open graphify-out/graph.html.")}
      </div>
    </section>
    <section id="tab-commands" class="hidden" data-skills="{_escape(skill_ids)}">
      <h2>Skills and commands</h2>
      {command_html}
    </section>
  </main>
  <script>
    const tabs = document.querySelectorAll("nav button");
    const show = (id) => {{
      document.querySelectorAll("main > section").forEach((section) => {{
        section.classList.toggle("hidden", section.id !== "tab-" + id);
      }});
      tabs.forEach((button) => {{
        button.setAttribute("aria-selected", button.getAttribute("data-tab") === id ? "true" : "false");
      }});
      location.hash = id;
    }};
    tabs.forEach((button) => button.addEventListener("click", () => show(button.getAttribute("data-tab"))));
    const initial = (location.hash || "#overview").slice(1);
    if (["overview", "sources", "graph", "commands"].includes(initial)) show(initial);
    document.querySelectorAll("button.copy").forEach((button) => {{
      button.addEventListener("click", async () => {{
        const value = button.getAttribute("data-copy") || "";
        if (navigator.clipboard && navigator.clipboard.writeText) {{
          await navigator.clipboard.writeText(value);
        }}
      }});
    }});
    const frame = document.getElementById("graph-frame");
    const fallback = document.getElementById("graph-fallback");
    if (!frame) {{
      fallback.hidden = false;
    }} else {{
      fallback.hidden = true;
      frame.addEventListener("error", () => {{ fallback.hidden = false; }});
      window.setTimeout(() => {{
        try {{
          if (frame.offsetHeight < 40) fallback.hidden = false;
        }} catch (error) {{
          fallback.hidden = false;
        }}
      }}, 1200);
    }}
  </script>
</body>
</html>
"""


def dashboard_file_uri(path: Path) -> str:
    """Return a pasteable file:// URL for the system browser."""
    return path.resolve().as_uri()


def dashboard_http_url(path: Path, port: int, host: str = "127.0.0.1") -> str:
    """Return a clickable localhost URL (Cursor Simple Browser can open http)."""
    relative = path.resolve().relative_to(ROOT.resolve()).as_posix()
    return f"http://{host}:{port}/{relative}"


class _DashboardHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(ROOT), **kwargs)

    def log_message(self, format: str, *args: object) -> None:
        del format, args


def bind_dashboard_server(
    port: int = 0,
) -> tuple[http.server.ThreadingHTTPServer, int]:
    httpd = http.server.ThreadingHTTPServer(("127.0.0.1", port), _DashboardHandler)
    return httpd, int(httpd.server_address[1])


def start_dashboard_server(
    port: int = 0,
) -> tuple[http.server.ThreadingHTTPServer, int]:
    httpd, bound = bind_dashboard_server(port=port)
    thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    thread.start()
    return httpd, bound


def print_dashboard_location(path: Path, http_url: str | None = None) -> None:
    if http_url:
        print(http_url, flush=True)
        print(f"path: {path}", file=sys.stderr, flush=True)
        print(
            "Click the http:// URL — Cursor can open localhost. "
            "Leave this process running until you close the preview.",
            file=sys.stderr,
            flush=True,
        )
        return
    uri = dashboard_file_uri(path)
    print(uri, flush=True)
    print(f"path: {path}", file=sys.stderr, flush=True)
    print(
        "file:// is not clickable in Cursor Simple Browser (it becomes "
        "https://users/... and ERR_NAME_NOT_RESOLVED). "
        "For a clickable preview run: ./repobrain dashboard html --serve",
        file=sys.stderr,
        flush=True,
    )


def write_dashboard(data: dict[str, Any] | None = None) -> Path:
    payload = data or collect_dashboard_data()
    PATHS.dashboard_dir.mkdir(parents=True, exist_ok=True)
    destination = PATHS.dashboard_dir / "index.html"
    destination.write_text(render_html(payload), encoding="utf-8")
    return destination


def cmd_html(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(prog="repobrain dashboard html")
    parser.add_argument(
        "--serve",
        action="store_true",
        help="serve over http://127.0.0.1 so the printed URL is clickable in Cursor",
    )
    parser.add_argument("--port", type=int, default=0, help="bind port (0 = ephemeral)")
    parser.add_argument(
        "--open",
        action="store_true",
        help="open the preview URL with the system webbrowser module",
    )
    args = parser.parse_args([] if argv is None else argv)
    path = write_dashboard()
    if not args.serve and not args.open:
        print_dashboard_location(path)
        return 0
    httpd, port = bind_dashboard_server(port=args.port)
    url = dashboard_http_url(path, port)
    print_dashboard_location(path, http_url=url)
    if args.open:
        webbrowser.open(url)
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        return 0
    finally:
        httpd.shutdown()
        httpd.server_close()
    return 0
