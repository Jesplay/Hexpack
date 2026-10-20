---
aliases: ["Pitch Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:rot_about/x"
signature: "aaqqqqqea"
start_direction: "SOUTH_WEST"
input: "vec, num"
output: "vec"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Hextra Vector Math]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Pitch Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Vector Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 65.7,60.5 104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="85.0,27.0 65.7,60.5 104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="78.8" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,45.4 75.1,35.6 82.5,39.9" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqqqqqea` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `vec` ([[Vector]])
- **ID:** `hextrapats:rot_about/x`

```text
before   … │ vec │ num  ◂ top
after    … │ vec        ◂ top
```

## Description

Rotates the vector by the given angle in radians around the X axis.

## Related patterns (same book page)

- [[Norm Purification]] `vec → vec`
- [[Length Distillation]] `vec, num → vec`
- [[Chaos Reflection]] `— → vec`
- [[Distance Distillation]] `vec, vec → num`
- [[Polar Distillation]] `num, num → vec`
- [[Polar Decomposition]] `vec → num, num`
- [[Yaw Distillation]] `vec, num → vec`
- [[Roll Distillation]] `vec, num → vec`
- [[Pitch Purification]] `num → vec`
- [[Yaw Purification]] `num → vec`
- [[Roll Purification]] `num → vec`

## Sources

- 📖 **Guidebook:** [[Hextra Vector Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:rot_about/x`
