---
aliases: ["Vector Exaltation"]
tags: [hexcasting/pattern, mod/hex-casting, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:construct_vec"
signature: "eqqqqq"
start_direction: "EAST"
input: "num, num, num"
output: "vector"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Mathematics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vector Exaltation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:2.0s" points="27.0,60.5 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num, num` ([[Number]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:construct_vec`

```text
before   … │ num │ num │ num  ◂ top
after    … │ vector           ◂ top
```

## Description

Combine three numbers at the top of the [[The Stack|stack]] into a vector's X, Y, and Z components (bottom to top).

## Example

`1, 2, 3 → (1, 2, 3)`. Handy for building exact offsets, e.g. `0, 5, 0` for "5 blocks up".

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] `num|vec, num|vec → num|vec`
- [[Division Distillation]] `num|vec, num|vec → num|vec`
- [[Length Purification]] `num|vec → number`
- [[Power Distillation]] `num|vec, num|vec → num|vec`
- [[Floor Purification]] `num|vec → num|vec`
- [[Ceiling Purification]] `num|vec → num|vec`
- [[Vector Disintegration]] `vector → num, num, num`
- [[Modulus Distillation]] `num|vec, num|vec → num|vec`
- [[Axial Purification]] `vec|num → vec|num`
- [[Entropy Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Vector Exaltation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:construct_vec)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:construct_vec`
