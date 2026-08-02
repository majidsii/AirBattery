# تست تصاویر واقعی محصولات در AirBattery

این نسخه هیچ تصویر محصولی را با هوش مصنوعی تولید نمی‌کند. تصاویر دقیق فقط از منابع ثبت‌شده در `assets/exact-artwork/sources.json` دریافت می‌شوند و قبل از نصب، مجوز زنده فایل در Wikimedia Commons بررسی می‌شود.

## دریافت تصاویر دقیق

```bash
python3 scripts/fetch-exact-artwork.py
```

پس از موفقیت باید این فایل‌ها ساخته شوند:

```bash
find apps/desktop/src/assets/device-artwork/exact -name '*.webp' -type f | sort
cat apps/desktop/src/assets/device-artwork/exact/ATTRIBUTION.generated.json
```

## اعتبارسنجی

```bash
python3 scripts/verify-exact-artwork.py
npm --prefix apps/desktop test
```

## ساخت کامل اوبونتو

`RUN_UBUNTU_VALIDATION.sh` به‌صورت پیش‌فرض تصاویر مجاز را قبل از Build دریافت می‌کند:

```bash
CARGO_INCREMENTAL=0 \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
./RUN_UBUNTU_VALIDATION.sh
```

در صورت قطعی اینترنت می‌توان فقط برای تست SVG fallback، دانلود را موقتاً رد کرد:

```bash
AIRBATTERY_SKIP_EXACT_ARTWORK_FETCH=1 ./RUN_UBUNTU_VALIDATION.sh
```

## پوشش فاز اول

- AirPods Pro نسل اول: گوشی تکی، جفت و کیس
- Samsung Galaxy Buds Live: جفت
- Samsung Galaxy Buds 2019: جفت
- Nothing Ear (1): گوشی تکی و کیس
- Google Pixel Buds Pro: کیس
- Huawei FreeBuds 6: جفت
- Xiaomi Buds 5: جفت و کیس
- Beats Studio Buds+: جفت
- Sony WF-1000XM4: جفت
- Soundcore Sport X20: جفت
- JBL Free: جفت

هر مدل یا حالت فاقد عکس تأییدشده، صادقانه از SVG عمومی همان خانواده استفاده می‌کند و عکس یک مدل دیگر به‌جای آن نمایش داده نمی‌شود.
