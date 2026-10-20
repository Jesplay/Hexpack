---
aliases: ["Conjure Fragments"]
tags: [hexcasting/spell, mod/special-efhexs, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:play_item_particle"
signature: "eqqqqqaaeaq"
start_direction: "NORTH_EAST"
input: "..., item"
output: ""
cost: ""
iotas_in: ["[[Item Stack]]"]
iotas_out: []
entry: "[[Particles (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Fragments

✦ **Spell** · [[Special Efhexs]] · 📖 [[Particles (Book Entry)]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,127.4 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="65.7,127.4 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="127.4" x2="71.9" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,109.0 75.5,118.8 68.2,114.6" fill="#fc77be"/><circle cx="65.7" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqqaaeaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `..., item` ([[Item Stack]])
- **Leaves:** `nothing` (—)
- **ID:** `efhexs:play_item_particle`

## Description

Conjures a visual bit of fragments, similar to that caused by a tool breaking after wearing. Takes in an item [[Iotas|iota]].

## Related patterns (same book page)

- [[Particulate Reflection]] `— → list of identifiers`
- [[Conjure Particle]] `identifier, vector, vector → —`
- [[Conjure Dust]] `..., vector, number → —`
- [[Conjure Sparkly Dust]] `..., vector, vector, number → —`
- [[Conjure Debris]] `..., identifier → —`
- [[Conjure Sediment]] `..., identifier → —`
- [[Conjure Media]] `..., vector → —`

## Sources

- 📖 **Guidebook:** [[Particles (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Fragments](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/particles@efhexs:play_item_particle)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPlayComplexParticle` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `efhexs:play_item_particle`
