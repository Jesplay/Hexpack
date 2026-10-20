---
aliases: ["Conjure Media"]
tags: [hexcasting/spell, mod/special-efhexs, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:play_hex_particle"
signature: "eqqqqqaewawqwaw"
start_direction: "NORTH_EAST"
input: "..., vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Particles (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Media

✦ **Spell** · [[Special Efhexs]] · 📖 [[Particles (Book Entry)]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,145.5 41.9,115.3 76.9,115.3 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 59.4,85.0 94.4,85.0 129.3,85.0 111.8,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 59.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:559;--dur:4.5s" points="24.5,145.5 41.9,115.3 76.9,115.3 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 59.4,85.0 94.4,85.0 129.3,85.0 111.8,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 59.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="145.5" x2="30.1" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="34.1,128.9 33.4,137.8 26.7,133.9" fill="#fc77be"/><circle cx="24.5" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqqaewawqwaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `..., vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `efhexs:play_hex_particle`

## Description

Conjures a blob of visible **[[Media|media]]**, not unlike the waste product of my **Hexes**. Takes in a color.

## Related patterns (same book page)

- [[Particulate Reflection]] `— → list of identifiers`
- [[Conjure Particle]] `identifier, vector, vector → —`
- [[Conjure Dust]] `..., vector, number → —`
- [[Conjure Sparkly Dust]] `..., vector, vector, number → —`
- [[Conjure Debris]] `..., identifier → —`
- [[Conjure Sediment]] `..., identifier → —`
- [[Conjure Fragments]] `..., item → —`

## Sources

- 📖 **Guidebook:** [[Particles (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Media](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/particles@efhexs:play_hex_particle)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPlayComplexParticle` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `efhexs:play_hex_particle`
