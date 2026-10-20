---
aliases: ["Flattening Exaltation"]
tags: [hexcasting/pattern, mod/hexthings, family/exaltation, topic/lists]
kind: "Pattern"
family: "Exaltation"
topics: ["Lists"]
mod: "HexThings"
op_id: "hexthings:insert"
signature: "wqwaeawqw"
start_direction: "NORTH_WEST"
input: "list, num, list"
output: "list"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[List]]"]
entry: "[[Advanced List Operators]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Flattening Exaltation

◇ **Pattern** · [[HexThings]] · 📖 [[Advanced List Operators]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="94.0" x2="136.8" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,75.6 140.5,81.1 133.1,85.4" fill="#fc77be"/><circle cx="143.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwaeawqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, num, list` ([[Number]], [[List]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexthings:insert`

```text
before   … │ list │ num │ list  ◂ top
after    … │ list               ◂ top
```

## Description

Similar to [[Surgeon's Exaltation]], but replaces the element of the first list instead with the *contents* of the second list, rather than with the list itself.

## Related patterns (same book page)

- [[Retention Distillation (HexThings)]] `list, (num | [num]) → list`
- [[Ranger's Exaltation]] `num, num, (num | null) → list`
- [[Rejection Distillation (HexThings)]] `list, (num | [num]) → list`
- [[Ferris Distillation (HexThings)]] `list, num → list`
- [[Deduction Purification]] `list → list`

## Sources

- 📖 **Guidebook:** [[Advanced List Operators]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexthings:insert`
