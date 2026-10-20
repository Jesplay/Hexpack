---
aliases: ["Gasp"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:gasp"
signature: "aweeeeewaweeeee"
start_direction: "NORTH_WEST"
input: "entity"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gasp

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="98.3,41.6 85.0,18.6 71.7,41.6 58.4,64.6 31.9,64.6 18.6,41.6 31.9,18.6 58.4,18.6 71.7,41.6 85.0,64.6 98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:4.5s" points="98.3,41.6 85.0,18.6 71.7,41.6 58.4,64.6 31.9,64.6 18.6,41.6 31.9,18.6 58.4,18.6 71.7,41.6 85.0,64.6 98.3,41.6 111.6,18.6 138.1,18.6 151.4,41.6 138.1,64.6 111.6,64.6 98.3,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="98.3" y1="41.6" x2="94.0" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="91.0,28.9 96.6,32.8 91.5,35.7" fill="#fc77be"/><circle cx="98.3" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aweeeeewaweeeee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexical:gasp`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Instantly replenishes a creature's air bubbles. Costs about one [[Amethyst|Amethyst Dust]].

Regardless of amount of breath restored, this spell costs exactly the same so I should wait as long as I safely can before casting this spell. I will find it useful while diving, being teleported into a block, or even keeping sea creatures alive on land.

*The best way to observe a fish is to become a fish.*

## Related patterns (same book page)

- [[Confetti]] `vector, number/vector → —`
- [[Vibrate]] `vector, entity/vector, number → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Crack Device]] `— → —`
- [[Conjure Flower]] `vector, identifer → —`
- [[Illuminate]] `vector, number → —`
- [[Squawk]] `vector, identifier → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Gasp](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/hexical_spells@hexical:gasp)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpGasp` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:gasp`
