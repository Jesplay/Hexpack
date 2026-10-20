---
aliases: ["Quaterionic Prfn.: i"]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:qx"
signature: "wdeeeeeqq"
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

# Quaterionic Prfn.: i

◇ **Pattern** · [[Complex Hex]] · 📖 [[Quaternionics]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="46.4,27.0 65.7,60.5 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 104.3,127.4 143.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="46.4,27.0 65.7,60.5 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 104.3,127.4 143.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="27.0" x2="52.5" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,45.4 48.9,39.9 56.2,35.6" fill="#fc77be"/><circle cx="46.4" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdeeeeeqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `quaternion` ([[Quaternion]])
- **Leaves:** `num` ([[Number]])
- **ID:** `complexhex:qx`

```text
before   … │ quaternion  ◂ top
after    … │ num         ◂ top
```

## Description

Pushes the coefficient of i.

## Related patterns (same book page)

- [[Quaternionic Exal]] `num, vec → quaternion`
- [[Quaternionic Dntg]] `quaternion → num, vec`
- [[Quaternionic Rfln. 1]] `— → quaternion`
- [[Quaternionic Rfln. i]] `— → quaternion`
- [[Quaternionic Rfln j]] `— → quaternion`
- [[Quaternionic Rfln k]] `— → quaternion`
- [[Quaterionic Prfn. 1]] `quaternion → num`
- [[Quaterionic Prfn. j]] `quaternion → num`
- [[Quaterionic Prfn k]] `quaternion → num`
- [[Quaternionic Inverse]] `quaternion → quat`
- [[Matrixification]] `quaternion → matrix`
- [[Quaterniation]] `matrix → quaternion`
- [[Axis Angle Distillation]] `vec, num → quaternion`

## Sources

- 📖 **Guidebook:** [[Quaternionics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Quaterionic Prfn.: i](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/quaternionics@complexhex:qx)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetQuaternionComponent` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:qx`
