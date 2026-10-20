---
aliases: ["Craft"]
tags: [hexcasting/pattern, mod/hexal, family/other, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/craft"
signature: "wwawdedwawdewwdwaqawdwwedwawdedwaww"
start_direction: "SOUTH_EAST"
input: "mote | [mote] | [ [mote] ]"
output: "[mote]"
cost: "Costs a tenth of an Amethyst Dust."
iotas_in: ["[[List]]", "[[Mote]]"]
iotas_out: ["[[List]]", "[[Mote]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Craft

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="113" height="170" viewBox="0 0 113.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.2" cy="18.0" r="2"/><circle cx="30.9" cy="18.0" r="2"/><circle cx="56.7" cy="18.0" r="2"/><circle cx="82.5" cy="18.0" r="2"/><circle cx="108.2" cy="18.0" r="2"/><circle cx="18.0" cy="40.4" r="2"/><circle cx="43.8" cy="40.4" r="2"/><circle cx="69.6" cy="40.4" r="2"/><circle cx="95.4" cy="40.4" r="2"/><circle cx="5.2" cy="62.7" r="2"/><circle cx="30.9" cy="62.7" r="2"/><circle cx="56.7" cy="62.7" r="2"/><circle cx="82.5" cy="62.7" r="2"/><circle cx="108.2" cy="62.7" r="2"/><circle cx="18.0" cy="85.0" r="2"/><circle cx="43.8" cy="85.0" r="2"/><circle cx="69.6" cy="85.0" r="2"/><circle cx="95.4" cy="85.0" r="2"/><circle cx="5.2" cy="107.3" r="2"/><circle cx="30.9" cy="107.3" r="2"/><circle cx="56.7" cy="107.3" r="2"/><circle cx="82.5" cy="107.3" r="2"/><circle cx="108.2" cy="107.3" r="2"/><circle cx="18.0" cy="129.6" r="2"/><circle cx="43.8" cy="129.6" r="2"/><circle cx="69.6" cy="129.6" r="2"/><circle cx="95.4" cy="129.6" r="2"/><circle cx="5.2" cy="152.0" r="2"/><circle cx="30.9" cy="152.0" r="2"/><circle cx="56.7" cy="152.0" r="2"/><circle cx="82.5" cy="152.0" r="2"/><circle cx="108.2" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="18.0,85.0 30.9,107.3 43.8,129.6 56.7,152.0 69.6,129.6 82.5,107.3 95.4,129.6 82.5,152.0 69.6,129.6 56.7,107.3 43.8,129.6 30.9,152.0 18.0,129.6 30.9,107.3 43.8,85.0 56.7,62.7 69.6,85.0 82.5,107.3 95.4,85.0 82.5,62.7 69.6,85.0 56.7,107.3 43.8,85.0 30.9,62.7 18.0,40.4 30.9,18.0 43.8,40.4 56.7,62.7 69.6,40.4 82.5,18.0 95.4,40.4 82.5,62.7 69.6,40.4 56.7,18.0 43.8,40.4 30.9,62.7 18.0,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:928;--dur:6.0s" points="18.0,85.0 30.9,107.3 43.8,129.6 56.7,152.0 69.6,129.6 82.5,107.3 95.4,129.6 82.5,152.0 69.6,129.6 56.7,107.3 43.8,129.6 30.9,152.0 18.0,129.6 30.9,107.3 43.8,85.0 56.7,62.7 69.6,85.0 82.5,107.3 95.4,85.0 82.5,62.7 69.6,85.0 56.7,107.3 43.8,85.0 30.9,62.7 18.0,40.4 30.9,18.0 43.8,40.4 56.7,62.7 69.6,40.4 82.5,18.0 95.4,40.4 82.5,62.7 69.6,40.4 56.7,18.0 43.8,40.4 30.9,62.7 18.0,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.0" y1="85.0" x2="22.2" y2="92.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="25.1,97.3 19.7,93.6 24.6,90.7" fill="#fc77be"/><circle cx="18.0" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwawdedwawdewwdwaqawdwwedwawdedwaww` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mote | [mote] | [[mote]]` ([[List]], [[Mote]])
- **Leaves:** `[mote]` ([[List]], [[Mote]])
- **Cost:** Costs a tenth of an Amethyst Dust.
- **ID:** `hexal:mote/craft`

## Description

Takes a mote, list of motes, or list of list of motes. returns a list containing the results of crafting with those items. For example, the input Stone x128 would return \[Stone Button x128\].

For more complex crafts, inserting nulls to space items properly will be required. A diamond pickaxe for example would require \[\[Diamond x2, Diamond x2, Diamond x2\], \[null, Stick x2\], \[null, Stick x2\]\] to get \[Diamond Pickaxe x2\]. Costs a tenth of an [[Amethyst|Amethyst Dust]].

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
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Craft](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/craft)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCraftMote` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/craft`
