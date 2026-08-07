# AirBattery Governance Roadmap Reconstruction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the AirBattery governance recovery gate using the approved reconstructed reference-app matrix, update the authoritative roadmap, create a new immutable snapshot, and identify Phase 5 as the first incomplete product phase without changing product code.

**Architecture:** Documentation-only governance transition. The approved matrix design remains the decision source; the Master Roadmap becomes the ordered source of truth; START_HERE points to the latest immutable snapshot. Phase 5 implementation stays blocked until its own Design and RED -> GREEN plan are approved.

**Tech Stack:** Markdown, Git, Python 3 validation.

## Global Constraints

- Do not modify product source code.
- Do not rewrite existing progress snapshots.
- Keep `Overall project: Unscored`.
- Record the historical matrix as not recoverable from available artifacts.
- Do not claim the reconstructed matrix is the original historical matrix.
- Canonical matrix design: `docs/superpowers/specs/2026-08-07-airbattery-reference-app-matrix-design.md`.
- Never synthesize Aggregate from Left/Right and present it as measured data.
- Left/Right/Case only when actually reported.
- Preferred, Pinned, Active, Nearby, and Known remain distinct.
- Hardware observations do not choose roadmap phases.
- No destructive Git operations.
- No Phase 5 implementation before its own approved Design and Plan.

---

### Task 1 — Update Master Roadmap

**Modify:** `docs/project/AIRBATTERY_MASTER_ROADMAP.md`

- [ ] Verify the old matrix-recovery gate is still present.
- [ ] Replace it with a `Controlled Reconstruction Result`.
- [ ] Record that repository history, reflog, backups, local archives, shell history, and the governance generator were checked and the historical matrix was not recovered.
- [ ] Record Adopt / Adapt / Reject / Defer as the decision model.
- [ ] Add the ordered phases exactly:

```text
Phase 0 — Foundation & Platform Architecture — Completed baseline
Phase 1 — Universal Truthful Battery Runtime — Completed baseline
Phase 2 — Hybrid Native Runtime — Completed
Phase 3 — Brand, Artwork & Live Status — Completed
Phase 4 — Liquid Glass Desktop Experience — Completed
Phase 5 — Battery Alerts & Notification Policy — Next / Design required
Phase 6 — Quick Device Actions — Queued
Phase 7 — Battery History & Insights — Queued
Phase 8 — Proximity Experience — Queued
Phase 9 — Advanced Headset Controls — Deferred
```

- [ ] Map completed milestones:
  - Linux foundation -> Phase 0
  - GNOME integration -> Phase 0
  - Release engineering -> Phase 0
  - Universal battery runtime -> Phase 1
  - Hybrid native architecture -> Phase 2
  - Logo and live status colors -> Phase 3
  - Liquid Glass desktop UI -> Phase 4
  - Durable roadmap/handoff -> Governance completed
- [ ] Mark Project governance `Completed`.
- [ ] Keep Overall project `Unscored`.
- [ ] Set Exact Next Action to designing Phase 5, not implementing it.

Validation:

```bash
git diff --check
if grep -q 'ماتریس قبلاً تأییدشده اپ‌های موفق' docs/project/AIRBATTERY_MASTER_ROADMAP.md; then
  echo "FAIL: obsolete recovery gate still present"
  exit 1
fi
```

---

### Task 2 — Create immutable Governance completion snapshot

**Create:** `docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md`

The snapshot must record:

- historical matrix recovery was exhausted;
- historical matrix was not recoverable;
- governance-generator origin of the old assumption was identified;
- controlled reference-app matrix was approved;
- Adopt / Adapt / Reject / Defer rules were formalized;
- completed milestones were mapped to ordered phases;
- Phase 5 is first incomplete product phase;
- Overall remains `Unscored`;
- no Phase 5 implementation begins before its Design and Plan.

Exact Next Action:

```text
Design Phase 5 — Battery Alerts & Notification Policy
```

The Phase 5 Design must define:

- real notification data sources;
- capability gating;
- freshness/stale suppression;
- per-device thresholds;
- charging evidence;
- deduplication/cooldowns;
- Linux/Windows behavior;
- persistence;
- success tests.

---

### Task 3 — Update START_HERE

**Modify:** `docs/project/START_HERE.md`

- [ ] Point explicitly to:
  `docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md`
- [ ] Record:
  - Latest completed product phase: Phase 4
  - Governance reconstruction: Completed
  - First incomplete product phase: Phase 5
  - Overall project: Unscored
  - Exact Next Action: create and approve the Phase 5 Design
- [ ] Preserve startup order: Roadmap -> latest Snapshot -> active Design/Plan -> Git status/log.
- [ ] Add guard: if next phase has no approved Design/Plan, do not implement it.

---

### Task 4 — Documentation consistency checks

Run:

```bash
git diff --check

python3 <<'PY'
from pathlib import Path
root = Path(".")
roadmap = (root / "docs/project/AIRBATTERY_MASTER_ROADMAP.md").read_text()
start = (root / "docs/project/START_HERE.md").read_text()
snapshot = (root / "docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md").read_text()
design = (root / "docs/superpowers/specs/2026-08-07-airbattery-reference-app-matrix-design.md").read_text()

for marker in ("Phase 5", "Battery Alerts"):
    assert marker in roadmap
    assert marker in start
    assert marker in snapshot
    assert marker in design

for text in (roadmap, start, snapshot):
    assert "Unscored" in text

assert "Synthetic" in design and "Aggregate" in design
assert "2026-08-07-01-governance-matrix-reconstruction-completed.md" in start
assert "ماتریس قبلاً تأییدشده اپ‌های موفق" not in roadmap

for text in (roadmap, start, snapshot):
    for bad in ("TODO", "TBD", "FIXME", "PLACEHOLDER"):
        assert bad not in text

print("Governance documentation invariants: PASS")
PY
```

Expected:

```text
Governance documentation invariants: PASS
```

Only these governance outputs may change:

```text
docs/project/AIRBATTERY_MASTER_ROADMAP.md
docs/project/START_HERE.md
docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md
```

---

### Task 5 — Commit completed Governance reconstruction

Stage only the three governance outputs, validate `git diff --cached --check`, then commit:

```bash
git commit -m "docs: complete roadmap matrix reconstruction"
```

Final verification:

```bash
git status -sb
git log -4 --oneline --decorate
```

## Completion Gate

Complete only when:

- historical matrix is recorded as unrecoverable;
- reconstructed matrix is authoritative;
- Adopt / Adapt / Reject / Defer is part of governance;
- completed milestones are mapped to phases;
- Governance is completed;
- Phase 5 is first incomplete phase;
- Phase 5 implementation remains blocked pending its Design and Plan;
- Overall project remains `Unscored`;
- latest immutable Snapshot is referenced by START_HERE;
- validation passes;
- no product code changed.

**Next Exact Action:** `Design Phase 5 — Battery Alerts & Notification Policy`
