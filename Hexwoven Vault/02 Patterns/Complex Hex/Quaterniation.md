---
aliases: ["Quaterniation"]
tags: [hexcasting/pattern, mod/complex-hex, family/other, topic/math]
kind: "Pattern"
family: "Other"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:mattoquat"
signature: "waqqqqqeeawwaeaww"
start_direction: "SOUTH_EAST"
input: "matrix"
output: "quaternion"
cost: ""
iotas_in: ["[[Matrix]]"]
iotas_out: ["[[Quaternion]]"]
entry: "[[Quaternionics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Quaterniation

◇ **Pattern** · [[Complex Hex]] · 📖 [[Quaternionics]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="140" viewBox="0 0 170.0 140.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.8" cy="20.2" r="2"/><circle cx="34.6" cy="20.2" r="2"/><circle cx="63.4" cy="20.2" r="2"/><circle cx="92.2" cy="20.2" r="2"/><circle cx="121.0" cy="20.2" r="2"/><circle cx="149.8" cy="20.2" r="2"/><circle cx="20.2" cy="45.1" r="2"/><circle cx="49.0" cy="45.1" r="2"/><circle cx="77.8" cy="45.1" r="2"/><circle cx="106.6" cy="45.1" r="2"/><circle cx="135.4" cy="45.1" r="2"/><circle cx="164.2" cy="45.1" r="2"/><circle cx="5.8" cy="70.1" r="2"/><circle cx="34.6" cy="70.1" r="2"/><circle cx="63.4" cy="70.1" r="2"/><circle cx="92.2" cy="70.1" r="2"/><circle cx="121.0" cy="70.1" r="2"/><circle cx="149.8" cy="70.1" r="2"/><circle cx="20.2" cy="95.0" r="2"/><circle cx="49.0" cy="95.0" r="2"/><circle cx="77.8" cy="95.0" r="2"/><circle cx="106.6" cy="95.0" r="2"/><circle cx="135.4" cy="95.0" r="2"/><circle cx="164.2" cy="95.0" r="2"/><circle cx="5.8" cy="120.0" r="2"/><circle cx="34.6" cy="120.0" r="2"/><circle cx="63.4" cy="120.0" r="2"/><circle cx="92.2" cy="120.0" r="2"/><circle cx="121.0" cy="120.0" r="2"/><circle cx="149.8" cy="120.0" r="2"/></g><polyline class="hexpat-base" points="34.6,20.2 49.0,45.1 63.4,70.1 77.8,45.1 63.4,20.2 34.6,20.2 20.2,45.1 34.6,70.1 63.4,70.1 77.8,95.0 63.4,120.0 92.2,120.0 121.0,120.0 149.8,120.0 135.4,95.0 149.8,70.1 121.0,70.1 92.2,70.1 63.4,70.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:519;--dur:5.0s" points="34.6,20.2 49.0,45.1 63.4,70.1 77.8,45.1 63.4,20.2 34.6,20.2 20.2,45.1 34.6,70.1 63.4,70.1 77.8,95.0 63.4,120.0 92.2,120.0 121.0,120.0 149.8,120.0 135.4,95.0 149.8,70.1 121.0,70.1 92.2,70.1 63.4,70.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.6" y1="20.2" x2="39.2" y2="28.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,33.9 36.4,29.7 41.9,26.6" fill="#fc77be"/><circle cx="34.6" cy="20.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqqqqqeeawwaeaww` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `matrix` ([[Matrix]])
- **Leaves:** `quaternion` ([[Quaternion]])
- **ID:** `complexhex:mattoquat`

```text
before   … │ matrix      ◂ top
after    … │ quaternion  ◂ top
```

## Description

Pushes a quaternion that (should) reperesent the same rotation as the matrix

## Related patterns (same book page)

- [[Quaternionic Exal]] `num, vec → quaternion`
- [[Quaternionic Dntg]] `quaternion → num, vec`
- [[Quaternionic Rfln. 1]] `— → quaternion`
- [[Quaternionic Rfln. i]] `— → quaternion`
- [[Quaternionic Rfln j]] `— → quaternion`
- [[Quaternionic Rfln k]] `— → quaternion`
- [[Quaterionic Prfn. 1]] `quaternion → num`
- [[Quaterionic Prfn. i]] `quaternion → num`
- [[Quaterionic Prfn. j]] `quaternion → num`
- [[Quaterionic Prfn k]] `quaternion → num`
- [[Quaternionic Inverse]] `quaternion → quat`
- [[Matrixification]] `quaternion → matrix`
- [[Axis Angle Distillation]] `vec, num → quaternion`

## Sources

- 📖 **Guidebook:** [[Quaternionics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Quaterniation](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/quaternionics@complexhex:mattoquat)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMatrixToQuaternion` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:mattoquat`
