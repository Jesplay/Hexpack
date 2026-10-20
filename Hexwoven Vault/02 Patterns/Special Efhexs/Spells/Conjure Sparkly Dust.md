---
aliases: ["Conjure Sparkly Dust"]
tags: [hexcasting/spell, mod/special-efhexs, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:play_dust_transition_particle"
signature: "eqqqqqaaqda"
start_direction: "NORTH_EAST"
input: "..., vector, vector, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Particles (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Sparkly Dust

✦ **Spell** · [[Special Efhexs]] · 📖 [[Particles (Book Entry)]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="76.3,114.4 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 93.7,84.4 111.0,54.3 76.3,54.3 59.0,84.4 41.6,54.3 24.3,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:416;--dur:3.4s" points="76.3,114.4 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 93.7,84.4 111.0,54.3 76.3,54.3 59.0,84.4 41.6,54.3 24.3,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="114.4" x2="81.9" y2="104.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="85.9,97.9 85.2,106.7 78.6,102.9" fill="#fc77be"/><circle cx="76.3" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqqaaqda` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `..., vector, vector, number` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `efhexs:play_dust_transition_particle`

## Description

Conjures a gradient-like mote of dust, which smoothly transitions between the two colors given over the given number of seconds.

## Related patterns (same book page)

- [[Particulate Reflection]] `— → list of identifiers`
- [[Conjure Particle]] `identifier, vector, vector → —`
- [[Conjure Dust]] `..., vector, number → —`
- [[Conjure Debris]] `..., identifier → —`
- [[Conjure Sediment]] `..., identifier → —`
- [[Conjure Fragments]] `..., item → —`
- [[Conjure Media]] `..., vector → —`

## Sources

- 📖 **Guidebook:** [[Particles (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Sparkly Dust](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/particles@efhexs:play_dust_transition_particle)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPlayComplexParticle` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `efhexs:play_dust_transition_particle`
