---
aliases: ["Quaternionic Exal."]
tags: [hexcasting/pattern, mod/complex-hex, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Complex Hex"
op_id: "complexhex:qmake"
signature: "waqqqqqe"
start_direction: "SOUTH_EAST"
input: "num, vec"
output: "quaternion"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Quaternion]]"]
entry: "[[Quaternionics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Quaternionic Exal.

◇ **Pattern** · [[Complex Hex]] · 📖 [[Quaternionics]] · Patterns › Complex Mathematics

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,29.8 72.3,66.6 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="51.0,29.8 72.3,66.6 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="29.8" x2="57.8" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.7,50.0 53.8,43.9 61.9,39.2" fill="#fc77be"/><circle cx="51.0" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqqqqqe` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, vec` ([[Number]], [[Vector]])
- **Leaves:** `quaternion` ([[Quaternion]])
- **ID:** `complexhex:qmake`

```text
before   … │ num │ vec   ◂ top
after    … │ quaternion  ◂ top
```

## Description

Pushes a Quaternion with num as the real coefficient and the vector's x, y, & z as the coefficients of i, j & k respectively.

## Related patterns (same book page)

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
- [[Quaterniation]] `matrix → quaternion`
- [[Axis Angle Distillation]] `vec, num → quaternion`

## Sources

- 📖 **Guidebook:** [[Quaternionics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Quaternionic Exal.](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/maths/quaternionics@complexhex:qmake)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpQMake` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:qmake`
