---
id: raw-sources-{{ group_id }}
title: "{{ title }}"
type: raw-pointer
status: active
created: '{{ date }}'
updated: '{{ date }}'
tags: [raw, source-inventory, {{ group_id }}]
domain: meta
summary: "Managed non-authoritative pointers to {{ title_lower }}."
origin: source-manifest
managed_by: repobrain-source-pipeline
nodes: []
edges: []
related:
  - "[[INDEX]]"
agent:
  priority: low
  read_when:
    - consulting original repository sources
  maintain:
    - do not promote these paths into compiled claims
---

# {{ title }}

These are inventory pointers to original repository sources. They are raw,
non-authoritative material and are not compiled semantic claims.

{{ entries }}
