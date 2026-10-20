---
aliases: ["Set Recipe"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:set_craft"
signature: "eaqwqaeqwaeadawwadaeaw"
start_direction: "SOUTH_WEST"
input: "vec, [item stack|item type|null]"
output: ""
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]", "[[List]]", "[[Null]]", "[[Item Type]]", "[[Item Stack]]"]
iotas_out: []
entry: "[[Patterned Assembler Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Set Recipe

◇ **Pattern** · [[Slate Works]] · 📖 [[Patterned Assembler Patterns]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="85.0,22.0 69.3,49.3 37.8,49.3 53.5,76.6 85.0,76.6 116.5,76.6 132.2,49.3 100.7,49.3 85.0,22.0 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 37.8,103.8 69.3,103.8 100.7,103.8 132.2,103.8 116.5,76.6 148.0,76.6 132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:724;--dur:6.0s" points="85.0,22.0 69.3,49.3 37.8,49.3 53.5,76.6 85.0,76.6 116.5,76.6 132.2,49.3 100.7,49.3 85.0,22.0 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 37.8,103.8 69.3,103.8 100.7,103.8 132.2,103.8 116.5,76.6 148.0,76.6 132.2,49.3 148.0,22.0 116.5,22.0 85.0,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="22.0" x2="80.0" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,37.0 77.0,29.0 83.0,32.5" fill="#fc77be"/><circle cx="85.0" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaeqwaeadawwadaeaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, [item stack|item type|null]` ([[Vector]], [[List]], [[Null]], [[Item Type]], [[Item Stack]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount of media.
- **ID:** `slate_work:set_craft`

## Description

Sets the crafting recipe of a [[Patterned Assemblers|Patterned Assembler]]. Takes a list of Nulls, items, or item variants, and applies it to the targeted [[Patterned Assemblers|Assembler]].

To note: the given list gets "formatted" as left to right, top to bottom in the [[Patterned Assemblers|Patterned Assembler]]. As well, the Nulls represent blank spots in recipes. Finally, the Spell will [[Mishaps|mishap]] if the given list is longer than 9 items (but passes with less than or equal to 9 items).

Does not mishap if cast outside of a [[Spell Circles|Spell Circle]]. Costs a negligible amount of **[[Media|media]]**.

## Related patterns (same book page)

- [[Set Craft Count]] `vec, number → —`
- [[Get Craft Count]] `vec → number`

## Sources

- 📖 **Guidebook:** [[Patterned Assembler Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetCraftingLoci` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:set_craft`
