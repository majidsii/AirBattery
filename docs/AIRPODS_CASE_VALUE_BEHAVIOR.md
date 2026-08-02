# AirPods case battery behavior

Active Apple accessory battery packets can temporarily report `0xFF` (255) for a component that is not currently readable. This is an unavailable sentinel, not a battery percentage.

AirBattery handles it component-by-component:

1. Valid left and right values in the same packet are accepted.
2. A `0xFF` case record becomes an unavailable case update rather than rejecting the entire packet.
3. The registry keeps the previous valid case percentage and its original timestamp.
4. The UI labels retained values as last known when freshness expires; it never presents 255%.
5. A later valid case record replaces the retained value immediately.

The main dashboard automatically shows every connected device. Temporarily unavailable accessory components retain their last truthful reading instead of disappearing.
