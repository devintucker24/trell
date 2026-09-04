---
id: critical-infrastructure-and-energy
title: Critical Infrastructure & Energy
type: application
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- grid
- water
- rail
- infrastructure
domain: applications
summary: Smart grid frequency, water dosing, high-speed rail interlocking.
nodes:
- id: app-smart-grid
  kind: application
- id: app-water-treatment
  kind: application
- id: app-highspeed-rail
  kind: application
edges:
- from: three-beat-safety-pattern
  to: app-smart-grid
  rel: applies_to
- from: app-smart-grid
  to: reg-eu-ai-act
  rel: regulated_by
related:
- '[[applications/overview-and-safety-patterns]]'
- '[[applications/security-cloud-and-governance]]'
agent:
  priority: high
  read_when:
  - energy
  - utilities
  - rail
  maintain: []
---

# Applications: Critical Infrastructure & Energy

Civil infrastructure requires continuous 24/7 uptime where milliseconds of instability cause regional blackouts or tainted public water supplies.

---

## 15. Regional Electrical Smart Grid Frequency Regulation

* **Context:** Synchronous grid balancing at 60.0 Hz (or 50.0 Hz in Europe) balancing intermittent renewable input.
* **Failure Mode:** AI dispatch predicts wind gust incorrectly, throttling gas peakers and dropping grid frequency below 59.5 Hz, triggering cascade blackouts.
* **Trell Implementation:**

```trell
guard InertiaFrequencySafe(mw_delta: int):
    mw_delta >= -200 and mw_delta <= 200 // Max ramp rate per cycle
end

action balance_transmission_line:
    let grid_proposal = ask GridStabilityAI("Current frequency 59.98Hz, solar irradiance declining 12%")
    let verified_dispatch: certain int = require grid_proposal with InertiaFrequencySafe else 0
    adjust_pumped_hydro_turbines(verified_dispatch)
end
```

---

## 16. Municipal Water Treatment Chemical Dosing

* **Context:** Automated water purification facilities treating 500 million gallons daily for metropolitan areas.
* **Failure Mode:** Chemical calculation model hallucinates decimal position, pumping toxic chlorine levels into drinking mains.
* **Trell Implementation:**

```trell
guard ChlorinePartsPerMillionSafe(ppm: float):
    ppm >= 0.5 and ppm <= 4.0 // EPA absolute limits
end

action regulate_disinfection_pumps:
    let dose_recommendation = ask WaterQualityModel("Current coliform count and influent pH")
    let approved_ppm: certain float = require dose_recommendation with ChlorinePartsPerMillionSafe else 1.5
    chemical_injection_actuator.set_ppm(approved_ppm)
end
```

---

## 17. High-Speed Bullet Train Interlocking & Braking

* **Context:** 350 km/h train control system running with 3-minute headways.
* **Failure Mode:** Wayside camera model misclassifies a plastic bag as a buckled rail, initiating unnecessary emergency stop that derails trailing freight.
* **Trell Implementation:**

```trell
guard ValidRailClearance(signal: string):
    signal == "ClearGreen" or signal == "CautionYellow" or signal == "StopRed"
end

action wayside_signaling_loop:
    let optical_verdict = quorum(3, 0.85):
        ask TrackVisionAI("Optical and lidar scan of Track 4 switch interlocking")
    end
    let verified_signal: certain string = require optical_verdict with ValidRailClearance else "StopRed"
    pneumatic_cab_signaling.set_aspect(verified_signal)
end
```

---

## Cross-References
* Security & cloud applications: [[applications/security-cloud-and-governance]]
* Universal safety patterns: [[applications/overview-and-safety-patterns]]
* Robotics applications: [[applications/autonomous-physical-systems]]
