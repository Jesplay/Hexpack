---
aliases: ["Range Exaltation II"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/exaltation, topic/logic]
kind: "Pattern"
family: "Exaltation"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:out_range"
signature: "eaae"
start_direction: "SOUTH_EAST"
input: "num/vec, num, num, num"
output: "bool"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Hextra Logic]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Range Exaltation II

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0 65.1,85.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0 65.1,85.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="38.0" x2="46.7" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,63.8 41.5,56.0 51.8,50.1" fill="#fc77be"/><circle cx="38.0" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaae` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num/vec, num, num, num` ([[Number]], [[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hextrapats:out_range`

```text
before   … │ num/vec │ num │ num │ num  ◂ top
after    … │ bool                       ◂ top
```

## Description

If the first number is not between the second and third, return True. Otherwise, return False. If the first value is a vector, compare its length instead.

The fourth number seems to describe how to check if the value is outside the range.
- 0: val &lt; min or max &lt; val
- 1: val &lt;= min or max &lt; val
- 2: val &lt; min or max &lt;= val
- 3: val &lt;= min or max &lt;= val

## Related patterns (same book page)

- [[Length Equality Distillation]] `vec, vec → bool`
- [[Length Inequality Distillation]] `vec, vec → bool`
- [[Range Exaltation]] `num/vec, num, num, num → bool`

## Sources

- 📖 **Guidebook:** [[Hextra Logic]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:out_range`
