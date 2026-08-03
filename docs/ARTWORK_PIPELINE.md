# Product Artwork Pipeline

AirBattery follows an exact-or-generic artwork policy.

## Runtime behavior

1. Use a reviewed pre-rendered asset only when its `modelKey` exactly matches the detected model and its mode exactly matches `left`, `right`, `pair`, or `case`.
2. Left and right are separate assets. Production product artwork is never mirrored.
3. When reviewed artwork is unavailable, show a small neutral inline category glyph that makes no model-specific claim.
4. Never substitute a photograph, SVG drawing, or render from a similar product.
5. Never download product art at runtime.

The former model-specific SVG catalog and experimental photo/AI directories are intentionally absent from production source.

## Preferred production format

Product visuals are modeled or obtained under compatible redistribution terms, checked against the physical product, and rendered offline. Runtime 3D is intentionally avoided. The application ships transparent WebP or AVIF outputs rather than loading GLB models in the WebView.

Each approved model should provide:

```text
<model-key>/
  left.webp
  right.webp
  pair.webp
  case.webp
  metadata.json
```

`metadata.json` must identify:

- exact commercial model and hardware generation;
- author or model creator;
- source and redistribution license;
- modeling/render method;
- camera and lighting preset version;
- reviewer and review date;
- source-reference list used to check geometry.

## Review gate

An asset is accepted only when all of the following are true:

- exact model identity is documented;
- silhouette, vents, stems, tips, sensors, hinges, and case geometry match trusted references;
- left and right orientation are correct;
- transparent edges are clean at 1× and 2× UI scale;
- all views use one camera, scale, light rig, and neutral color treatment;
- no text, watermark, invented logo, or AI-generated approximation is present;
- redistribution terms are recorded and compatible with the repository;
- the asset remains legible in light and dark themes;
- the product art never competes visually with the battery percentage.

Until an asset passes this gate, the neutral category glyph is the correct production behavior.
