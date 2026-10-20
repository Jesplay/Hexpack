---
aliases: ["Moses's Gambit"]
tags: [hexcasting/pattern, mod/hexic, family/gambit, topic/lists]
kind: "Pattern"
family: "Gambit"
topics: ["Lists"]
mod: "Hexic"
op_id: "hexic:connect"
signature: "qaeaqeeaqwaqa"
start_direction: "NORTH_WEST"
input: "[a], (a, a → bool)"
output: "[ [a] ]"
cost: ""
iotas_in: ["[[Boolean]]", "[[List]]", "[[Any Iota]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Advanced Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Moses's Gambit

◇ **Pattern** · [[Hexic]] · 📖 [[Advanced Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="76.3,84.4 59.0,54.3 24.3,54.3 41.6,84.4 24.3,114.4 59.0,114.4 76.3,84.4 111.0,84.4 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 93.7,54.3 111.0,84.4 128.4,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:486;--dur:3.9s" points="76.3,84.4 59.0,54.3 24.3,54.3 41.6,84.4 24.3,114.4 59.0,114.4 76.3,84.4 111.0,84.4 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 93.7,54.3 111.0,84.4 128.4,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="84.4" x2="70.8" y2="74.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="66.8,67.9 74.1,72.9 67.5,76.7" fill="#fc77be"/><circle cx="76.3" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqeeaqwaqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[a], (a, a → bool)` ([[Boolean]], [[List]], [[Any Iota]])
- **Leaves:** `[[a]]` ([[List]], [[Any Iota]])
- **ID:** `hexic:connect`

```text
before   … │ [a] │ (a, a → bool)  ◂ top
after    … │ [[a]]                ◂ top
```

## Description

Casts the supplied **Hex** between each pair of elements in the given list, and splits the list at locations that return False.

## Related patterns (same book page)

- [[Ferris Distillation]] `[a], int → [a]`
- [[Retention Distillation]] `[a], int → [a]`
- [[Rejection Distillation]] `[a], int → [a]`
- [[Deductive Purification]] `[bool] → [int]`
- [[Excisor's Gambit]] `[a], int → [a], [a], a`
- [[Refinement Distillation]] `[a], [a → bool] → [a]`

## Sources

- 📖 **Guidebook:** [[Advanced Lists]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexic:connect`
