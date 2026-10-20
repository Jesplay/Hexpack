---
aliases: ["Robbie's Exaltation"]
tags: [hexcasting/pattern, mod/lapisworks, family/exaltation, topic/math]
kind: "Pattern"
family: "Exaltation"
topics: ["Math & Vectors"]
mod: "Lapisworks"
op_id: "lapisworks:robbie_exalt"
signature: "qaeaqaweaqa"
start_direction: "NORTH_WEST"
input: "[any], int, any"
output: "[any]"
cost: ""
iotas_in: ["[[Number]]", "[[List]]", "[[Any Iota]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Miscellaneous]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Robbie's Exaltation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Miscellaneous]] · Lapisworks

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="143.0,60.5 123.6,27.0 85.0,27.0 104.3,60.5 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="143.0,60.5 123.6,27.0 85.0,27.0 104.3,60.5 85.0,94.0 123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="60.5" x2="136.8" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,42.1 140.5,47.7 133.1,51.9" fill="#fc77be"/><circle cx="143.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqaweaqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[any], int, any` ([[Number]], [[List]], [[Any Iota]])
- **Leaves:** `[any]` ([[List]], [[Any Iota]])
- **ID:** `lapisworks:robbie_exalt`

```text
before   … │ [any] │ int │ any  ◂ top
after    … │ [any]              ◂ top
```

## Description

Inserts the given [[Iotas|iota]] into the given list before the specified index. An input of \[a, b, c\], 1, d would return \[a, d, b, c\].

*Arf, arf!*

## Related patterns (same book page)

- [[Hadamard Distillation]] `vec, vec → vec`
- [[Dealer's Purification]] `[any] → any`
- [[Minimum Distillation]] `num, num → num`
- [[Maximum Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Miscellaneous]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `EstrogenExalt` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:robbie_exalt`
