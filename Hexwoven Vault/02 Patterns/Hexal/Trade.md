---
aliases: ["Trade"]
tags: [hexcasting/pattern, mod/hexal, family/other, topic/items]
kind: "Pattern"
family: "Other"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/trade"
signature: "awdedwaeqded"
start_direction: "NORTH_WEST"
input: "villager, [mote], int?"
output: "item"
cost: "Costs a tenth of an Amethyst Dust."
iotas_in: ["[[Number]]", "[[Entity]]", "[[List]]", "[[Mote]]"]
iotas_out: ["[[Item Stack]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Trade

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 69.3,22.0 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 53.5,49.3 69.3,76.6 85.0,49.3 116.5,49.3 132.2,22.0 148.0,49.3 132.2,76.6 116.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:409;--dur:3.6s" points="85.0,49.3 69.3,22.0 53.5,49.3 37.8,76.6 22.0,49.3 37.8,22.0 53.5,49.3 69.3,76.6 85.0,49.3 116.5,49.3 132.2,22.0 148.0,49.3 132.2,76.6 116.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="80.0" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,34.3 83.0,38.8 77.0,42.3" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awdedwaeqded` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `villager, [mote], int?` ([[Number]], [[Entity]], [[List]], [[Mote]])
- **Leaves:** `item` ([[Item Stack]])
- **Cost:** Costs a tenth of an Amethyst Dust.
- **ID:** `hexal:mote/trade`

```text
before   … │ villager │ [mote] │ int?  ◂ top
after    … │ item                      ◂ top
```

## Description

Accepts a villager, a list of motes to trade with to villager, and optionally an int for which of that villager's trades to attempt. (int? above means that you can pass 2 or 3 arguments, the optional 3rd being an int).

If no trade number was passed, it will try each of the villager's trade until finding one that accepts the passed items. If a trade number was passed it will skip straight to using that trade. It will then repeat that trade for as long as it has enough items and the villager is still willing to trade. It will then return the items the villager traded away as a mote to the [[The Stack|stack]].

The villager will find this trade to be a learnable experience as it would if trading with you directly, and will improve its skills and trade offerings commensurately. Costs a tenth of an [[Amethyst|Amethyst Dust]].

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
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Trade](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/trade)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTradeMote` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/trade`
