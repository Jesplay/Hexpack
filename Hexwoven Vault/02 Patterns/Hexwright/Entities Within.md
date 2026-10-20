---
aliases: ["Entities Within"]
tags: [hexcasting/pattern, mod/hexwright, family/other, topic/hexwright, topic/blocks]
kind: "Pattern"
family: "Other"
topics: ["Hex Wright Magic", "World: Blocks, Terrain & Weather"]
mod: "Hexwright"
op_id: "hexwright:region_entities"
signature: "eeeawede"
start_direction: "EAST"
input: "region"
output: "[entities]"
cost: ""
iotas_in: ["[[Region]]"]
iotas_out: ["[[Entity]]", "[[List]]"]
entry: "[[Executing Regions]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Entities Within

◇ **Pattern** · [[Hexwright]] · 📖 [[Executing Regions]] · Hex Wright › Bounded Geometry

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="86" height="170" viewBox="0 0 86.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.8" cy="20.8" r="2"/><circle cx="50.4" cy="20.8" r="2"/><circle cx="80.1" cy="20.8" r="2"/><circle cx="5.9" cy="46.5" r="2"/><circle cx="35.6" cy="46.5" r="2"/><circle cx="65.3" cy="46.5" r="2"/><circle cx="20.8" cy="72.2" r="2"/><circle cx="50.4" cy="72.2" r="2"/><circle cx="80.1" cy="72.2" r="2"/><circle cx="5.9" cy="97.8" r="2"/><circle cx="35.6" cy="97.8" r="2"/><circle cx="65.3" cy="97.8" r="2"/><circle cx="20.8" cy="123.5" r="2"/><circle cx="50.4" cy="123.5" r="2"/><circle cx="80.1" cy="123.5" r="2"/><circle cx="5.9" cy="149.2" r="2"/><circle cx="35.6" cy="149.2" r="2"/><circle cx="65.3" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="20.8,20.8 50.4,20.8 65.3,46.5 50.4,72.2 20.8,72.2 35.6,97.8 50.4,123.5 35.6,149.2 20.8,123.5 35.6,97.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:2.5s" points="20.8,20.8 50.4,20.8 65.3,46.5 50.4,72.2 20.8,72.2 35.6,97.8 50.4,123.5 35.6,149.2 20.8,123.5 35.6,97.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.8" y1="20.8" x2="30.3" y2="20.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.1,20.8 30.3,24.0 30.3,17.5" fill="#fc77be"/><circle cx="20.8" cy="20.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeawede` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region` ([[Region]])
- **Leaves:** `[entities]` ([[Entity]], [[List]])
- **ID:** `hexwright:region_entities`

```text
before   … │ region      ◂ top
after    … │ [entities]  ◂ top
```

## Description

Returns a list of all entities within the specified region, ordered from centre out.

## Related patterns (same book page)

- [[Reach Purification]] `region → boolean`
- [[Region Thoth]] `region, [pattern] → list`
- [[Region Fill]] `region → —`
- [[Region Break]] `region → —`

## Sources

- 📖 **Guidebook:** [[Executing Regions]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RegionActions$10` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:region_entities`
