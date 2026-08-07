# AirBattery Master Roadmap

## هدف محصول

ساخت یک اپ حرفه‌ای، دقیق و Cross-platform برای نمایش و مدیریت باتری
دستگاه‌های بلوتوثی با استفاده از بهترین قابلیت‌های اپ‌های موفق این حوزه،
بدون کپی کورکورانه و بدون جعل داده.

## اصول معماری

- پشتیبانی Capability-based و مستقل از برند
- Provider جدا برای هر پروتکل یا خانواده دستگاه
- Linux و Windows به‌عنوان پلتفرم‌های اصلی
- داشبورد خودکار برای نمایش همه دستگاه‌های Active
- Preferred device فقط برای Widget/Tray/Ordering
- عدم ساخت داده جعلی برای Left/Right/Case
- استفاده از Artwork دقیق فقط بعد از تشخیص مدل و Review
- CI سبز قبل از Merge

## وضعیت‌ها

- `Completed`
- `Active`
- `Queued`
- `Blocked`
- `Observed`
- `Superseded`

## Milestoneهای تأییدشده

| Milestone | وضعیت |
|---|---|
| Linux foundation | Completed baseline |
| GNOME integration | Completed baseline |
| Release engineering | Completed baseline |
| Universal battery runtime | Completed baseline |
| Hybrid native architecture | Completed |
| Logo and live status colors | Completed |
| Liquid Glass desktop UI | Completed in PR #8 |
| Durable roadmap and handoff system | Active |

## قانون انتخاب قابلیت

هر قابلیت جدید فقط وقتی وارد توسعه می‌شود که موارد زیر مشخص باشند:

1. از کدام نیاز یا اپ مرجع آمده است؟
2. برای AirBattery پذیرفته، تطبیق داده، رد یا Deferred شده؟
3. ارزش کاربری آن چیست؟
4. منبع واقعی داده چیست؟
5. رفتار Fallback چیست؟
6. روی چه پلتفرم‌هایی کار می‌کند؟
7. تست موفقیت آن چیست؟
8. در کدام فاز قرار دارد؟

## گیت فعلی Roadmap

قبل از شروع قابلیت بعدی باید:

1. ماتریس قبلاً تأییدشده اپ‌های موفق و قابلیت‌های آن‌ها بازیابی شود.
2. تصمیم Adopt/Adapt/Reject/Defer ثبت شود.
3. ترتیب فازها دوباره در همین فایل نوشته شود.
4. Milestoneهای تکمیل‌شده روی آن ترتیب Map شوند.
5. اولین فاز ناقص واقعی مشخص شود.

## Observations

- یک AirPods دیگر فقط Aggregate Battery داد.
- QCY T13 فقط Aggregate Battery داد.
- SHABIN AirPods داده جداگانه Left/Right/Case داد.

این موارد فعلاً `Observed` هستند و خودکار به فاز بعدی تبدیل نمی‌شوند.

## سنجش پیشرفت

- Liquid Glass UI: `100% Completed`
- Project governance: `Active`
- Overall project: `Unscored`

درصد کلی بعد از بازیابی ماتریس کامل قابلیت‌ها محاسبه می‌شود.
