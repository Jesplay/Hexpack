---
aliases: ["Yaw Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:rot_about/y"
signature: "aaqqqqqew"
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

# Yaw Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Vector Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="69.3,22.0 53.5,49.3 85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:2.8s" points="69.3,22.0 53.5,49.3 85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="22.0" x2="64.2" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,37.0 61.2,29.0 67.2,32.5" fill="#fc77be"/><circle cx="69.3" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqqqqqew` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `vec` ([[Vector]])
- **ID:** `hextrapats:rot_about/y`

```text
before   … │ vec │ num  ◂ top
after    … │ vec        ◂ top
```

## Description

Rotates the vector by the given angle in radians around the Y axis.

## Related patterns (same book page)

- [[Norm Purification]] `vec → vec`
- [[Length Distillation]] `vec, num → vec`
- [[Chaos Reflection]] `— → vec`
- [[Distance Distillation]] `vec, vec → num`
- [[Polar Distillation]] `num, num → vec`
- [[Polar Decomposition]] `vec → num, num`
- [[Pitch Distillation]] `vec, num → vec`
- [[Roll Distillation]] `vec, num → vec`
- [[Pitch Purification]] `num → vec`
- [[Yaw Purification]] `num → vec`
- [[Roll Purification]] `num → vec`

## Sources

- 📖 **Guidebook:** [[Hextra Vector Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:rot_about/y`
