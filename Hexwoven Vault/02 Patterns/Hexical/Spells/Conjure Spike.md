---
aliases: ["Conjure Spike"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hexical"
op_id: "hexical:spike"
signature: "qdqdqdqdww"
start_direction: "NORTH_EAST"
input: "vector, vector, number"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Conjure Spike (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Spike

✦ **Spell** · [[Hexical]] · 📖 [[Conjure Spike (Book Entry)]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,127.4 46.4,94.0 27.0,60.5 65.7,60.5 85.0,27.0 104.3,60.5 143.0,60.5 123.6,94.0 143.0,127.4 104.3,127.4 65.7,127.4 27.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="27.0,127.4 46.4,94.0 27.0,60.5 65.7,60.5 85.0,27.0 104.3,60.5 143.0,60.5 123.6,94.0 143.0,127.4 104.3,127.4 65.7,127.4 27.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="127.4" x2="33.2" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,109.0 36.9,118.8 29.5,114.6" fill="#fc77be"/><circle cx="27.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qdqdqdqdww` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexical:spike`

```text
before   … │ vector │ vector │ number  ◂ top
after    …                             ◂ top
```

## Description

Takes a non-air position, an axis vector, and a delay in seconds up to ten seconds long and conjures a spike at that location. Costs about one [[Amethyst|Amethyst Dust]].

Each spike takes about a half second to strike and deal a hefty three full hearts of damage before slowly sinking back into the ground. If I cast this spell where a spike has not yet disappeared, it will do nothing. Spikes will also throw back targets in the direction they were conjured.

## Sources

- 📖 **Guidebook:** [[Conjure Spike (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Spike](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/spike@hexical:spike)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureSpike` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:spike`
