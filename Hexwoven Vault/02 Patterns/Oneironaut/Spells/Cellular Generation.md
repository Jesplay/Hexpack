---
aliases: ["Cellular Generation"]
tags: [hexcasting/spell, mod/oneironaut, family/spell, topic/blocks, hexcasting/not-in-book]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Oneironaut"
op_id: "oneironaut:advanceautomaton"
signature: "qqwqwqwaqeee"
start_direction: "SOUTH_WEST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Undocumented Oneironaut Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: false
---

# Cellular Generation

> [!caution] Not in the guidebook
> This pattern exists in the mod's code, but the in-game book doesn't describe it. Its inputs and outputs are unknown; test it on a small stack with [[Reveal]].

✦ **Spell** · [[Oneironaut]] · 📖 [[Undocumented Oneironaut Patterns]] · Not in the Book

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="169" viewBox="0 0 170.0 168.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/><circle cx="6.9" cy="144.5" r="2"/><circle cx="41.6" cy="144.5" r="2"/><circle cx="76.3" cy="144.5" r="2"/><circle cx="111.0" cy="144.5" r="2"/><circle cx="145.7" cy="144.5" r="2"/></g><polyline class="hexpat-base" points="41.6,84.4 24.3,114.4 41.6,144.5 76.3,144.5 111.0,144.5 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 93.7,54.3 111.0,84.4 93.7,114.4 59.0,114.4 41.6,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:451;--dur:3.6s" points="41.6,84.4 24.3,114.4 41.6,144.5 76.3,144.5 111.0,144.5 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 93.7,54.3 111.0,84.4 93.7,114.4 59.0,114.4 41.6,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.6" y1="84.4" x2="36.1" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.1,100.9 32.8,92.1 39.4,95.9" fill="#fc77be"/><circle cx="41.6" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqwqwqwaqeee` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes / Leaves:** *unknown (not documented)*
- **ID:** `oneironaut:advanceautomaton`

## Description

## Related patterns (same book page)

- [[Signature Purification]] `soulprint → bool`
- [[Circle]] `— → —`
- [[Stabilize Matter]] `— → —`
- [[Soulprint Reflection]] `— → soulprint`
- [[Sign Item]] `— → —`

## Sources

- 📖 **Guidebook:** [[Undocumented Oneironaut Patterns]] *(not in the in-game book)*
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAdvanceAutomaton` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:advanceautomaton`
