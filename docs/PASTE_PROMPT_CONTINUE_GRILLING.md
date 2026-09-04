# Paste this into the palimpsest-repo agent

Agent: https://cursor.com/agents/bc-01a06caa-c01e-7a32-904d-96a84a736356  
Repo: https://github.com/devintucker24/palimpsest

---

```text
You are taking over Palimpsest on THIS repo. Do not redesign from scratch.

## Step 1 — Import the language if the repo is still empty/thin
Source of truth (clean export):
  https://github.com/devintucker24/trell/tree/handoff/palimpsest-standalone

git clone --depth 1 --branch handoff/palimpsest-standalone \
  https://github.com/devintucker24/trell.git /tmp/palimpsest-src
find . -mindepth 1 -maxdepth 1 ! -name .git -exec rm -rf {} +
find /tmp/palimpsest-src -mindepth 1 -maxdepth 1 ! -name .git -exec cp -a {} ./ \;
cargo test
git add -A
git commit -m "Initial commit: Palimpsest epistemic memory language (+ continuity handoff)"
git push -u origin main

If Trell clone 404s, ask me to grant this workspace access to devintucker24/trell, then retry.

## Step 2 — Read the continuity doc BEFORE coding
Read: docs/CONTINUITY_HANDOFF.md

That doc has:
- what Palimpsest is / is not
- what v0 already implements
- research conclusions (RAG, GBrain, LLM Wiki)
- known gaps and syntax options
- the full grilling interview bank
- interview STATE: Q1 was asked, unanswered; resume there

## Step 3 — Continue the grilling (user rule: ONE question at a time)
Resume with Q1 exactly as in the handoff doc.
For each question: options table + your recommendation + wait.
Do not dump all questions. Do not implement syntax v2 mid-interview.
When all answers are in, freeze a one-page Language Charter and confirm it with me.

Original Trell PR for context only: https://github.com/devintucker24/trell/pull/5
```
