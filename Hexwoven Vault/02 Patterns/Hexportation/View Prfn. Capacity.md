---
aliases: ["View Prfn. : Capacity"]
tags: [hexcasting/pattern, mod/hexportation, family/purification, topic/devices, topic/storage]
kind: "Pattern"
family: "Purification"
topics: ["Casting Devices", "Reading & Writing"]
mod: "Hexportation"
op_id: "hexportation:get_view_capacity"
signature: "adeeedeww"
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

# View Prfn. : Capacity

◇ **Pattern** · [[Hexportation]] · 📖 [[Conduits]] · Patterns › Great Spells

**Topics:** [[Casting Devices (Topic)|Casting Devices]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="22.0,49.3 53.5,49.3 37.8,22.0 69.3,22.0 85.0,49.3 69.3,76.6 37.8,76.6 53.5,49.3 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:2.8s" points="22.0,49.3 53.5,49.3 37.8,22.0 69.3,22.0 85.0,49.3 69.3,76.6 37.8,76.6 53.5,49.3 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="49.3" x2="32.1" y2="49.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.4,49.3 32.1,52.8 32.1,45.8" fill="#fc77be"/><circle cx="22.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adeeedeww` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `view` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexportation:get_view_capacity`

```text
before   … │ view  ◂ top
after    … │ num   ◂ top
```

## Description

Returns the capacity of the resources in the specified view

It should be noted that whilst a view [[Iotas|iota]] that represents a fluid will display its amount/capacity in millibuckets for ease of comparison, the patterns to get each value will return the amount in droplets.
 Conversions between fluid units can be found elsewhere, but for simplicities sake, 1 Bucket = 81000 Droplets

## Related patterns (same book page)

- [[Create Conduit]] `vec, vec, vec, vec → conduit`
- [[Send]] `conduit, num → —`
- [[Source Data]] `conduit → [view, ...]`
- [[Sink Data]] `conduit → [view, ...]`
- [[Metered Send]] `conduit, [num, ...] → —`
- [[View Prfn. Amount]] `view → num`
- [[View Prfn. Name]] `view → string`
- [[Slurp]] `conduit → —`
- [[Spit]] `conduit → —`
- [[Transplace]] `conduit → —`

## Sources

- 📖 **Guidebook:** [[Conduits]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexportation on Modrinth](https://modrinth.com/mod/AsqM28gQ)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetViewData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexportation:get_view_capacity`
