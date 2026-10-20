---
aliases: ["Depot Gambit"]
tags: [hexcasting/pattern, mod/hexal, family/gambit, topic/items]
kind: "Pattern"
family: "Gambit"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/storage/set"
signature: "eeeeedw"
start_direction: "SOUTH_EAST"
input: "mote, vec"
output: "mote"
cost: ""
iotas_in: ["[[Vector]]", "[[Mote]]"]
iotas_out: ["[[Mote]]"]
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Depot Gambit

◇ **Pattern** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,35.0 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="110.0,35.0 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 85.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="35.0" x2="118.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="123.8,58.8 113.2,51.6 122.8,46.1" fill="#fc77be"/><circle cx="110.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeedw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mote, vec` ([[Vector]], [[Mote]])
- **Leaves:** `mote` ([[Mote]])
- **ID:** `hexal:mote/storage/set`

```text
before   … │ mote │ vec  ◂ top
after    … │ mote        ◂ top
```

## Description

Move the record referenced by the passed mote to the Mote Nexus at the passed position. This returns a new mote pointing to that record, and invalidates all previous motes pointing to that record.

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
- [[Craft]] `mote | [mote] | [[mote]] → [mote]`
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Depot Gambit](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/storage/set)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetMoteStorage` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/storage/set`
