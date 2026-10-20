---
aliases: ["Bind Storage"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexal"
op_id: "hexal:mote/storage/bind"
signature: "qaqwqaqwqaq"
start_direction: "NORTH_WEST"
input: "vec"
output: ""
cost: "Costs 32 Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Motes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Bind Storage

✦ **Spell** · [[Hexal]] · 📖 [[Motes]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0 104.3,27.0 85.0,60.5 65.7,94.0 85.0,127.4 104.3,94.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="85.0,60.5 65.7,27.0 27.0,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0 104.3,27.0 85.0,60.5 65.7,94.0 85.0,127.4 104.3,94.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="60.5" x2="78.8" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,42.1 82.5,47.7 75.1,51.9" fill="#fc77be"/><circle cx="85.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwqaqwqaq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 32 Amethyst Dust.
- **ID:** `hexal:mote/storage/bind`

```text
before   … │ vec  ◂ top
after    …        ◂ top
```

## Description

Target a Mote Nexus block in the world and bind yourself to it, meaning all items you mediafy will be added to that nexus' capacity. Targeting any other block will unbind you from your currently bound nexus. Costs 32 [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

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
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

## Sources

- 📖 **Guidebook:** [[Motes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Bind Storage](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/motes@hexal:mote/storage/bind)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpBindStorage` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:mote/storage/bind`
