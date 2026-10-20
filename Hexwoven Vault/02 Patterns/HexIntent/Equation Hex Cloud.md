---
aliases: ["Equation Hex Cloud"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:equation_hex_cloud"
signature: "qaqeaddwe"
start_direction: "NORTH_EAST"
input: "vec|entity, equation, num"
output: "none"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Visual Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Equation Hex Cloud

◇ **Pattern** · [[HexIntent]] · 📖 [[Visual Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 101.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 41.9,115.3 76.9,115.3 59.4,145.5 41.9,115.3 24.5,85.0 41.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="59.4,85.0 76.9,54.7 59.4,24.5 41.9,54.7 59.4,85.0 41.9,115.3 76.9,115.3 59.4,145.5 41.9,115.3 24.5,85.0 41.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="85.0" x2="65.0" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,68.4 68.3,77.2 61.7,73.4" fill="#fc77be"/><circle cx="59.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqeaddwe` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec|entity, equation, num` ([[Number]], [[Vector]], [[Entity]])
- **Leaves:** `none` (—)
- **ID:** `manifestation:equation_hex_cloud`

## Description

Creates large particle effects from an Equation Particle [[Iotas|iota]] to avoid large vector lists. The Iota can be created using an Equation Synthesizer block. Uses an ID to keep positional state like Hex Trail.

Inputs:
target: vector where the point cloud is created,
OR
anchor: entity center to follow,
OR
anchor + offset: entity center plus offset vector.
equation: Equation Particle iota that defines point layout and colour behavior.
id: numeric cloud id.

Using the same id smoothly moves the cloud between positions.

## Related patterns (same book page)

- [[Hex Trail]] `vec, vec, vec, num, num, num? → none`
- [[Manifest Spell Circle]] `[pattern], vec, vec, num, num?, vec? → none`

## Sources

- 📖 **Guidebook:** [[Visual Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEquationHexCloud` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:equation_hex_cloud`
