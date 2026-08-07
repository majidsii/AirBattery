# Start Here: AirBattery

این فایل نقطه شروع اجباری هر چت جدید یا ادامه توسعه است.

## ترتیب مطالعه

1. `docs/project/AIRBATTERY_MASTER_ROADMAP.md`
2. آخرین Snapshot:
   `docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md`
3. Design و Plan فاز فعال، اگر وجود دارند
4. وضعیت Git

```bash
cd ~/projects/personal/AirBattery
git status -sb
git log -5 --oneline --decorate
```

## آخرین Snapshot

`docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md`

## قوانین قطعی

- تسک بعدی از روی آخرین باگ یا Observation دیده‌شده انتخاب نمی‌شود.
- هر قابلیت باید جای مشخصی در Roadmap داشته باشد.
- داده Left/Right/Case فقط وقتی نمایش داده می‌شود که واقعاً دریافت شده باشد.
- مقدار Aggregate به اجزای ساختگی تبدیل نمی‌شود.
- Synthetic Aggregate از Left/Right به‌عنوان داده واقعی ممنوع است.
- همه دستگاه‌های Active در داشبورد اصلی دیده می‌شوند.
- Preferred فقط Tray/Widget/Ordering را تحت‌تأثیر قرار می‌دهد.
- Active، Nearby، Known، Preferred و Pinned مفاهیم جدا هستند.
- فایل‌های Progress قبلی بازنویسی نمی‌شوند؛ هر بار Snapshot جدید ساخته می‌شود.
- درصد کلی پروژه فعلاً `Unscored` است.
- اگر فاز بعدی Design و Plan مصوب ندارد، پیاده‌سازی آن شروع نمی‌شود.
- اول Design باید نوشته و تأیید شود.

## وضعیت پایه

- Stable product branch: `main`
- Stable product commit: `3530385`
- آخرین Product Phase تکمیل‌شده: `Phase 4 — Liquid Glass Desktop Experience`
- آخرین PR محصول ادغام‌شده: `#8`
- Governance reconstruction: `Completed`
- اولین Product Phase ناقص: `Phase 5 — Battery Alerts & Notification Policy`
- Overall project: `Unscored`

## مرحله بعدی دقیق

`Design Phase 5 — Battery Alerts & Notification Policy`

Design این فاز باید قبل از هر Implementation مشخص کند:
- real notification data sources
- capability gating
- freshness/stale suppression
- per-device thresholds
- charging evidence
- deduplication/cooldowns
- Linux/Windows behavior
- persistence
- success tests

بعد از تأیید Design، Implementation Plan با RED → GREEN نوشته می‌شود.

## متن آماده برای چت جدید

> قبل از هر پیشنهاد یا پیاده‌سازی، فایل‌های
> `docs/project/START_HERE.md`،
> `docs/project/AIRBATTERY_MASTER_ROADMAP.md`
> و Snapshot
> `docs/project/progress/2026-08-07-01-governance-matrix-reconstruction-completed.md`
> را کامل بخوان.
> سپس Design و Plan فاز فعال را در صورت وجود بخوان و
> `git status -sb` و `git log -5 --oneline --decorate` را بررسی کن.
> فقط از Exact Next Action ثبت‌شده در جدیدترین Snapshot ادامه بده.
> اگر Design/Plan فاز بعدی هنوز وجود ندارد، وارد Implementation نشو.
