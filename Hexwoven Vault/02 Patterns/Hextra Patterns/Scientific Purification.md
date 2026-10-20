---
aliases: ["Scientific Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:scientific_exp"
signature: "waqe"
start_direction: "SOUTH_EAST"
input: "num/vec"
output: "num/vec"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Number]]", "[[Vector]]"]
entry: "[[Hextra Math]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Scientific Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,66.6 51.0,103.4 72.3,140.2 93.5,103.4 72.3,66.6 93.5,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="29.8,66.6 51.0,103.4 72.3,140.2 93.5,103.4 72.3,66.6 93.5,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="66.6" x2="36.6" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,86.8 32.5,80.7 40.6,76.0" fill="#fc77be"/><circle cx="29.8" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqe` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num/vec` ([[Number]], [[Vector]])
- **Leaves:** `num/vec` ([[Number]], [[Vector]])
- **ID:** `hextrapats:scientific_exp`

```text
before   … │ num/vec  ◂ top
after    … │ num/vec  ◂ top
```

## Description

Multiplies the input by 10^n or 10^-n, where n is the length of the tail, depending on whether the start is similar to [[Multiplicative Distillation|Mult. Dstl.]] or [[Division Distillation|Div. Dstl.]]. The top 3 patterns are x10^1, x10^2, and x10^3. The bottom 3 are x10^-1, x10^-2, and x10^-3.

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
- [[Approach Exaltation]] `num, num, num → num`
- [[Theta Distillation]] `num, num → num`
- [[Turning Exaltation]] `num, num, num → num`

## Sources

- 📖 **Guidebook:** [[Hextra Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:scientific_exp`
