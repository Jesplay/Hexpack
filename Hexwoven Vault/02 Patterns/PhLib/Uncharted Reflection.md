---
aliases: ["Uncharted Reflection"]
tags: [hexcasting/pattern, mod/phlib, family/reflection, topic/data-structures]
kind: "Pattern"
family: "Reflection"
topics: ["Sets, Maps & Special Iotas"]
mod: "PhLib"
op_id: "phlib:empty_map"
signature: "dqdwdqd"
start_direction: "EAST"
input: ""
output: "map<⊥, ⊥>"
cost: ""
iotas_in: []
iotas_out: ["[[Dictionary]]"]
entry: "[[Maps]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Uncharted Reflection

◇ **Pattern** · [[PhLib]] · 📖 [[Maps]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,35.0 135.0,35.0 110.0,78.3 135.0,121.6 85.0,121.6 35.0,121.6 60.0,78.3 35.0,35.0 85.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="85.0,35.0 135.0,35.0 110.0,78.3 135.0,121.6 85.0,121.6 35.0,121.6 60.0,78.3 35.0,35.0 85.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="35.0" x2="101.0" y2="35.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,35.0 101.0,40.5 101.0,29.5" fill="#fc77be"/><circle cx="85.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqdwdqd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `map<⊥, ⊥>` ([[Dictionary]])
- **ID:** `phlib:empty_map`

```text
before   …              ◂ top
after    … │ map<⊥, ⊥>  ◂ top
```

## Description

Creates a new empty map for my usage.

## Sources

- 📖 **Guidebook:** [[Maps]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `phlib:empty_map`
