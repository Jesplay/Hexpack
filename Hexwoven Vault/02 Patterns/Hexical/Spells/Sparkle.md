---
aliases: ["Sparkle"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:sparkle"
signature: "dqa"
start_direction: "NORTH_EAST"
input: "vector, vector, number"
output: ""
cost: "Costs a negligible amount of media."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sparkle

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 60.0,35.0 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,78.3 60.0,35.0 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="43.0" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,54.5 47.8,67.2 38.2,61.7" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexical:sparkle`

```text
before   … │ vector │ vector │ number  ◂ top
after    …                             ◂ top
```

## Description

Produces a small sparkling particle of my desired position, color, and lifespan. Costs a negligible amount of **[[Media|media]]**.

The first vector controls position and the second vector controls color. Each component should range from 0 to 1 and corresponds to red, green, and blueness respectively. The final number dictates how long the sparkle should remain in twentieths of a second, up to five seconds.

*All that glitters is not gold.*

## Related patterns (same book page)

- [[Confetti]] `vector, number/vector → —`
- [[Vibrate]] `vector, entity/vector, number → —`
- [[Crack Device]] `— → —`
- [[Conjure Flower]] `vector, identifer → —`
- [[Illuminate]] `vector, number → —`
- [[Gasp]] `entity → —`
- [[Squawk]] `vector, identifier → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sparkle](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/hexical_spells@hexical:sparkle)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSparkle` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:sparkle`
