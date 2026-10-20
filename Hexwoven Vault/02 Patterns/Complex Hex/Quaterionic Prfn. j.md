---
aliases: ["Quaterionic Prfn.: j"]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:qy"
signature: "wdeeeeeqe"
start_direction: "SOUTH_EAST"
input: "quaternion"
output: "num"
cost: ""
iotas_in: ["[[Quaternion]]"]
iotas_out: ["[[Number]]"]
entry: "[[Quaternionics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Quaterionic Prfn.: j

◇ **Pattern** · [[Complex Hex]] · 📖 [[Quaternionics]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,24.5 59.4,54.7 76.9,85.0 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 94.4,115.3 76.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="41.9,24.5 59.4,54.7 76.9,85.0 41.9,85.0 24.5,54.7 41.9,24.5 76.9,24.5 94.4,54.7 76.9,85.0 94.4,115.3 76.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="24.5" x2="47.5" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,41.1 44.2,36.1 50.9,32.2" fill="#fc77be"/><circle cx="41.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdeeeeeqe` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `quaternion` ([[Quaternion]])
- **Leaves:** `num` ([[Number]])
- **ID:** `complexhex:qy`

```text
before   … │ quaternion  ◂ top
after    … │ num         ◂ top
```

## Description

Pushes the coefficient of j.

## Related patterns (same book page)

- [[Quaternionic Exal]] `num, vec → quaternion`
- [[Quaternionic Dntg]] `quaternion → num, vec`
- [[Quaternionic Rfln. 1]] `— → quaternion`
- [[Quaternionic Rfln. i]] `— → quaternion`
- [[Quaternionic Rfln j]] `— → quaternion`
- [[Quaternionic Rfln k]] `— → quaternion`
- [[Quaterionic Prfn. 1]] `quaternion → num`
- [[Quaterionic Prfn. i]] `quaternion → num`
- [[Quaterionic Prfn k]] `quaternion → num`
- [[Quaternionic Inverse]] `quaternion → quat`
- [[Matrixification]] `quaternion → matrix`
- [[Quaterniation]] `matrix → quaternion`
- [[Axis Angle Distillation]] `vec, num → quaternion`

## Sources

- 📖 **Guidebook:** [[Quaternionics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Quaterionic Prfn.: j](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/quaternionics@complexhex:qy)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetQuaternionComponent` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:qy`
