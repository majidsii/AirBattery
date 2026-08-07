# AirBattery Master Roadmap

## هدف محصول

ساخت یک اپ حرفه‌ای، دقیق و Cross-platform برای نمایش و مدیریت باتری دستگاه‌های بلوتوثی، با استفاده از بهترین قابلیت‌های اپ‌های موفق این حوزه، بدون کپی کورکورانه و بدون جعل داده.

## اصول معماری

- پشتیبانی Capability-based و مستقل از برند
- Provider جدا برای هر پروتکل یا خانواده دستگاه
- Linux و Windows به‌عنوان پلتفرم‌های اصلی
- داشبورد خودکار برای نمایش همه دستگاه‌های Active
- Preferred device فقط برای Widget/Tray/Ordering
- عدم ساخت داده جعلی برای Left/Right/Case
- Aggregate واقعی فقط از منبع واقعی داده پذیرفته می‌شود
- Synthetic Aggregate از Left/Right به‌عنوان مقدار واقعی ممنوع است
- استفاده از Artwork دقیق فقط بعد از تشخیص مدل و Review
- CI سبز قبل از Merge

## وضعیت‌ها

- `Completed`
- `Active`
- `Queued`
- `Blocked`
- `Observed`
- `Deferred`
- `Superseded`

## Controlled Reconstruction Result

عبارت قبلی Roadmap درباره «بازیابی ماتریس قبلاً تأییدشده» بعد از یک Recovery کامل بررسی شد.

منابع بررسی‌شده شامل فایل‌های فعلی Repository، همه Commitهای قابل‌دسترسی Git، Git reflog، Backupها و Workspaceهای قدیمی AirBattery، Downloads / Documents / Desktop، Designها و Planهای قدیمی، Shell history و اسکریپت سازنده Governance بودند.

هیچ Artifact قابل‌استناد از ماتریس تاریخی اپ‌های مرجع و تصمیم‌های `Adopt / Adapt / Reject / Defer` بازیابی نشد.

بررسی `create_airbattery_governance.py` نشان داد عبارت مربوط به «ماتریس قبلاً تأییدشده» مستقیماً در Generator اولیه Governance نوشته شده بود. بنابراین وجود آن عبارت به‌تنهایی مدرک وجود یک Artifact تاریخی قابل‌بازیابی نیست.

از این نقطه، منبع رسمی تصمیم‌ها **Controlled Reconstruction** زیر است:

`docs/superpowers/specs/2026-08-07-airbattery-reference-app-matrix-design.md`

این سند به‌عنوان بازسازی کنترل‌شده پذیرفته شده و نباید به‌عنوان «ماتریس تاریخی بازیابی‌شده» معرفی شود.

## مدل تصمیم قابلیت‌ها

### Adopt
قابلیت مستقیماً با هدف و معماری AirBattery سازگار است.

### Adapt
نیاز کاربری مفید است ولی رفتار یا پیاده‌سازی مرجع باید برای حفظ اصول Truth-first، Universal و Cross-platform تغییر کند.

### Reject
قابلیت با هدف محصول یا Semantics واقعی داده تضاد دارد یا Scope نامناسب ایجاد می‌کند.

### Defer
قابلیت ممکن است بعداً مفید باشد اما برای Milestoneهای بعدی Core الزامی نیست.

## Milestoneهای تأییدشده و Mapping

| Milestone | وضعیت | Phase |
|---|---|---|
| Linux foundation | Completed baseline | Phase 0 |
| GNOME integration | Completed baseline | Phase 0 |
| Release engineering | Completed baseline | Phase 0 |
| Universal battery runtime | Completed baseline | Phase 1 |
| Hybrid native architecture | Completed | Phase 2 |
| Logo and live status colors | Completed | Phase 3 |
| Liquid Glass desktop UI | Completed in PR #8 | Phase 4 |
| Durable roadmap and handoff system | Completed | Governance |

## ترتیب رسمی فازهای محصول

