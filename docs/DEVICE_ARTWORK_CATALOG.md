# Device artwork catalog

AirBattery ships its device artwork locally as original SVG illustrations. Official product pages are used only as visual and naming references; no vendor raster image, downloaded SVG, logo, or trademark artwork is bundled.

## Design rules

- Match the physical family: stem, semi-in-ear, in-ear with tip, bean, ring, clip, ear-hook, or over-ear.
- Select by exact model name/model code first, then product family, then category fallback.
- Keep separate `single`, `pair`, and `case` assets for every visual family.
- Reuse one SVG family when multiple models have materially the same silhouette.
- Never infer a battery value from artwork or model identity.
- Keep all files self-contained vector XML; embedded PNG/JPEG assets are forbidden.

## Official reference pages

- Apple AirPods compare: https://www.apple.com/airpods/compare/
- Apple AirPods and charging-case identification: https://support.apple.com/109525
- QCY current product catalog: https://www.qcy.com/
- QCY support model index: https://www.qcy.com/de/pages/support-center-arcbuds
- Xiaomi global TWS catalog: https://www.mi.com/global/product-list/tws-earphones/
- Soundcore true-wireless catalog: https://www.soundcore.com/collections/true-wireless-earbuds
- Samsung Galaxy Buds catalog: https://www.samsung.com/us/mobile/audio/headphones/galaxy-buds/
- Sony truly-wireless catalog: https://electronics.sony.com/audio/headphones/truly-wireless-earbuds/c/truly-wireless-earbuds
- JBL wireless earbuds: https://www.jbl.com/wireless-earbuds/
- Google Pixel Buds: https://store.google.com/category/earbuds
- Nothing audio: https://nothing.tech/collections/audio
- OnePlus audio: https://www.oneplus.com/audio
- Huawei audio: https://consumer.huawei.com/en/headphones/
- Beats earbuds: https://www.beatsbydre.com/earbuds

## Current visual families

The catalog contains 29 original visual families and 87 SVG files:

- Apple: classic AirPods, short-stem AirPods, AirPods Pro, AirPods Pro 3, AirPods Max.
- QCY: stem, half-in-ear/open, bean/round, Crossky clip/open-ear, H-series over-ear.
- Xiaomi/Redmi: semi-in-ear and stem families.
- Soundcore: stem, round, and ear-hook/open-ear.
- Samsung: bean, round, and blade/stem.
- Sony: round and ring/open.
- JBL: stem and round.
- Google Pixel Buds, Nothing Ear, OnePlus Buds, Huawei FreeBuds/FreeClip, and Beats round/wing families.

## QCY model coverage

Exact aliases include:

- MeloBuds N70, N60, Pro, A30, N20, N50, N65, Neo, and ANC.
- T13, T13 Pro, T13 ANC, T13 ANC 2, T13X, and T17.
- AilyBuds Pro+, E10, and Clear; QCY Air.
- ArcBuds, ArcBuds Lite, QCY Buds, and QCY Buds ANC.
- Crossky C10, C30, C30S, C50, R70, Link, and GTR2.
- H2 Pro, H3, H3 Lite, H3 Pro, and H3S.
- Heroad/V200/VT200 families use the QCY over-ear artwork family.
- QCY speakers and watches are deliberately excluded from earbud artwork matching.

Unknown or renamed QCY models still fall back to the closest QCY family instead of a generic Bluetooth icon.
