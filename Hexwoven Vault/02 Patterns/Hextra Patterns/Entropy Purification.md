---
aliases: ["Entropy Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:rand_zero"
signature: "dedqeqqq"
start_direction: "EAST"
input: "num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hextra Math]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Entropy Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="94.4,115.3 129.3,115.3 111.8,145.5 76.9,145.5 94.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:2.5s" points="94.4,115.3 129.3,115.3 111.8,145.5 76.9,145.5 94.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="94.4" y1="115.3" x2="105.5" y2="115.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="113.6,115.3 105.5,119.1 105.5,111.4" fill="#fc77be"/><circle cx="94.4" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedqeqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hextrapats:rand_zero`

```text
before   … │ num  ◂ top
after    … │ num  ◂ top
```

## Description

Returns a random number between 0 (inclusive) and the number (exclusive).

## Related patterns (same book page)

- [[Radian Purification]] `num → num`
- [[Degree Purification]] `num → num`
- [[Reflection Purification]] `num/vec → num/vec`
- [[Incrementation Purification]] `num → num`
- [[Decrementation Purification]] `num → num`
- [[Rounding Purification]] `num/vec → num/vec`
- [[Rounding Distillation]] `num/vec, num → num/vec`
- [[Entropy Distillation]] `num, num → num`
- [[Scientific Purification]] `num/vec → num/vec`
- [[Approach Exaltation]] `num, num, num → num`
- [[Theta Distillation]] `num, num → num`
- [[Turning Exaltation]] `num, num, num → num`

## Sources

- 📖 **Guidebook:** [[Hextra Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRandZero` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:rand_zero`
