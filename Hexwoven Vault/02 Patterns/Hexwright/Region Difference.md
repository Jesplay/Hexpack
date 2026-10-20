---
aliases: ["Region Difference"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_difference"
signature: "eewqqqqqweedqqdwdqq"
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

# Region Difference

◇ **Pattern** · [[Hexwright]] · 📖 [[Shaping Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 37.8,49.3 69.3,49.3 85.0,76.6 100.7,103.8 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3 85.0,76.6 69.3,103.8 37.8,103.8 22.0,76.6 53.5,76.6 69.3,49.3 53.5,22.0 85.0,22.0 116.5,22.0 100.7,49.3 116.5,76.6 148.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:630;--dur:5.6s" points="22.0,76.6 37.8,49.3 69.3,49.3 85.0,76.6 100.7,103.8 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3 85.0,76.6 69.3,103.8 37.8,103.8 22.0,76.6 53.5,76.6 69.3,49.3 53.5,22.0 85.0,22.0 116.5,22.0 100.7,49.3 116.5,76.6 148.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="27.1" y2="67.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,61.6 30.1,69.6 24.1,66.1" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eewqqqqqweedqqdwdqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region, region` ([[Region]])
- **Leaves:** `region` ([[Region]])
- **ID:** `hexwright:region_difference`

```text
before   … │ region │ region  ◂ top
after    … │ region           ◂ top
```

## Description

Returns the first region with any blocks overlapping with region two removed.

## Related patterns (same book page)

- [[Region Union]] `region, region → region`
- [[Region Intersection]] `region, region → region`
- [[Region Cut]] `region, vector, vector → region, region`
- [[Region Query]] `region, vector → boolean`

## Sources

- 📖 **Guidebook:** [[Shaping Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RegionActions$6` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:region_difference`
