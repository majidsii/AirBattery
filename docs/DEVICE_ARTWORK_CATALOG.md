# Device artwork catalog

AirBattery uses an **exact-or-generic** product-artwork policy. The production registry starts empty and accepts only reviewed, model-accurate, offline 3D renders whose identity, orientation, source, license, and review date are recorded in source control.

## Runtime rules

- Resolve artwork by the exact normalized `modelKey` and exact component mode: `left`, `right`, `pair`, or `case`.
- Never reuse a render from a similar model or hardware generation.
- Never mirror a left-earbud asset to impersonate a right-earbud asset.
- Never download product artwork at runtime.
- Never ship AI-generated approximations, unreviewed product photographs, or the former model-specific SVG drawings as exact product imagery.
- When an exact reviewed asset is unavailable, display a small neutral category glyph for earbuds, headsets, speakers, mice, keyboards, controllers, styluses, or generic Bluetooth devices.

## Audited registry

The registry is defined in:

```text
apps/desktop/src/domain/pre-rendered-artwork.ts
```

The audited registry starts empty. This is deliberate: a generic glyph is more truthful than an inaccurate product image.

Every accepted entry must include:

```ts
{
  src: string;
  modelKey: string;
  mode: 'left' | 'right' | 'pair' | 'case';
  license: string;
  author: string;
  source: string;
  reviewedAt: 'YYYY-MM-DD';
  renderMethod: 'offline-3d';
}
```

## Asset layout

Approved assets should be grouped by exact commercial model:

```text
apps/desktop/src/assets/device-artwork/reviewed/<model-key>/
  left.webp
  right.webp
  pair.webp
  case.webp
```

A model does not need every mode, but the UI falls back to the neutral category glyph for any missing mode. It must not borrow another mode or model.

## Review gate

Before registration, verify all of the following:

- exact product and hardware generation;
- silhouette, vents, stems, tips, sensors, hinge, and case geometry;
- correct left/right orientation;
- one consistent camera, scale, light rig, and neutral color treatment;
- transparent edges at normal and high-DPI scale;
- no text, watermark, invented logo, or AI approximation;
- redistribution rights recorded in repository metadata;
- visual review in light and dark themes;
- battery values remain more prominent than product artwork.

The full authoring and review process is documented in [`ARTWORK_PIPELINE.md`](ARTWORK_PIPELINE.md).
