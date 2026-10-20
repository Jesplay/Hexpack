---
aliases: ["Ranger's Exaltation"]
tags: [hexcasting/pattern, mod/hexthings, family/exaltation, topic/lists]
kind: "Pattern"
family: "Exaltation"
topics: ["Lists"]
mod: "HexThings"
op_id: "hexthings:make_range"
signature: "qqqqqeweaqa"
start_direction: "NORTH_WEST"
input: "num, num, (num | null)"
output: "list"
cost: ""
iotas_in: ["[[Number]]", "[[Null]]"]
iotas_out: ["[[List]]"]
entry: "[[Advanced List Operators]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Ranger's Exaltation

◇ **Pattern** · [[HexThings]] · 📖 [[Advanced List Operators]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 98.3,41.6 124.8,41.6 138.1,64.6 151.4,41.6 138.1,18.6 124.8,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:319;--dur:3.4s" points="71.7,41.6 58.4,18.6 31.9,18.6 18.6,41.6 31.9,64.6 58.4,64.6 71.7,41.6 98.3,41.6 124.8,41.6 138.1,64.6 151.4,41.6 138.1,18.6 124.8,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="71.7" y1="41.6" x2="67.5" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="64.4,28.9 70.0,32.8 64.9,35.7" fill="#fc77be"/><circle cx="71.7" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqeweaqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num, (num | null)` ([[Number]], [[Null]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexthings:make_range`

## Description

Creates a list from num #1 to num #2, with a step size of num #3 (or 1 if passed null). For example, A [[The Stack|stack]] of \[1, 2, 0.5\] would result in a list of \[1, 1.5\].

This pattern will [[Mishaps|mishap]] if the step size is 0, or if the resulting list would be larger than Nature allows.

## Related patterns (same book page)

- [[Retention Distillation (HexThings)]] `list, (num | [num]) → list`
- [[Flattening Exaltation]] `list, num, list → list`
- [[Rejection Distillation (HexThings)]] `list, (num | [num]) → list`
- [[Ferris Distillation (HexThings)]] `list, num → list`
- [[Deduction Purification]] `list → list`

## Sources

- 📖 **Guidebook:** [[Advanced List Operators]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexthings:make_range`
