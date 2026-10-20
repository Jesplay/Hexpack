---
aliases: ["Vector Disintegration"]
tags: [hexcasting/pattern, mod/hex-casting, family/disintegration, topic/math]
kind: "Pattern"
family: "Disintegration"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:deconstruct_vec"
signature: "qeeeee"
start_direction: "EAST"
input: "vector"
output: "num, num, num"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Mathematics]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Vector Disintegration

◇ **Pattern** · [[Hex Casting]] · 📖 [[Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✷ Disintegration
> This pattern takes **one** iota and spills out **many**: its name marks it as a **Disintegration**. See [[Pattern Naming Guide]] · [[Patterns by Family#Disintegration|all Disintegration patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:2.0s" points="27.0,60.5 65.7,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeeeee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `num, num, num` ([[Number]])
- **ID:** `hexcasting:deconstruct_vec`

```text
before   … │ vector           ◂ top
after    … │ num │ num │ num  ◂ top
```

## Description

Split a vector into its X, Y, and Z components (bottom to top).

## Example

`(1, 2, 3) → 1, 2, 3` (z ends up on top).

## Related patterns (same book page)

- [[Additive Distillation]] `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] `num|vec, num|vec → num|vec`
- [[Division Distillation]] `num|vec, num|vec → num|vec`
- [[Length Purification]] `num|vec → number`
- [[Power Distillation]] `num|vec, num|vec → num|vec`
- [[Floor Purification]] `num|vec → num|vec`
- [[Ceiling Purification]] `num|vec → num|vec`
- [[Vector Exaltation]] `num, num, num → vector`
- [[Modulus Distillation]] `num|vec, num|vec → num|vec`
- [[Axial Purification]] `vec|num → vec|num`
- [[Entropy Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Vector Disintegration](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/math@hexcasting:deconstruct_vec)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:deconstruct_vec`
