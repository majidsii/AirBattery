# AirBattery Reference-App Capability Matrix — Design

**Date:** 2026-08-07  
**Status:** Approved design direction  
**Type:** Controlled roadmap reconstruction  
**Project:** AirBattery

---

## 1. Purpose

This document defines how AirBattery reconstructs its reference-app capability
matrix after the previously mentioned historical matrix could not be recovered
from the available repository history, reflog, backups, local archives, shell
history, or governance-generation artifacts.

This is a **controlled reconstruction**, not a claim that the original matrix
was recovered.

The goal is to establish a durable, source-backed capability matrix that can be
used to:

1. distinguish completed work from future work;
2. prevent observations and bugs from hijacking roadmap priority;
3. evaluate reference-app features consistently;
4. record Adopt / Adapt / Reject / Defer decisions;
5. map existing AirBattery milestones onto one ordered roadmap;
6. identify the first genuinely incomplete product phase;
7. make future project-progress scoring possible.

---

## 2. Existing AirBattery principles are authoritative

The reconstruction does not replace decisions already established by AirBattery
designs, plans, ADRs, tests, and completed phases.

The following remain non-negotiable:

- AirBattery is universal rather than AirPods-specific.
- Architecture remains capability-based and provider-based.
- Platform and protocol logic remain isolated behind providers/adapters.
- Linux and Windows remain the primary product platforms.
- Every Active device is visible on the main dashboard.
- Preferred device affects tray/widget/ordering behavior, not whether another
  Active device exists.
- Aggregate battery data must not be converted into fabricated Left/Right/Case
  components.
- Left/Right/Case are displayed only when a real data source reports them.
- Missing, unavailable, stale, and disconnected are distinct from `0%`.
- Charging state is shown only when supported by evidence.
- Exact artwork is used only when model identity is reliable.
- A generic fallback is used when exact model identity is unavailable.
- CI and relevant automated validation must be green before merge.
- Device observations are evidence, not automatic roadmap phases.

---

## 3. Recovery result

The Governance branch originally stated that a previously approved
reference-app matrix should be recovered.

A later recovery investigation checked:

- current repository files;
- all reachable Git commits;
- Git reflog;
- old AirBattery workspaces;
- old AirBattery source backups;
- Downloads/Documents/Desktop;
- Markdown, text, JSON, HTML and log artifacts;
- shell history;
- the governance document generator itself.

No recoverable reference-app matrix containing the expected comparison or
Adopt/Adapt/Reject/Defer decisions was found.

The governance generator itself contained the phrase referring to the
"previously approved matrix"; therefore that phrase is not evidence that a
recoverable matrix artifact exists.

The project will therefore proceed using this controlled reconstruction.

---

## 4. Reference-app tiers

### 4.1 Primary commercial references

#### AirBuddy

Role:

- primary UX reference;
- nearby-device presentation;
- battery alerts;
- device pinning;
- quick actions and automation;
- local device statistics.

Important constraints observed in AirBuddy:

- third-party device battery support depends on what macOS actually reports;
- charged alerts are not available for every device because some devices stop
  reporting battery information while charging.

Sources:

- https://v2.airbuddy.app/
- https://support.airbuddy.app/articles/how-to-configure-battery-alerts-for-devices-in-airbuddy
- https://support.airbuddy.app/articles/how-to-keep-a-device-always-visible-in-airbuddy/
- https://support.airbuddy.app/articles/how-can-i-see-stats-about-my-devices-in-airbuddy/
- https://support.airbuddy.app/articles/how-to-use-the-shortcuts-actions-provided-by-airbuddy/
- https://support.airbuddy.app/articles/will-airbuddy-support-my-non-apple-mouse-keyboard-or-headset/

#### MagicPods

Role:

- primary Windows UX reference;
- Windows tray behavior;
- Windows 11 battery widget;
- per-headset configuration;
- low-battery notifications;
- connection hotkeys;
- headset-specific interaction reference.

Sources:

- https://help.magicpods.app/
- https://help.magicpods.app/fun-widgets/
- https://help.magicpods.app/fun-tray-icon/
- https://help.magicpods.app/changelog/

#### ToothFairy

Role:

- quick-device-action reference;
- one-click connect/disconnect;
- per-device hotkeys;
- menu-bar battery visibility;
- automation hooks.

