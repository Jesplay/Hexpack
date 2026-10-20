---
aliases: ["Particles"]
tags: [hexcasting/spell, mod/hexal, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexal"
op_id: "hexal:particles"
signature: "eqqqqa"
start_direction: "NORTH_EAST"
input: "vec | [vec]"
output: ""
cost: "Costs two thousandths of an Amethyst Dust per location."
iotas_in: ["[[Vector]]", "[[List]]"]
iotas_out: []
entry: "[[Hexal Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Particles

✦ **Spell** · [[Hexal]] · 📖 [[Hexal Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 72.3,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="29.8,140.2 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 72.3,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec | [vec]` ([[Vector]], [[List]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs two thousandths of an Amethyst Dust per location.
- **ID:** `hexal:particles`

## Description

Accepts a location or a list of locations, and either creates a particle at the location, or creates lines of particles between the locations in the list. Costs two thousandths of an [[Amethyst|Amethyst Dust]] per location.

## Related patterns (same book page)

- [[Smelt]] `vec | entity | mote → —`
- [[Freeze]] `vec → —`
- [[Falling Block]] `vec → —`
- [[Place Block II]] `(itemtype | mote), vec → —`

## Sources

- 📖 **Guidebook:** [[Hexal Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Particles](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/hexal_spells@hexal:particles)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpParticles` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:particles`
