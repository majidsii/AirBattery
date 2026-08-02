#!/usr/bin/env python3
"""Independent executable checks for documented AirPods fixture semantics.

This does not replace Rust tests. It guards the research fixture and resolver
ordering while the current container lacks a Rust toolchain.
"""

from dataclasses import dataclass


def nibble_at(data: bytes, index: int) -> int:
    byte = data[index // 2]
    return byte >> 4 if index % 2 == 0 else byte & 0x0F


def battery(nibble: int) -> int | None:
    return nibble * 10 if nibble <= 10 else None


def parse(data: bytes) -> dict[str, object] | None:
    if not data or data[0] != 0x07:
        return None
    if len(data) < 27:
        raise ValueError("truncated")
    flipped = nibble_at(data, 10) & 0x02 == 0
    left_index, right_index = ((12, 13) if flipped else (13, 12))
    charge = nibble_at(data, 14)
    return {
        "model": nibble_at(data, 7),
        "left": battery(nibble_at(data, left_index)),
        "right": battery(nibble_at(data, right_index)),
        "case": battery(nibble_at(data, 15)),
        "left_charging": bool(charge & (0b0010 if flipped else 0b0001)),
        "right_charging": bool(charge & (0b0001 if flipped else 0b0010)),
        "case_charging": bool(charge & 0b0100),
    }


@dataclass(frozen=True)
class Candidate:
    has_value: bool
    fresh: bool
    confidence: int
    source: int
    provider_priority: int
    updated_at: int
    provider_id: str


def key(candidate: Candidate) -> tuple[object, ...]:
    return (
        candidate.has_value,
        candidate.fresh,
        candidate.confidence,
        candidate.source,
        candidate.updated_at,
        candidate.provider_priority,
        tuple(-ord(char) for char in candidate.provider_id),
    )


def main() -> None:
    payload = bytearray(27)
    payload[0] = 0x07
    payload[3] = 0x0E
    payload[5] = 0x20
    payload[6] = 0x87
    payload[7] = 0x45

    parsed = parse(bytes(payload))
    assert parsed == {
        "model": 0xE,
        "left": 70,
        "right": 80,
        "case": 50,
        "left_charging": False,
        "right_charging": False,
        "case_charging": True,
    }

    payload[5] = 0x00
    flipped = parse(bytes(payload))
    assert flipped is not None
    assert flipped["left"] == 80 and flipped["right"] == 70

    fresh = Candidate(True, True, 3, 50, 100, 8, "airpods")
    stale = Candidate(True, False, 4, 30, 80, 10, "bluez")
    assert max((fresh, stale), key=key) is fresh

    old_high_priority = Candidate(True, True, 2, 20, 100, 5, "old")
    new_low_priority = Candidate(True, True, 2, 20, 1, 10, "new")
    assert max((old_high_priority, new_low_priority), key=key) is new_low_priority

    print("reference checks: 4 passed")


if __name__ == "__main__":
    main()
