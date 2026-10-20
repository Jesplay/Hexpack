---
aliases: ["Hashing Purification"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "Ephemera"
op_id: "ephemera:hash"
signature: "qqawqaqw"
start_direction: "SOUTH_EAST"
input: "any"
output: "hash"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Iota Hashing]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hashing Purification

◇ **Pattern** · [[Ephemera]] · 📖 [[Iota Hashing]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="52.3,68.3 74.1,106.0 117.7,106.0 139.5,68.3 95.9,68.3 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 117.7,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:392;--dur:2.5s" points="52.3,68.3 74.1,106.0 117.7,106.0 139.5,68.3 95.9,68.3 52.3,68.3 30.5,106.0 74.1,106.0 95.9,68.3 117.7,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.3" y1="68.3" x2="59.3" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="64.3,89.0 55.1,82.7 63.4,77.9" fill="#fc77be"/><circle cx="52.3" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqawqaqw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `hash` (—)
- **ID:** `ephemera:hash`

```text
before   … │ any   ◂ top
after    … │ hash  ◂ top
```

## Description

Accepts any [[Iotas|iota]], and converts it into its hashed form.

## Sources

- 📖 **Guidebook:** [[Iota Hashing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Hashing Purification](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/hashing@ephemera:hash)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpHash` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:hash`
