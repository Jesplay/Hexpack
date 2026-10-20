---
aliases: ["Entry Purification"]
tags: [hexcasting/pattern, mod/hexthings, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "HexThings"
op_id: "hexthings:dict_ents"
signature: "dqwaeawqd"
start_direction: "SOUTH_WEST"
input: "dict"
output: "list"
cost: ""
iotas_in: ["[[Dictionary]]"]
iotas_out: ["[[List]]"]
entry: "[[Dictionary Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Entry Purification

◇ **Pattern** · [[HexThings]] · 📖 [[Dictionary Manipulation]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="143.0,27.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="143.0,27.0 123.6,60.5 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="27.0" x2="136.8" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,45.4 133.1,35.6 140.5,39.9" fill="#fc77be"/><circle cx="143.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqwaeawqd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `dict` ([[Dictionary]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexthings:dict_ents`

```text
before   … │ dict  ◂ top
after    … │ list  ◂ top
```

## Description

Pushes a list of the key-value pairs in this Dictionary, each being a 2 element list.

## Related patterns (same book page)

- [[Impasse Reflection]] `— → dict`
- [[Accessor's Purification]] `dict → list`
- [[Accessed's Purification]] `dict → list`

## Sources

- 📖 **Guidebook:** [[Dictionary Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexthings:dict_ents`
