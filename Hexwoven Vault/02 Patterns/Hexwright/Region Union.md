---
aliases: ["Region Union"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_union"
signature: "eewqqqqqweedqeeq"
start_direction: "NORTH_EAST"
input: "region, region"
output: "region"
cost: ""
iotas_in: ["[[Region]]"]
iotas_out: ["[[Region]]"]
entry: "[[Shaping Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Region Union

◇ **Pattern** · [[Hexwright]] · 📖 [[Shaping Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="22.0,49.3 37.8,22.0 69.3,22.0 85.0,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3 69.3,76.6 37.8,76.6 22.0,49.3 53.5,49.3 69.3,22.0 100.7,22.0 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:535;--dur:4.8s" points="22.0,49.3 37.8,22.0 69.3,22.0 85.0,49.3 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3 69.3,76.6 37.8,76.6 22.0,49.3 53.5,49.3 69.3,22.0 100.7,22.0 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="49.3" x2="27.1" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,34.3 30.1,42.3 24.1,38.8" fill="#fc77be"/><circle cx="22.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eewqqqqqweedqeeq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region, region` ([[Region]])
- **Leaves:** `region` ([[Region]])
- **ID:** `hexwright:region_union`

```text
before   … │ region │ region  ◂ top
after    … │ region           ◂ top
```

## Description

Combines both regions into one.

## Related patterns (same book page)

- [[Region Intersection]] `region, region → region`
- [[Region Difference]] `region, region → region`
- [[Region Cut]] `region, vector, vector → region, region`
- [[Region Query]] `region, vector → boolean`

## Sources

- 📖 **Guidebook:** [[Shaping Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RegionActions$4` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:region_union`
