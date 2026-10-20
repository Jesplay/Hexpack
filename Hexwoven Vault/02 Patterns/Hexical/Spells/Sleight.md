---
aliases: ["Sleight"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:sleight"
signature: "aaqqadeeeq"
start_direction: "WEST"
input: "item entity/vector"
output: ""
cost: "Costs about a fourth of one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Wristpocket (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sleight

✦ **Spell** · [[Hexical]] · 📖 [[Wristpocket (Book Entry)]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="111.8,115.3 76.9,115.3 94.4,145.5 111.8,115.3 94.4,85.0 59.4,85.0 76.9,115.3 41.9,115.3 24.5,85.0 41.9,54.7 76.9,54.7 94.4,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="111.8,115.3 76.9,115.3 94.4,145.5 111.8,115.3 94.4,85.0 59.4,85.0 76.9,115.3 41.9,115.3 24.5,85.0 41.9,54.7 76.9,54.7 94.4,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.8" y1="115.3" x2="100.7" y2="115.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="92.6,115.3 100.7,111.4 100.7,119.1" fill="#fc77be"/><circle cx="111.8" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqqadeeeq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity/vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a fourth of one Amethyst Dust.
- **ID:** `hexical:sleight`

```text
before   … │ item entity/vector  ◂ top
after    …                       ◂ top
```

## Description

Performs an act of magical sleight of hand to steal items into my wristpocket or expel my wristpocket back into the world. Costs about a fourth of one [[Amethyst|Amethyst Dust]].

If given a vector, it conjures my wristpocketed item at that vector. If given an item entity, my wristpocket either pulls in or swaps with the item, depending on whether it is empty.

*If your sleight of hand causes you to break eye contact with your audience, it is too advanced for your skill level.*

## Related patterns (same book page)

- [[Wristpocket]] `— → —`
- [[Wristpocket Reflection]] `— → item`
- [[Mage Hand]] `entity/vector → —`
- [[Mage Mouth]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wristpocket (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sleight](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/wristpocket@hexical:sleight)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSleight` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:sleight`
