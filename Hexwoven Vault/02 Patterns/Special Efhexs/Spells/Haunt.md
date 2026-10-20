---
aliases: ["Haunt"]
tags: [hexcasting/spell, mod/special-efhexs, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:set_target"
signature: "aawawqeqqqqqwa"
start_direction: "EAST"
input: "null | list of entities"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[List]]", "[[Null]]"]
iotas_out: []
entry: "[[Haunting]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Haunt

✦ **Spell** · [[Special Efhexs]] · 📖 [[Haunting]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="126" viewBox="0 0 170.0 125.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/></g><polyline class="hexpat-base" points="37.8,76.6 69.3,76.6 53.5,49.3 37.8,76.6 22.0,103.8 53.5,103.8 85.0,103.8 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3 100.7,76.6 116.5,103.8 132.2,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="37.8,76.6 69.3,76.6 53.5,49.3 37.8,76.6 22.0,103.8 53.5,103.8 85.0,103.8 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 85.0,49.3 100.7,76.6 116.5,103.8 132.2,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="76.6" x2="47.9" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,76.6 47.9,80.0 47.9,73.1" fill="#fc77be"/><circle cx="37.8" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawawqeqqqqqwa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `null | list of entities` ([[Entity]], [[List]], [[Null]])
- **Leaves:** `nothing` (—)
- **ID:** `efhexs:set_target`

## Description

Sets a list of players to be exclusively influenced by my special effects. If Null, all can see my effects again.

## Sources

- 📖 **Guidebook:** [[Haunting]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Haunt](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/targetted@efhexs:set_target)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSetTargets` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `efhexs:set_target`
