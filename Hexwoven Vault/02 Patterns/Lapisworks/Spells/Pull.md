---
aliases: ["Pull"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Lapisworks"
op_id: "lapisworks:pull"
signature: "awwqqqwwaqww"
start_direction: "SOUTH_WEST"
input: "entity, vec, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Enchanted Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Pull

✦ **Spell** · [[Lapisworks]] · 📖 [[Enchanted Spells]] · Lapisworks

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="41.6,54.3 24.3,84.4 59.0,84.4 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3 24.3,24.3 41.6,54.3 76.3,54.3 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:451;--dur:3.6s" points="41.6,54.3 24.3,84.4 59.0,84.4 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3 24.3,24.3 41.6,54.3 76.3,54.3 111.0,54.3 145.7,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.6" y1="54.3" x2="36.1" y2="63.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.1,70.9 32.8,62.0 39.4,65.9" fill="#fc77be"/><circle cx="41.6" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwqqqwwaqww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vec, num` ([[Number]], [[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:pull`

```text
before   … │ entity │ vec │ num  ◂ top
after    …                       ◂ top
```

## Description

Accelerates the given entity by the provided vector (like Impulse) for the specified amount of time (in twentieths of a second). Can be used to achieve drift/curve-shot-like effects. Costs the same as Impulse (including the subsequent cost per entity per cast) but multiplied by the time.

## Related patterns (same book page)

- [[Conjure Solid Color]] `vec, num → —`
- [[Dispel Conjureable]] `vec → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Spells]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `Pull` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:pull`
