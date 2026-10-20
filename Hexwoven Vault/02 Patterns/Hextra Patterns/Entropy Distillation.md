---
aliases: ["Entropy Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/math]
kind: "Pattern"
family: "Distillation"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:rand_range"
signature: "eeeqeqqq"
start_direction: "SOUTH_WEST"
input: "num, num"
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

# Entropy Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 145.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="124.6,123.5 109.8,149.2 80.1,149.2 65.3,123.5 80.1,97.8 65.3,72.2 80.1,46.5 65.3,20.8 35.6,20.8 20.8,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:2.5s" points="124.6,123.5 109.8,149.2 80.1,149.2 65.3,123.5 80.1,97.8 65.3,72.2 80.1,46.5 65.3,20.8 35.6,20.8 20.8,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="124.6" y1="123.5" x2="119.9" y2="131.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="116.4,137.7 117.0,130.1 122.7,133.4" fill="#fc77be"/><circle cx="124.6" cy="123.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeqeqqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hextrapats:rand_range`

```text
before   … │ num │ num  ◂ top
after    … │ num        ◂ top
```

## Description

Returns a random number between the first number (inclusive) and the second (exclusive).

## Related patterns (same book page)

- [[Radian Purification]] `num → num`
- [[Degree Purification]] `num → num`
- [[Reflection Purification]] `num/vec → num/vec`
- [[Incrementation Purification]] `num → num`
- [[Decrementation Purification]] `num → num`
- [[Rounding Purification]] `num/vec → num/vec`
- [[Rounding Distillation]] `num/vec, num → num/vec`
- [[Entropy Purification]] `num → num`
- [[Scientific Purification]] `num/vec → num/vec`
- [[Approach Exaltation]] `num, num, num → num`
- [[Theta Distillation]] `num, num → num`
- [[Turning Exaltation]] `num, num, num → num`

## Sources

- 📖 **Guidebook:** [[Hextra Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRandRange` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:rand_range`
