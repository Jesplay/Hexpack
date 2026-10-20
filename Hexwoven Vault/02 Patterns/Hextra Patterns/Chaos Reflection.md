---
aliases: ["Chaos Reflection"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:rand_vec"
signature: "eeeeeqeqqq"
start_direction: "EAST"
input: ""
output: "vec"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Hextra Vector Math]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Chaos Reflection

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Vector Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 145.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="80.1,97.8 109.8,97.8 124.6,123.5 109.8,149.2 80.1,149.2 65.3,123.5 80.1,97.8 65.3,72.2 80.1,46.5 65.3,20.8 35.6,20.8 20.8,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:3.1s" points="80.1,97.8 109.8,97.8 124.6,123.5 109.8,149.2 80.1,149.2 65.3,123.5 80.1,97.8 65.3,72.2 80.1,46.5 65.3,20.8 35.6,20.8 20.8,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="80.1" y1="97.8" x2="89.6" y2="97.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.4,97.8 89.6,101.1 89.6,94.6" fill="#fc77be"/><circle cx="80.1" cy="97.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeeqeqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vec` ([[Vector]])
- **ID:** `hextrapats:rand_vec`

```text
before   …        ◂ top
after    … │ vec  ◂ top
```

## Description

Returns a unit vector facing a random direction.

## Related patterns (same book page)

- [[Norm Purification]] `vec → vec`
- [[Length Distillation]] `vec, num → vec`
- [[Distance Distillation]] `vec, vec → num`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRandVec` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:rand_vec`
