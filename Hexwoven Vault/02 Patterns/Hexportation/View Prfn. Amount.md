---
aliases: ["View Prfn. : Amount"]
tags: [hexcasting/pattern, mod/hexportation, family/purification, topic/devices, topic/storage]
kind: "Pattern"
family: "Purification"
topics: ["Casting Devices", "Reading & Writing"]
mod: "Hexportation"
op_id: "hexportation:get_view_amount"
signature: "adeeedewq"
start_direction: "EAST"
input: "view"
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Conduits]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# View Prfn. : Amount

◇ **Pattern** · [[Hexportation]] · 📖 [[Conduits]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,54.3 59.0,54.3 41.6,24.3 76.3,24.3 93.7,54.3 76.3,84.4 41.6,84.4 59.0,54.3 93.7,54.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="24.3,54.3 59.0,54.3 41.6,24.3 76.3,24.3 93.7,54.3 76.3,84.4 41.6,84.4 59.0,54.3 93.7,54.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="54.3" x2="35.4" y2="54.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,54.3 35.4,58.1 35.4,50.5" fill="#fc77be"/><circle cx="24.3" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeeedewq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `view` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexportation:get_view_amount`

```text
before   … │ view  ◂ top
after    … │ num   ◂ top
```

## Description

Returns the amount of the resources in the specified view

## Related patterns (same book page)

- [[Create Conduit]] `vec, vec, vec, vec → conduit`
- [[Send]] `conduit, num → —`
- [[Source Data]] `conduit → [view, ...]`
- [[Sink Data]] `conduit → [view, ...]`
- [[Metered Send]] `conduit, [num, ...] → —`
- [[View Prfn. Capacity]] `view → num`
- [[View Prfn. Name]] `view → string`
- [[Slurp]] `conduit → —`
- [[Spit]] `conduit → —`
- [[Transplace]] `conduit → —`

## Sources

- 📖 **Guidebook:** [[Conduits]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexportation on Modrinth](https://modrinth.com/mod/AsqM28gQ)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetViewData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexportation:get_view_amount`
