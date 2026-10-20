---
aliases: ["Prestidigitation"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:prestidigitation"
signature: "wedewedew"
start_direction: "NORTH_EAST"
input: "entity/vector"
output: ""
cost: "Costs about a tenth of one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Prestidigitation (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Prestidigitation

✦ **Spell** · [[Hexical]] · 📖 [[Prestidigitation (Book Entry)]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 85.0,60.5 46.4,60.5 27.0,27.0 65.7,27.0 85.0,60.5 104.3,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="65.7,94.0 85.0,60.5 104.3,27.0 143.0,27.0 123.6,60.5 85.0,60.5 46.4,60.5 27.0,27.0 65.7,27.0 85.0,60.5 104.3,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="71.9" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,75.6 75.5,85.4 68.2,81.1" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wedewedew` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity/vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a tenth of one Amethyst Dust.
- **ID:** `hexical:prestidigitation`

```text
before   … │ entity/vector  ◂ top
after    …                  ◂ top
```

## Description

Causes a small magical effect, not too distinct from the original nature or function of the target. Costs about a tenth of one [[Amethyst|Amethyst Dust]].

I've compiled all of its recorded uses but there is no guarantee all have been found: opens and closes fence gates, trapdoors, and doors of any materials; flicks levers and presses buttons; holds down and releases pressure plates; extinguishes fires of all kinds; lights and extinguishes candles, candle cakes, and campfires; ignites soul fire on blocks that support it; rings bells; strips wood; carves pumpkins; makes a note block play its sound; dries mud into clay; toggles Redstone lamps;

triggers dispensers and droppers; turns most soil blocks into path blocks and path blocks into tilled soil; drain cauldrons; modifies the state of Redstone repeaters, comparators, and daylight sensors; ignites and deprimes TNT and creepers; shears sheep; toggles armor stand arms; squirts squid; makes pandas sneeze; removes the helmet of a snow golem; inflates pufferfish; takes honeycomb from beehives and nests, and activates some grand **Hexcasting** work.

## Sources

- 📖 **Guidebook:** [[Prestidigitation (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Prestidigitation](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/prestidigitation@hexical:prestidigitation)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPrestidigitation` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:prestidigitation`
