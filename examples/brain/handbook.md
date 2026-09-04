---
source: hr_handbook_2026
authority: policy
---

# Employee Handbook 2026

Ratified by the board on 1 January 2026. This page is the authoritative
statement of leave and expense policy for all staff.

## Paid time off

Full-time employees accrue twenty days of paid leave per calendar year.
Unused days do not carry over.

```pal
acme.pto.days is 20 as policy on 2026-01-01
acme.pto.carryover is false as policy on 2026-01-01
```

## Expenses

The per-diem rate is reviewed annually and expires at the end of the year it
was set in, so a stale rate reports itself instead of quietly underpaying
somebody.

```pal
acme.expenses.per_diem_eur is 75 as policy on 2026-01-01 for 1 year
```

## Notes

The handbook is the source of record. Statements elsewhere in the brain that
disagree with this page are expected to lose, and the `check` pass will list
them.
