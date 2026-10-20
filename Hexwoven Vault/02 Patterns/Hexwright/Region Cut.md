---
aliases: ["Region Cut"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_cut"
signature: "eeeeedqwawqwa"
start_direction: "NORTH_EAST"
input: "region, vector, vector"
output: "region, region"
cost: ""
iotas_in: ["[[Vector]]", "[[Region]]"]
iotas_out: ["[[Region]]"]
entry: "[[Shaping Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Region Cut

◇ **Pattern** · [[Hexwright]] · 📖 [[Shaping Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 85.0,60.5 123.6,60.5 143.0,94.0 123.6,127.4 85.0,127.4 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 46.4,60.5 27.0,94.0 65.7,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="65.7,94.0 85.0,60.5 123.6,60.5 143.0,94.0 123.6,127.4 85.0,127.4 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 46.4,60.5 27.0,94.0 65.7,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="71.9" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,75.6 75.5,85.4 68.2,81.1" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeedqwawqwa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region, vector, vector` ([[Vector]], [[Region]])
- **Leaves:** `region, region` ([[Region]])
- **ID:** `hexwright:region_cut`

```text
before   … │ region │ vector │ vector  ◂ top
after    … │ region │ region           ◂ top
```

## Description

Splits a region along the plane through a point with a normal. Produces the positive and negative sides.

## Related patterns (same book page)

- [[Region Union]] `region, region → region`
- [[Region Intersection]] `region, region → region`
- [[Region Difference]] `region, region → region`
- [[Region Query]] `region, vector → boolean`

## Sources

- 📖 **Guidebook:** [[Shaping Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RegionActions$7` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:region_cut`
