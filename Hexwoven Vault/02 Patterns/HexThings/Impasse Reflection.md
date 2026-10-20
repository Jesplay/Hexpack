---
aliases: ["Impasse Reflection"]
tags: [hexcasting/pattern, mod/hexthings, family/reflection, topic/data-structures]
kind: "Pattern"
family: "Reflection"
topics: ["Sets, Maps & Special Iotas"]
mod: "HexThings"
op_id: "hexthings:empty_dict"
signature: "eaaea"
start_direction: "NORTH_WEST"
input: ""
output: "dict"
cost: ""
iotas_in: []
iotas_out: ["[[Dictionary]]"]
entry: "[[Dictionary Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Impasse Reflection

◇ **Pattern** · [[HexThings]] · 📖 [[Dictionary Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="92.3,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 38.0,132.0 92.3,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="132.0" x2="83.6" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="77.3,106.2 88.8,114.0 78.4,119.9" fill="#fc77be"/><circle cx="92.3" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaaea` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `dict` ([[Dictionary]])
- **ID:** `hexthings:empty_dict`

```text
before   …         ◂ top
after    … │ dict  ◂ top
```

## Description

Creates an empty Dictionary [[Iotas|iota]].

## Related patterns (same book page)

- [[Accessor's Purification]] `dict → list`
- [[Accessed's Purification]] `dict → list`
- [[Entry Purification]] `dict → list`

## Sources

- 📖 **Guidebook:** [[Dictionary Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEmptyDict` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexthings:empty_dict`
