---
aliases: ["Offering Purification"]
tags: [hexcasting/pattern, mod/hexal, family/purification, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/trade/get"
signature: "awdedwaawwqded"
start_direction: "SOUTH_EAST"
input: "villager"
output: "[complicated!]"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[List]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Offering Purification

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="22.0,49.3 37.8,76.6 53.5,49.3 69.3,22.0 85.0,49.3 69.3,76.6 53.5,49.3 37.8,22.0 22.0,49.3 53.5,49.3 85.0,49.3 116.5,49.3 132.2,22.0 148.0,49.3 132.2,76.6 116.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="22.0,49.3 37.8,76.6 53.5,49.3 69.3,22.0 85.0,49.3 69.3,76.6 53.5,49.3 37.8,22.0 22.0,49.3 53.5,49.3 85.0,49.3 116.5,49.3 132.2,22.0 148.0,49.3 132.2,76.6 116.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="49.3" x2="27.1" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,64.3 24.1,59.8 30.1,56.3" fill="#fc77be"/><circle cx="22.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awdedwaawwqded` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `villager` ([[Entity]])
- **Leaves:** `[complicated!]` ([[List]])
- **ID:** `hexal:mote/trade/get`

## Description

Accepts a villager and returns all of the trades that villager is offering. This is a list of lists of lists. Each trade comprises two lists, the cost, and the returned items. The returned items is a list containing the type of item, and the number of items.

The cost is a list of one or two pairs of item and number of items required by the villager for the trade to be accepted. For example, one trade offer could be \[\[\[Emerald, 1\]\], \[Redstone, 1\]\].

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
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Offering Purification](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/trade/get)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetItemTrades` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/trade/get`
