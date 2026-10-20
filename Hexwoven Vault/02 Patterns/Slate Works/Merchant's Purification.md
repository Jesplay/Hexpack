---
aliases: ["Merchant's Purification"]
tags: [hexcasting/pattern, mod/slate-works, family/purification, topic/circles, topic/items]
kind: "Pattern"
family: "Purification"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:get_trades"
signature: "eaqwqaewedeadwdwd"
start_direction: "SOUTH_WEST"
input: "vec"
output: "complicated"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Merchant Directrix Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Merchant's Purification

◇ **Pattern** · [[Slate Works]] · 📖 [[Merchant Directrix Patterns]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,85.0 59.4,115.3 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 94.4,115.3 76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0 111.8,85.0 94.4,115.3 76.9,145.5 59.4,115.3 41.9,85.0 76.9,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="76.9,85.0 59.4,115.3 24.5,115.3 41.9,145.5 76.9,145.5 111.8,145.5 129.3,115.3 94.4,115.3 76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0 111.8,85.0 94.4,115.3 76.9,145.5 59.4,115.3 41.9,85.0 76.9,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="85.0" x2="71.3" y2="94.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,101.6 68.0,92.8 74.6,96.6" fill="#fc77be"/><circle cx="76.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaewedeadwdwd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `complicated` (—)
- **ID:** `slate_work:get_trades`

## Description

Despite the return of this spell, its not extremely complicated... I think. This takes a [[Merchant Directrix]], and returns: \[\[**Item [[The Stack|Stack]]**, **Item Stack**, **Item Stack**, **Number**\]...\]. Free to cast.

Where the first [[Items|Item Stack]] is the item(s) the Villager will buy; second [[Items|Item Stack]] is another set of item(s) the Villager may want to buy; the 3rd [[Items|Item Stack]] is the item will the Villager sell to me; and the **Number** is ratio between the times the trade has been done (since last restock), and the max times it can be done. If the **Number** is at one, the trade can no longer be traded.

## Related patterns (same book page)

- [[Exchange Mind]] `vector, entity → —`
- [[Induce Restock]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Merchant Directrix Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetTrades` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:get_trades`
