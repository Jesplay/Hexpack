---
aliases: ["Craft Trinket"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hex Casting"
op_id: "hexcasting:craft/trinket"
signature: "wwaqqqqqeaqeaeqqqeaeq"
start_direction: "EAST"
input: "entity, list"
output: ""
cost: "Costs about five Charged Amethysts."
iotas_in: ["[[Entity]]", "[[List]]"]
iotas_out: []
entry: "[[Crafting Casting Items]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Craft Trinket

✦ **Spell** · [[Hex Casting]] · 📖 [[Crafting Casting Items]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="22.0,76.6 53.5,76.6 85.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 148.0,76.6 132.2,49.3 100.7,49.3 85.0,22.0 69.3,49.3 37.8,49.3 22.0,76.6 37.8,103.8 69.3,103.8 85.0,131.1 100.7,103.8 132.2,103.8 148.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="22.0,76.6 53.5,76.6 85.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 148.0,76.6 132.2,49.3 100.7,49.3 85.0,22.0 69.3,49.3 37.8,49.3 22.0,76.6 37.8,103.8 69.3,103.8 85.0,131.1 100.7,103.8 132.2,103.8 148.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="76.6" x2="32.1" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.4,76.6 32.1,80.0 32.1,73.1" fill="#fc77be"/><circle cx="22.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwaqqqqqeaqeaeqqqeaeq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, list` ([[Entity]], [[List]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about five Charged Amethysts.
- **ID:** `hexcasting:craft/trinket`

```text
before   … │ entity │ list  ◂ top
after    …                  ◂ top
```

## Description

Costs about five [[Amethyst|Charged Amethysts]].

## Related patterns (same book page)

- [[Craft Cypher]] `entity, list → —`
- [[Craft Artifact]] `entity, list → —`
- [[Recharge Item]] `entity → —`
- [[Erase Item]] `— → —`

## Sources

- 📖 **Guidebook:** [[Crafting Casting Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Craft Trinket](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/hexcasting@hexcasting:craft/trinket)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpMakePackagedSpell` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:craft/trinket`
