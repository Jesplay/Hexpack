---
aliases: ["Squawk"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:parrot"
signature: "wweedadw"
start_direction: "NORTH_EAST"
input: "vector, identifier"
output: ""
cost: "Costs about a half of an Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Identifier]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Squawk

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,127.4 46.4,94.0 65.7,60.5 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 123.6,94.0 85.0,94.0 46.4,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="27.0,127.4 46.4,94.0 65.7,60.5 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 123.6,94.0 85.0,94.0 46.4,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="127.4" x2="33.2" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,109.0 36.9,118.8 29.5,114.6" fill="#fc77be"/><circle cx="27.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wweedadw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, identifier` ([[Vector]], [[Identifier]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a half of an Amethyst Dust.
- **ID:** `hexical:parrot`

```text
before   … │ vector │ identifier  ◂ top
after    …                        ◂ top
```

## Description

Emits a sound associated with a mob near a location, as though a parrot had imitated that mob. Costs about a half of an [[Amethyst|Amethyst Dust]].

This spell does nothing but still consumes my **[[Media|media]]** if a parrot can not replicate the sound.

I can already imagine this spell's use for playing practical jokes on my friends and enemies. Perhaps Nature does permit me to have fun occasionally with my **Hexes**.

*Hisss...*

## Related patterns (same book page)

- [[Confetti]] `vector, number/vector → —`
- [[Vibrate]] `vector, entity/vector, number → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Crack Device]] `— → —`
- [[Conjure Flower]] `vector, identifer → —`
- [[Illuminate]] `vector, number → —`
- [[Gasp]] `entity → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Squawk](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/hexical_spells@hexical:parrot)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpImitateParrot` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:parrot`
