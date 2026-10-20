---
aliases: ["Distance Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:vec_dist"
signature: "aqqqqqeqeeeeed"
start_direction: "EAST"
input: "vec, vec"
output: "num"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hextra Vector Math]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Distance Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Vector Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="45.2,41.6 71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6 124.8,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:398;--dur:4.2s" points="45.2,41.6 71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6 124.8,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.2" y1="41.6" x2="53.7" y2="41.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="59.8,41.6 53.7,44.5 53.7,38.7" fill="#fc77be"/><circle cx="45.2" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqqqqeqeeeeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hextrapats:vec_dist`

```text
before   … │ vec │ vec  ◂ top
after    … │ num        ◂ top
```

## Description

Returns the distance between two vectors.

## Related patterns (same book page)

- [[Norm Purification]] `vec → vec`
- [[Length Distillation]] `vec, num → vec`
- [[Chaos Reflection]] `— → vec`
- [[Polar Distillation]] `num, num → vec`
- [[Polar Decomposition]] `vec → num, num`
- [[Pitch Distillation]] `vec, num → vec`
- [[Yaw Distillation]] `vec, num → vec`
- [[Roll Distillation]] `vec, num → vec`
- [[Pitch Purification]] `num → vec`
- [[Yaw Purification]] `num → vec`
- [[Roll Purification]] `num → vec`

## Sources

- 📖 **Guidebook:** [[Hextra Vector Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpVecDist` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:vec_dist`
