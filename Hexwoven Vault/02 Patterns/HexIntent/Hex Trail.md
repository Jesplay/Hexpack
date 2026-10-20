---
aliases: ["Hex Trail"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:hex_trail"
signature: "qaqead"
start_direction: "NORTH_EAST"
input: "vec, vec, vec, num, num, num?"
output: "none"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Visual Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hex Trail

◇ **Pattern** · [[HexIntent]] · 📖 [[Visual Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="84" height="170" viewBox="0 0 83.9 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,85.0 59.4,54.7 41.9,24.5 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 41.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:2.0s" points="41.9,85.0 59.4,54.7 41.9,24.5 24.5,54.7 41.9,85.0 24.5,115.3 59.4,115.3 41.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="85.0" x2="47.5" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,68.4 50.9,77.2 44.2,73.4" fill="#fc77be"/><circle cx="41.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqead` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec, vec, num, num, num?` ([[Number]], [[Vector]])
- **Leaves:** `none` (—)
- **ID:** `manifestation:hex_trail`

```text
before   … │ vec │ vec │ vec │ num │ num │ num?  ◂ top
after    … │ none                                ◂ top
```

## Description

Hex Trail creates a particle trail as it moves. By using the same ID across casts you can create a continuous trail that moves smoothy between positions.

The positions have to be within 10 blocks of each other for the trail to smoothy move between them, otherwise it will snap to the new position. You also specify two colours for the trail to cycle between and a duration in ticks for the colour transition. There are a variety of particle types to choose from, some new and some from vanilla Minecraft. Not all particles can be tinted. The ID allows you to have multiple trails active and make sure they all animate independently.

Inputs:
position: vector for the particle to appear at or move to.
color from: RGB vector (0..1) at the start of the color cycle.
color to: RGB vector (0..1) at the end of the color cycle.
duration: time in ticks it takes for the colour to transition
id: trail identifier. This is what's used to look up the last position and animate between the position.
type?: optional particle style as a number (default 0).

Types:
0 dust ribbon
1 sparks
2 trail
3 pulse beads
4 end rod
5 witch plume
6 enchant flecks
7 soul flames
8 solid beams
9 ring lattices
10 lightning arcs
11 hex conjure ribbons
12 hex conjure sparkss
13 hex conjure trails
14 dual-helix.

## Related patterns (same book page)

- [[Equation Hex Cloud]] `vec|entity, equation, num → none`
- [[Manifest Spell Circle]] `[pattern], vec, vec, num, num?, vec? → none`

## Sources

- 📖 **Guidebook:** [[Visual Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpHexTrail` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:hex_trail`
