---
aliases: ["Recharge Item"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hex Casting"
op_id: "hexcasting:recharge"
signature: "qqqqqwaeaeaeaeaea"
start_direction: "NORTH_WEST"
input: "entity"
output: ""
cost: "Costs about one Amethyst Shard per item."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Crafting Casting Items]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Recharge Item

✦ **Spell** · [[Hex Casting]] · 📖 [[Crafting Casting Items]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.8" y1="85.0" x2="106.2" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.2,68.4 109.6,73.4 102.9,77.2" fill="#fc77be"/><circle cx="111.8" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwaeaeaeaeaea` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Shard per item.
- **ID:** `hexcasting:recharge`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Recharge a **[[Media|media]]**-containing item in my other hand. Costs about one [[Amethyst|Amethyst Shard]] per item.

This spell is cast in a similar method to the crafting spells; an entity representing a dropped [[The Stack|stack]] of [[Amethyst]] is provided, and recharges the **media** battery of the item in my other hand.

This spell cannot recharge the item farther than its original battery size.

## Related patterns (same book page)

- [[Craft Cypher]] `entity, list → —`
- [[Craft Trinket]] `entity, list → —`
- [[Craft Artifact]] `entity, list → —`
- [[Erase Item]] `— → —`

## Sources

- 📖 **Guidebook:** [[Crafting Casting Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Recharge Item](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/hexcasting@hexcasting:recharge)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpRecharge` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:recharge`
