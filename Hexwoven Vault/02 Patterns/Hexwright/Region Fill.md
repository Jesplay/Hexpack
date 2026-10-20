---
aliases: ["Region Fill"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_fill"
signature: "eeeeedeqqqwqwqq"
start_direction: "SOUTH_WEST"
input: "region"
output: ""
cost: ""
iotas_in: ["[[Region]]"]
iotas_out: []
entry: "[[Executing Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Region Fill

◇ **Pattern** · [[Hexwright]] · 📖 [[Executing Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="128.4,54.3 111.0,84.4 76.3,84.4 59.0,54.3 76.3,24.3 111.0,24.3 128.4,54.3 93.7,54.3 76.3,24.3 41.6,24.3 24.3,54.3 41.6,84.4 59.0,114.4 93.7,114.4 128.4,114.4 145.7,84.4 128.4,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:555;--dur:4.5s" points="128.4,54.3 111.0,84.4 76.3,84.4 59.0,54.3 76.3,24.3 111.0,24.3 128.4,54.3 93.7,54.3 76.3,24.3 41.6,24.3 24.3,54.3 41.6,84.4 59.0,114.4 93.7,114.4 128.4,114.4 145.7,84.4 128.4,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="54.3" x2="122.8" y2="63.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="118.8,70.9 119.5,62.0 126.1,65.9" fill="#fc77be"/><circle cx="128.4" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeedeqqqwqwqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region` ([[Region]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:region_fill`

```text
before   … │ region  ◂ top
after    …           ◂ top
```

## Description

Fills every replaceable block in the region and operates the same as place block.

## Related patterns (same book page)

- [[Reach Purification]] `region → boolean`
- [[Entities Within]] `region → [entities]`
- [[Region Thoth]] `region, [pattern] → list`
- [[Region Break]] `region → —`

## Sources

- 📖 **Guidebook:** [[Executing Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexwright:region_fill`
