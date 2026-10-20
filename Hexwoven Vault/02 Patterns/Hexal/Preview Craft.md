---
aliases: ["Preview Craft"]
tags: [hexcasting/pattern, mod/hexal, family/other, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/craft/preview"
signature: "wwdwaqawdwaqwwawdedwawwqawdwaqawdww"
start_direction: "NORTH_EAST"
input: "item | [item] | [ [item] ]"
output: "[item]"
cost: ""
iotas_in: ["[[List]]", "[[Item Stack]]"]
iotas_out: ["[[List]]", "[[Item Stack]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Preview Craft

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="113" height="170" viewBox="0 0 113.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.2" cy="18.0" r="2"/><circle cx="30.9" cy="18.0" r="2"/><circle cx="56.7" cy="18.0" r="2"/><circle cx="82.5" cy="18.0" r="2"/><circle cx="108.2" cy="18.0" r="2"/><circle cx="18.0" cy="40.4" r="2"/><circle cx="43.8" cy="40.4" r="2"/><circle cx="69.6" cy="40.4" r="2"/><circle cx="95.4" cy="40.4" r="2"/><circle cx="5.2" cy="62.7" r="2"/><circle cx="30.9" cy="62.7" r="2"/><circle cx="56.7" cy="62.7" r="2"/><circle cx="82.5" cy="62.7" r="2"/><circle cx="108.2" cy="62.7" r="2"/><circle cx="18.0" cy="85.0" r="2"/><circle cx="43.8" cy="85.0" r="2"/><circle cx="69.6" cy="85.0" r="2"/><circle cx="95.4" cy="85.0" r="2"/><circle cx="5.2" cy="107.3" r="2"/><circle cx="30.9" cy="107.3" r="2"/><circle cx="56.7" cy="107.3" r="2"/><circle cx="82.5" cy="107.3" r="2"/><circle cx="108.2" cy="107.3" r="2"/><circle cx="18.0" cy="129.6" r="2"/><circle cx="43.8" cy="129.6" r="2"/><circle cx="69.6" cy="129.6" r="2"/><circle cx="95.4" cy="129.6" r="2"/><circle cx="5.2" cy="152.0" r="2"/><circle cx="30.9" cy="152.0" r="2"/><circle cx="56.7" cy="152.0" r="2"/><circle cx="82.5" cy="152.0" r="2"/><circle cx="108.2" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="18.0,85.0 30.9,62.7 43.8,40.4 56.7,18.0 69.6,40.4 82.5,62.7 95.4,40.4 82.5,18.0 69.6,40.4 56.7,62.7 43.8,40.4 30.9,18.0 18.0,40.4 30.9,62.7 43.8,85.0 56.7,107.3 69.6,85.0 82.5,62.7 95.4,85.0 82.5,107.3 69.6,85.0 56.7,62.7 43.8,85.0 30.9,107.3 18.0,129.6 30.9,152.0 43.8,129.6 56.7,107.3 69.6,129.6 82.5,152.0 95.4,129.6 82.5,107.3 69.6,129.6 56.7,152.0 43.8,129.6 30.9,107.3 18.0,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:928;--dur:6.0s" points="18.0,85.0 30.9,62.7 43.8,40.4 56.7,18.0 69.6,40.4 82.5,62.7 95.4,40.4 82.5,18.0 69.6,40.4 56.7,62.7 43.8,40.4 30.9,18.0 18.0,40.4 30.9,62.7 43.8,85.0 56.7,107.3 69.6,85.0 82.5,62.7 95.4,85.0 82.5,107.3 69.6,85.0 56.7,62.7 43.8,85.0 30.9,107.3 18.0,129.6 30.9,152.0 43.8,129.6 56.7,107.3 69.6,129.6 82.5,152.0 95.4,129.6 82.5,107.3 69.6,129.6 56.7,152.0 43.8,129.6 30.9,107.3 18.0,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.0" y1="85.0" x2="22.2" y2="77.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="25.1,72.7 24.6,79.3 19.7,76.4" fill="#fc77be"/><circle cx="18.0" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwdwaqawdwaqwwawdedwawwqawdwaqawdww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item | [item] | [[item]]` ([[List]], [[Item Stack]])
- **Leaves:** `[item]` ([[List]], [[Item Stack]])
- **ID:** `hexal:mote/craft/preview`

## Description

Takes the same input as [[Craft]], except it can take Item [[The Stack|Stack]] [[Iotas|Iotas]] as well as motes. Returns a list of Item Stack Iotas representing what the results of [[Craft]] would have been with motes containing those inputs.

## Related patterns (same book page)

- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
- [[Stocktake Reflection]] `— → [itemtype]`
- [[Stocktake Purification]] `mote | itemtype → [mote]`
- [[Capacity Reflection]] `— → int`
- [[Containment Distillation]] `vec, (itemtype | mote) → bool`
- [[Mediafy Item]] `item | item, mote → mote`
- [[Return Item]] `mote, vec | mote, vec, num → —`
- [[Stacking Distillation II]] `mote, (mote | item) → bool`
- [[Splitting Gambit]] `mote, int → mote, mote`
- [[Depot Purification]] `mote → vec`
- [[Depot Gambit]] `mote, vec → mote`
- [[Craft]] `mote | [mote] | [[mote]] → [mote]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Preview Craft](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/craft/preview)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCraftMotePreview` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/craft/preview`
