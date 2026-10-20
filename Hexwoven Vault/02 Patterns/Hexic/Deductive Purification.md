---
aliases: ["Deductive Purification"]
tags: [hexcasting/pattern, mod/hexic, family/purification, topic/lists]
kind: "Pattern"
family: "Purification"
topics: ["Lists"]
mod: "Hexic"
op_id: "hexic:where"
signature: "qaeaqwdd"
start_direction: "NORTH_WEST"
input: "[bool]"
output: "[int]"
cost: ""
iotas_in: ["[[Boolean]]", "[[List]]"]
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Advanced Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Deductive Purification

◇ **Pattern** · [[Hexic]] · 📖 [[Advanced Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,68.3 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 117.7,30.5 139.5,68.3 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:392;--dur:2.5s" points="95.9,68.3 74.1,30.5 30.5,30.5 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 117.7,30.5 139.5,68.3 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="68.3" x2="88.9" y2="56.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="83.9,47.5 93.1,53.8 84.8,58.6" fill="#fc77be"/><circle cx="95.9" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqwdd` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[bool]` ([[Boolean]], [[List]])
- **Leaves:** `[int]` ([[Number]], [[List]])
- **ID:** `hexic:where`

```text
before   … │ [bool]  ◂ top
after    … │ [int]   ◂ top
```

## Description

Creates a list of the indices for which True is present. For example, \[True, False, True\] becomes \[0, 2\]. I can also mix bools and ints in the input list.

## Other uses (same pattern, other inputs)

### [[Hexic]] · 📖 [[Advanced Lists]]

`[num]` → `[int]`

Adds each index to the list based on the value. For example, \[1, 2, 3\] becomes \[0, 1, 1, 2, 2, 2\]. Zero or negative numbers do not appear at all.

## Related patterns (same book page)

- [[Ferris Distillation]] `[a], int → [a]`
- [[Retention Distillation]] `[a], int → [a]`
- [[Rejection Distillation]] `[a], int → [a]`
- [[Moses's Gambit]] `[a], (a, a → bool) → [[a]]`
- [[Excisor's Gambit]] `[a], int → [a], [a], a`
- [[Refinement Distillation]] `[a], [a → bool] → [a]`

## Sources

- 📖 **Guidebook:** [[Advanced Lists]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexic:where`
