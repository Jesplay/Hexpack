---
aliases: ["Ferris Distillation"]
tags: [hexcasting/pattern, mod/hexic, family/distillation, topic/lists]
kind: "Pattern"
family: "Distillation"
topics: ["Lists"]
mod: "Hexic"
op_id: "hexic:rotate"
signature: "qaeaqweeee"
start_direction: "NORTH_WEST"
input: "[a], int"
output: "[a]"
cost: ""
iotas_in: ["[[Number]]", "[[List]]", "[[Any Iota]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Advanced Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Ferris Distillation

◇ **Pattern** · [[Hexic]] · 📖 [[Advanced Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="76.3,54.3 59.0,24.3 24.3,24.3 41.6,54.3 24.3,84.4 59.0,84.4 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:382;--dur:3.1s" points="76.3,54.3 59.0,24.3 24.3,24.3 41.6,54.3 24.3,84.4 59.0,84.4 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="54.3" x2="70.8" y2="44.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="66.8,37.8 74.1,42.8 67.5,46.6" fill="#fc77be"/><circle cx="76.3" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqweeee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[a], int` ([[Number]], [[List]], [[Any Iota]])
- **Leaves:** `[a]` ([[List]], [[Any Iota]])
- **ID:** `hexic:rotate`

```text
before   … │ [a] │ int  ◂ top
after    … │ [a]        ◂ top
```

## Description

Rotates the list leftward: an argument of 1 moves the first item of the list to the end. Negative numbers allow rotating the list rightward.

## Related patterns (same book page)

- [[Retention Distillation]] `[a], int → [a]`
- [[Rejection Distillation]] `[a], int → [a]`
- [[Moses's Gambit]] `[a], (a, a → bool) → [[a]]`
- [[Deductive Purification]] `[bool] → [int]`
- [[Excisor's Gambit]] `[a], int → [a], [a], a`
- [[Refinement Distillation]] `[a], [a → bool] → [a]`

## Sources

- 📖 **Guidebook:** [[Advanced Lists]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexic:rotate`
