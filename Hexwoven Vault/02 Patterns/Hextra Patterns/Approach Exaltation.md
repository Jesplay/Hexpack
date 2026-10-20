---
aliases: ["Approach Exaltation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:approach"
signature: "dedqadeeed"
start_direction: "SOUTH_WEST"
input: "num, num, num"
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

# Approach Exaltation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 104.3,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,60.5 46.4,94.0 27.0,60.5 46.4,27.0 65.7,60.5 104.3,60.5 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="59.5" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,78.9 55.8,69.1 63.2,73.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedqadeeed` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num, num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hextrapats:approach`

```text
before   … │ num │ num │ num  ◂ top
after    … │ num              ◂ top
```

## Description

Moves the first value closer to the second by the given amount. Will not overshoot.

## Other uses (same pattern, other inputs)

### [[Hextra Patterns]] · 📖 [[Hextra Vector Math]]

`vec, vec, num` → `vec`

Moves the first vector towards the second by a given distance. Will not overshoot.

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
- [[Theta Distillation]] `num, num → num`
- [[Turning Exaltation]] `num, num, num → num`

## Sources

- 📖 **Guidebook:** [[Hextra Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:approach`
