# AirPods Proximity-Pairing Protocol

## Scope and evidence level

AirBattery independently implements publicly documented interoperability facts for Apple manufacturer data (`company id 0x004C`) carrying Continuity proximity-pairing message type `0x07`. This is not an Apple-supported API. A 2026-07-31 AirPods Pro 2020 raw capture verified the parser fields for lid-open, left-out, and both-out transitions; rebuilt desktop runtime/UI behavior remains a separate hardware-validation gate.

## Accepted payload

The parser receives bytes after the Bluetooth SIG company id. It accepts only:

- first byte `0x07`;
- at least 27 bytes;
- complete nibble positions needed for model, orientation, battery, and charging fields.

A different first byte returns `Ok(None)`. A recognized but shorter type-`0x07` payload returns a typed truncation error.

## Nibble layout used by the parser

Nibble indexes count from the high nibble of byte zero.

| Nibble | Meaning |
|---:|---|
| 7 | model identifier |
| 10 | orientation/status nibble; bit `0x02` controls left/right orientation |
| 12 | first earbud battery decile |
| 13 | second earbud battery decile |
| 14 | charging flags |
| 15 | case battery decile |

Battery nibbles `0..=10` map to `0..=100` in steps of ten. Nibbles `11..=15` mean unavailable and never become zero.

The orientation bit determines whether nibble 12 is left or right. Charging bits are swapped using the same orientation. Bit `0x04` is treated as the case charging flag. A component with an unavailable percentage always receives `ChargingState::Unknown`; charging is not inferred for an absent component.

## Captured AirPods Pro 2020 fixtures

The regression suite includes three sanitized 27-byte manufacturer payloads from the user's `SHABIN` device:

| Physical state | Parsed result |
|---|---|
| Lid open, both earbuds seated | Left 100%, right 100%, case 20%; both earbuds charging |
| Left removed, right seated | Left 100% not charging; right 100% charging; case 20% |
| Both earbuds removed | Left 100%, right 100%; case unavailable; neither earbud charging |

The case being unavailable after both earbuds are removed is preserved as missing protocol evidence rather than converted to zero. The registry may retain an earlier case value until its normal freshness deadline.

## Current model mapping

| Nibble | Model label |
|---:|---|
| `0x2` | AirPods 1 |
| `0xF` | AirPods 2 |
| `0x3` | AirPods 3 |
| `0xE` | AirPods Pro |
| `0x4` | AirPods Pro 2 |
| `0xA` | AirPods Max |
| other | `Unknown(nibble)` |

Model mapping is descriptive metadata and does not alter battery parsing.

## Freshness behavior

The default case TTL is 90 seconds, while earbuds and aggregate values use five minutes. A temporarily absent case value does not immediately erase the last known case percentage; its original timestamp is retained and it becomes stale at the original deadline. A disconnected device is immediately presented as stale.

## Security and trust

Manufacturer advertisements are not authenticated. The parser validates shape and semantics but cannot prove that an advertisement came from a genuine Apple product. AirBattery therefore treats the result as protocol-specific evidence, not cryptographic identity proof.

## Research and licensing

The implementation was written independently. The MIT-licensed `hudsonbrendon/apple-ble` project was used as a protocol research reference. GPL projects were consulted only for interoperability comparison; no GPL source is included. See `crates/protocol-airpods/NOTICE.md`.

Reference locations:

- https://github.com/hudsonbrendon/apple-ble
- https://github.com/delphiki/AirStatus
- https://github.com/d4rken-org/capod
- https://github.com/adolfintel/OpenPods
- https://github.com/kavishdevar/librepods
