# Questions

The pages in this brain state what is known. This page asks things of it.
Everything below runs against the beliefs gathered from every other page,
because a brain is loaded as a whole.

```pal
trust legal above compliance above policy above staff above user above rumor
```

## Where does Alice live?

Two answers were recorded five months apart. The later one wins, and the
earlier one is still on the record.

```pal
what is alice.city
expect what is alice.city is "Berlin"

what was alice.city on 2026-04-01
expect what was alice.city on 2026-04-01 is "Lisbon"
```

## How much leave does Alice have?

The handbook says twenty. Alice said twenty-five, more recently. The handbook
is filed as `policy` and Alice's page as `user`, so the handbook wins — and
the disagreement is reported rather than silently resolved.

```pal
what is acme.pto.days
expect what is acme.pto.days is 20

conflicts
```

## What did the handbook actually say, and when?

```pal
why acme.pto.days
```

## Retracting a page

Withdrawing a source withdraws everything that page taught. Here the whole
incident report is pulled, and the beliefs that came from it go with it.

```pal
what is billing.db.pool_size

forget everything from pagerduty_incident_4471

why billing.db.pool_size
```

## Is this brain healthy?

```pal
check
```
