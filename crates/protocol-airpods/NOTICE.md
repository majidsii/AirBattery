# Protocol research notice

This crate independently implements public interoperability facts for Apple
Continuity proximity-pairing advertisements. No GPL source code is included.

Research references consulted:

- `hudsonbrendon/apple-ble` (MIT): message type, nibble offsets, orientation,
  model mapping, and decile semantics.
- `delphiki/AirStatus`, `d4rken-org/capod`, `adolfintel/OpenPods`, and
  `kavishdevar/librepods`: interoperability comparison only; source code was
  not copied into AirBattery.

The parser accepts type `0x07` payloads containing at least the required 27 bytes and does not
claim that the case advertises continuously.
