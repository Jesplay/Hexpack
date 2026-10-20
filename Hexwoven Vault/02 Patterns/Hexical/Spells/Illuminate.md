---
aliases: ["Illuminate"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather", "Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:light"
signature: "aeaeaeaeaeawqqqqq"
start_direction: "SOUTH_EAST"
input: "vector, number"
output: ""
cost: "Costs about a quarter of an Amethyst Dust."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Illuminate

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]] · [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="59.4,115.3 76.9,145.5 94.4,115.3 129.3,115.3 111.8,85.0 129.3,54.7 94.4,54.7 76.9,24.5 59.4,54.7 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 94.4,115.3 111.8,85.0 94.4,54.7 59.4,54.7 41.9,85.0 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="115.3" x2="65.0" y2="125.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,131.9 61.7,126.9 68.3,123.0" fill="#fc77be"/><circle cx="59.4" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeaeaeaeaeawqqqqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a quarter of an Amethyst Dust.
- **ID:** `hexical:light`

```text
before   … │ vector │ number  ◂ top
after    …                    ◂ top
```

## Description

Conjures a completely invisible light with illumination strength of my choosing at a location. Costs about a quarter of an [[Amethyst|Amethyst Dust]].

The block that it is into must be empty or extremely weak. The illumination strength can be a number between 1 and 15, inclusive. If I target a space where a light already exists, the spell is free and particleless.

By holding a [[Scrying Lens]], I can see the lights and place a block in their place to remove them.

*The light blinds us. It is only in the dark that we see clearly.*

## Related patterns (same book page)

- [[Confetti]] `vector, number/vector → —`
- [[Vibrate]] `vector, entity/vector, number → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Crack Device]] `— → —`
- [[Conjure Flower]] `vector, identifer → —`
- [[Gasp]] `entity → —`
- [[Squawk]] `vector, identifier → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Illuminate](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/hexical_spells@hexical:light)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureLight` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:light`