Sources:

- https://c-command.com/toothfairy/
- https://c-command.com/toothfairy/manual
- https://c-command.com/toothfairy/help/show-battery-status-in
- https://c-command.com/toothfairy/help/global-hotkey-to-connec

#### Batteries for Mac

Role:

- focused multi-device battery-monitoring reference;
- menu-bar overview;
- low-battery notifications;
- simple battery-first product scope.

Important limitation:

- its Bluetooth support is bounded by battery information available through the
  platform Bluetooth stack.

Source:

- https://www.fadel.io/batteries

---

### 4.2 Technical / interoperability references

#### LibrePods

Role:

- Linux interoperability research;
- Apple proprietary protocol feasibility reference;
- battery status;
- ear detection;
- listening modes;
- automatic connection behavior.

License rule:

LibrePods is GPL-3.0. AirBattery is not permitted to copy GPL implementation
code into its MIT codebase. Protocol facts may be independently researched and
clean-room reimplemented.

Source:

- https://github.com/librepods-org/librepods

#### CAPod

Role:

- proximity UX reference;
- case-open popup;
- nearby-device display;
- widgets;
- split battery/charging presentation;
- ear detection.

Source:

- https://github.com/d4rken-org/capod

---

### 4.3 Adjacent reference

#### BatFi

BatFi is not a direct AirBattery competitor.

Its product is MacBook battery charge control and automation, including charge
limits and host-battery management.

It is retained only as an adjacent reference for:

- menu-bar utility ergonomics;
- threshold configuration ideas;
- local automation UX.

MacBook charging control itself is outside AirBattery scope.

Source:

- https://micropixels.software/apps/batfi

---

## 5. Decision vocabulary

### Adopt

The capability directly fits AirBattery's product purpose and architectural
principles.

### Adapt

The underlying user need is valuable, but the reference implementation or
behavior must be modified to preserve AirBattery's truth-first, universal,
cross-platform architecture.

### Reject

The capability conflicts with AirBattery's purpose, creates misleading battery
semantics, or causes unacceptable scope expansion.

### Defer

The capability may be valuable later but is not required for the next product
milestones.

---

## 6. Capability matrix

| # | Capability | Main references | Decision | AirBattery status | Target |
|---|---|---|---|---|---|
| 1 | Real Aggregate / Left / Right / Case presentation | AirBattery baseline, AirBuddy, CAPod | Adopt | Implemented baseline | Completed |
| 2 | Never fabricate unavailable battery components | AirBattery baseline | Adopt | Implemented | Completed |
| 3 | Universal multi-device dashboard | AirBattery baseline, AirBuddy, MagicPods | Adopt | Implemented | Completed |
| 4 | Tray / menu-bar battery surface | AirBuddy, MagicPods, ToothFairy, Batteries | Adopt | Implemented baseline | Completed |
| 5 | Desktop / OS battery widget | MagicPods, CAPod | Adopt | Implemented baseline | Completed |
| 6 | Low-battery notification | AirBuddy, MagicPods, Batteries | Adopt | Planned | Phase 5 |
| 7 | Per-device low-battery threshold | AirBuddy, MagicPods | Adopt | Planned | Phase 5 |
| 8 | Fully-charged notification | AirBuddy | Adapt | Planned | Phase 5 |
| 9 | Freshness-aware notification suppression | AirBattery truth model | Adopt | Planned | Phase 5 |
| 10 | Notification deduplication / cooldown | AirBattery reliability requirement | Adopt | Planned | Phase 5 |
| 11 | Pin device | AirBuddy | Adapt | Planned | Phase 6 |
| 12 | Hide device | AirBattery baseline | Adopt | Implemented | Completed |
| 13 | Preferred device for tray/widget/ordering | AirBattery baseline | Adopt | Implemented | Completed |
| 14 | One-click connect/disconnect | AirBuddy, ToothFairy | Adopt | Planned | Phase 6 |
| 15 | Per-device connect/disconnect hotkey | AirBuddy, MagicPods, ToothFairy | Adopt | Planned | Phase 6 |
| 16 | Battery history | AirBuddy | Adapt | Planned | Phase 7 |
| 17 | Battery drain trend | AirBuddy | Adapt | Planned | Phase 7 |
| 18 | Listening/call usage statistics | AirBuddy | Reject for battery core | Not planned | — |
| 19 | Case-open / proximity popup | AirBuddy, CAPod | Adapt | Planned | Phase 8 |
| 20 | Nearby but not connected state | AirBuddy, CAPod | Adapt | Planned | Phase 8 |
| 21 | Last-known intermittent case data with stale state | AirBattery baseline | Adopt | Implemented | Completed |
| 22 | Ear detection / automatic play-pause | MagicPods, LibrePods, CAPod | Defer | Deferred | Phase 9+ |
| 23 | ANC / Transparency / listening modes | AirBuddy, LibrePods | Defer | Deferred | Phase 9+ |
| 24 | Automatic nearest-headset connection | AirBuddy, LibrePods, CAPod | Defer | Deferred | Phase 9+ |
| 25 | User automation / scripting API | AirBuddy, ToothFairy | Defer | Deferred | Post-core |
| 26 | Firmware-version / firmware-update awareness | AirBuddy | Defer | Deferred | Post-core |
| 27 | Phone/tablet/watch battery over local network | AirBuddy, Batteries | Defer | Deferred | Post-core |
| 28 | Audio streaming phone -> PC | MagicPods | Reject | Rejected | — |
| 29 | Codec forcing / audio-quality management | ToothFairy | Reject | Rejected | — |
| 30 | Synthetic average battery presented as real Aggregate | MagicPods tray option | Reject | Prohibited | — |
| 31 | MacBook charge limiting / host charge control | BatFi | Reject | Rejected | — |
| 32 | Cross-computer peripheral handoff | AirBuddy | Reject for current scope | Rejected | — |
| 33 | App-specific spoken notification narrator | MagicPods | Reject | Rejected | — |

