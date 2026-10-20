---
aliases: ["Theta Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:angle_dist"
signature: "awdaqqqqqea"
start_direction: "NORTH_EAST"
input: "num, num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hextra Math]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Theta Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="123.6,127.4 143.0,94.0 104.3,94.0 65.7,94.0 85.0,60.5 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 85.0,60.5 104.3,27.0 65.7,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="123.6,127.4 143.0,94.0 104.3,94.0 65.7,94.0 85.0,60.5 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 85.0,60.5 104.3,27.0 65.7,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="127.4" x2="129.8" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="134.3,109.0 133.5,118.8 126.1,114.6" fill="#fc77be"/><circle cx="123.6" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awdaqqqqqea` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hextrapats:angle_dist`

```text
before   … │ num │ num  ◂ top
after    … │ num        ◂ top
```

## Description

Returns the shortest angular distance between two angles in radians.

## Other uses (same pattern, other inputs)

### [[Hextra Patterns]] · 📖 [[Hextra Vector Math]]

`vec, vec` → `num`

Returns the shortest angular distance on a sphere between two vectors in radians

## Related patterns (same book page)

- [[Radian Purification]] `num → num`
- [[Degree Purification]] `num → num`
- [[Reflection Purification]] `num/vec → num/vec`
- [[Incrementation Purification]] `num → num`
- [[Decrementation Purification]] `num → num`
- [[Rounding Purification]] `num/vec → num/vec`
- [[Rounding Distillation]] `num/vec, num → num/vec`
- [[Entropy Purification]] `num → num`
- [[Entropy Distillation]] `num, num → num`
- [[Scientific Purification]] `num/vec → num/vec`
- [[Approach Exaltation]] `num, num, num → num`
- [[Turning Exaltation]] `num, num, num → num`

## Sources

- 📖 **Guidebook:** [[Hextra Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:angle_dist`
