---
aliases: ["Region Query"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_query"
signature: "eeea"
start_direction: "EAST"
input: "region, vector"
output: "boolean"
cost: ""
iotas_in: ["[[Vector]]", "[[Region]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Shaping Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Region Query

◇ **Pattern** · [[Hexwright]] · 📖 [[Shaping Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="29.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="29.8" x2="43.4" y2="29.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="53.2,29.8 43.4,34.4 43.4,25.1" fill="#fc77be"/><circle cx="29.8" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region, vector` ([[Vector]], [[Region]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexwright:region_query`

```text
before   … │ region │ vector  ◂ top
after    … │ boolean          ◂ top
```

## Description

Queries if the vector exists in the region and returns a bool. Does not need ambit.

## Related patterns (same book page)

- [[Region Union]] `region, region → region`
- [[Region Intersection]] `region, region → region`
- [[Region Difference]] `region, region → region`
- [[Region Cut]] `region, vector, vector → region, region`

## Sources

- 📖 **Guidebook:** [[Shaping Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RegionActions$8` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:region_query`