---

## 7. Critical semantic decisions

### 7.1 Aggregate battery

MagicPods can display an average of left/right earbud battery values in its tray
when configured to use a single battery indicator.

AirBattery must **not** represent such a calculation as a real Aggregate battery
measurement.

Example:

- Left = 25%
- Right = 45%

AirBattery must not claim:

- Aggregate = 35%

unless a real provider or operating-system source reports an actual Aggregate
value.

A derived value may only exist internally for a narrowly defined algorithm such
as notification prioritization, and must never be exposed to the user as a
measured battery component.

---

### 7.2 Charged notifications

A "fully charged" alert is capability-gated.

It may be emitted only if the provider can observe enough reliable charging
state and battery progression to establish the event truthfully.

If a device becomes unavailable while charging, AirBattery must not infer that
charging completed.

Fallback:

- suppress the charged notification;
- retain the last-known battery information according to freshness rules;
- never invent completion.

---

### 7.3 Active, Nearby, Known, Preferred and Pinned are different concepts

These states must not be collapsed.

**Active**

A device is currently connected, connecting, or actively providing data under
the project's Active-device rules.

**Nearby**

AirBattery has recent provider evidence that the device is physically nearby,
but it may not be connected.

**Known**

The device exists in the user's catalog but does not currently qualify as
Active.

**Preferred**

The user-selected device that influences tray/widget/ordering behavior.

**Pinned**

A user preference requesting persistent visibility or convenient access.

Pinned does not imply Active.

Nearby does not imply Active.

Preferred does not hide other Active devices.

---

## 8. Proposed roadmap mapping

### Phase 0 — Foundation & Platform Architecture

**Status:** Completed baseline

Includes:

- project foundation;
- Rust workspace;
- platform adapter boundaries;
- Linux foundation;
- Windows foundation;
- CI/release foundations.

---

### Phase 1 — Universal Truthful Battery Runtime

**Status:** Completed baseline

Includes:

- capability model;
- provider resolution;
- Generic battery provider;
- AirPods provider;
- Aggregate/Split semantics;
- freshness;
- stale handling;
- multi-device normalization;
- no fabricated battery components.

---

### Phase 2 — Hybrid Native Runtime

**Status:** Completed

Includes the completed Hybrid Native Architecture milestone.

---

### Phase 3 — Brand, Artwork & Live Status

**Status:** Completed

Includes:

- approved AirBattery brand;
- reviewed artwork policy;
- live battery/connection status colors.

---