### Phase 0 — Foundation & Platform Architecture
**Status:** `Completed baseline`

### Phase 1 — Universal Truthful Battery Runtime
**Status:** `Completed baseline`

### Phase 2 — Hybrid Native Runtime
**Status:** `Completed`

### Phase 3 — Brand, Artwork & Live Status
**Status:** `Completed`

### Phase 4 — Liquid Glass Desktop Experience
**Status:** `Completed`

### Phase 5 — Battery Alerts & Notification Policy
**Status:** `Next / Design required`

اولین فاز ناقص واقعی محصول.

Scope پیشنهادی بر اساس Matrix مصوب:
- Low-battery alerts
- Per-device thresholds
- Capability-gated fully-charged alerts
- Freshness/stale-aware suppression
- Duplicate-event suppression
- Notification cooldown
- Linux / Windows behavior contract
- Persistence
- Success tests

**Implementation is NOT authorized yet.**

Exact Next Action:
1. Design رسمی Phase 5 نوشته شود.
2. Design توسط کاربر تأیید شود.
3. Implementation Plan با RED → GREEN نوشته شود.
4. فقط بعد از آن پیاده‌سازی شروع شود.

### Phase 6 — Quick Device Actions
**Status:** `Queued`

### Phase 7 — Battery History & Insights
**Status:** `Queued`

### Phase 8 — Proximity Experience
**Status:** `Queued`

### Phase 9 — Advanced Headset Controls
**Status:** `Deferred`

## قوانین Semantics قطعی

### Aggregate
AirBattery نباید میانگین Left/Right را به‌عنوان Aggregate واقعی نمایش دهد.

مثال:
- Left = 25%
- Right = 45%

تا وقتی Provider یا OS مقدار Aggregate واقعی گزارش نکرده، `Aggregate = 35%` نباید به‌عنوان Measurement واقعی در UI ظاهر شود.

### Component Data
Left/Right/Case فقط وقتی نمایش داده می‌شوند که منبع واقعی داده آن‌ها را گزارش کند.

### Charging
Fully-charged notification فقط وقتی مجاز است که evidence کافی درباره charging state و progression وجود داشته باشد. Unavailable شدن دستگاه هنگام شارژ به معنی تکمیل شارژ نیست.

### Device States
این مفاهیم مستقل هستند: Active، Nearby، Known، Preferred و Pinned.

Pinned به معنی Active نیست. Nearby به معنی Active نیست. Preferred باعث مخفی‌شدن دیگر دستگاه‌های Active نمی‌شود.

## Observations

- یک AirPods دیگر فقط Aggregate Battery داد.
- QCY T13 فقط Aggregate Battery داد.
- SHABIN AirPods داده جداگانه Left/Right/Case داد.

این موارد `Observed` هستند و خودکار به فاز بعدی تبدیل نمی‌شوند.

## قانون انتخاب قابلیت

هر قابلیت جدید فقط وقتی وارد توسعه می‌شود که موارد زیر مشخص باشند:
1. از کدام نیاز یا اپ مرجع آمده است؟
2. Adopt / Adapt / Reject / Defer چیست؟
3. ارزش کاربری چیست؟
4. منبع واقعی داده یا کنترل چیست؟
5. Fallback چیست؟
6. کدام Platformها پشتیبانی می‌شوند؟
7. کدام Provider capability آن را Gate می‌کند؟
8. Success test چیست؟
9. مالک آن کدام Phase است؟

## سنجش پیشرفت

- Liquid Glass UI: `100% Completed`
- Project governance: `Completed`
- Overall project: `Unscored`

درصد کلی پروژه هنوز محاسبه نمی‌شود. ابتدا Scope فازهای Queued/Deferred و روش امتیازدهی باید به‌صورت جداگانه تعریف و تأیید شود.

## مرحله بعدی دقیق

`Design Phase 5 — Battery Alerts & Notification Policy`

هیچ Product Feature جدیدی قبل از Design و Plan مصوب Phase 5 پیاده‌سازی نمی‌شود.
