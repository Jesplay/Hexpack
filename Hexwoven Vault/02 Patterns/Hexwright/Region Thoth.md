---
aliases: ["Region Thoth"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_thoth"
signature: "eeeadeaqq"
start_direction: "EAST"
input: "region, [pattern]"
output: "list"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]", "[[Region]]"]
iotas_out: ["[[List]]"]
entry: "[[Executing Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Region Thoth

◇ **Pattern** · [[Hexwright]] · 📖 [[Executing Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,24.5 111.8,24.5 129.3,54.7 111.8,85.0 76.9,85.0 94.4,115.3 59.4,115.3 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="76.9,24.5 111.8,24.5 129.3,54.7 111.8,85.0 76.9,85.0 94.4,115.3 59.4,115.3 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="24.5" x2="88.1" y2="24.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.1,24.5 88.1,28.3 88.1,20.6" fill="#fc77be"/><circle cx="76.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeadeaqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region, [pattern]` ([[List]], [[Pattern]], [[Region]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexwright:region_thoth`

```text
before   … │ region │ [pattern]  ◂ top
after    … │ list                ◂ top
```

## Description

Runs a pattern over the region exactly as a thoth would, seeds the [[The Stack|stack]] with each vector in a region.
Not a real thoth.

## Related patterns (same book page)

- [[Reach Purification]] `region → boolean`
- [[Entities Within]] `region → [entities]`
- [[Region Fill]] `region → —`
- [[Region Break]] `region → —`

## Sources

- 📖 **Guidebook:** [[Executing Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexwright:region_thoth`