### Phase 4 — Liquid Glass Desktop Experience

**Status:** Completed

Includes:

- Liquid Glass desktop UI;
- shared glass controls;
- Active/Known device presentation;
- final device cards and visual state behavior.

---

### Phase 5 — Battery Alerts & Notification Policy

**Status:** First proposed incomplete product phase

Scope:

- low-battery alerts;
- per-device thresholds;
- capability-gated charged alerts;
- stale/unavailable suppression;
- duplicate-event suppression;
- notification cooldown;
- Linux/Windows behavior contract.

Out of scope:

- proximity popup;
- connect/disconnect controls;
- battery history;
- headset control features.

Exit criteria:

1. alert decisions are based only on fresh supported data;
2. unavailable/stale data cannot generate false battery alerts;
3. charged alerts require real charging evidence;
4. repeated observations cannot spam duplicate notifications;
5. per-device settings persist;
6. Linux and Windows contracts are testable independently;
7. automated tests cover threshold boundaries and state transitions.

---

### Phase 6 — Quick Device Actions

**Status:** Queued

Scope:

- one-click connect;
- one-click disconnect;
- per-device keyboard shortcuts;
- pinning;
- safe handling for input devices and devices that should not be aggressively
  disconnected.

---

### Phase 7 — Battery History & Insights

**Status:** Queued

Scope:

- local battery history;
- battery drain trends;
- freshness-aware samples;
- local-only persistence.

Explicitly excluded:

- call history;
- listening-time surveillance;
- fabricated/interpolated measurements presented as real observations.

---

### Phase 8 — Proximity Experience

**Status:** Queued

Scope:

- nearby-device state;
- case-open/proximity popup;
- provider capability gating;
- no promotion of Nearby devices to Active without evidence.

---

### Phase 9 — Advanced Headset Controls

**Status:** Deferred

Possible future scope:

- ear detection;
- play/pause;
- ANC/Transparency/listening modes;
- provider-specific automatic connection behavior.

These capabilities must remain isolated from the battery core.

---

## 9. Decision framework for all future capabilities

Every proposed feature must answer all of the following before development:

1. Which user need or reference app introduced it?
2. Is the decision Adopt, Adapt, Reject, or Defer?
3. What user value does it provide?
4. What is the real data or control source?
5. What happens when that source is unavailable?
6. Which platforms support it?
7. Which provider capability gates it?
8. What are the measurable success tests?
9. Which roadmap phase owns it?

If these questions cannot be answered, the feature does not enter development.

---

## 10. Source-quality policy

Reference research should prefer, in order:

1. official product documentation;
2. official help centers;
3. official open-source repositories;
4. independently verified platform documentation;
5. secondary articles only when primary material is unavailable.

Marketing screenshots alone are not sufficient evidence for protocol or
data-source claims.

Open-source implementations may establish feasibility and protocol facts, but
license compatibility must be reviewed before implementation code is reused.

---

## 11. Relationship to observations

Hardware observations remain separate from roadmap priority.

Current examples:

- one tested AirPods device exposed Aggregate only;
- QCY T13 exposed Aggregate only;
- another tested AirPods path exposed Left/Right/Case.

These observations help validate provider and fallback behavior.

They do not automatically create a new development phase.

---

## 12. Project-progress scoring

Overall project progress remains `Unscored` until:

1. this reconstructed matrix is accepted;
2. completed milestones are mapped against it;
3. queued/deferred scope is explicitly recorded;
4. the Master Roadmap is updated.

Only then may the project calculate a meaningful overall completion percentage.

---

## 13. Design outcome

The reconstruction establishes the following proposed order:

1. Foundation & Platform Architecture — Completed baseline
2. Universal Truthful Battery Runtime — Completed baseline
3. Hybrid Native Runtime — Completed
4. Brand, Artwork & Live Status — Completed
5. Liquid Glass Desktop Experience — Completed
6. Battery Alerts & Notification Policy — first incomplete target
7. Quick Device Actions — queued
8. Battery History & Insights — queued
9. Proximity Experience — queued
10. Advanced Headset Controls — deferred

No product feature implementation should begin merely because a new hardware
observation or bug was discovered.

The next product implementation phase is selected only after this reconstructed
design is accepted and mapped into the Master Roadmap.
