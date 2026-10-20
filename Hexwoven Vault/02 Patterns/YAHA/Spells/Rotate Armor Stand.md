---
aliases: ["Rotate Armor Stand"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/creatures]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities"]
mod: "YAHA"
op_id: "yaha:stand_rotate_yaw"
signature: "eadawddwea"
start_direction: "NORTH_EAST"
input: "entity, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Armor Stand Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Rotate Armor Stand

✦ **Spell** · [[YAHA]] · 📖 [[Armor Stand Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,145.5 41.9,115.3 76.9,115.3 59.4,85.0 94.4,85.0 76.9,54.7 59.4,24.5 94.4,24.5 76.9,54.7 59.4,85.0 24.5,85.0 41.9,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="24.5,145.5 41.9,115.3 76.9,115.3 59.4,85.0 94.4,85.0 76.9,54.7 59.4,24.5 94.4,24.5 76.9,54.7 59.4,85.0 24.5,85.0 41.9,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="145.5" x2="30.1" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="34.1,128.9 33.4,137.8 26.7,133.9" fill="#fc77be"/><circle cx="24.5" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eadawddwea` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `yaha:stand_rotate_yaw`

```text
before   … │ entity │ number  ◂ top
after    …                    ◂ top
```

## Description

Faces the armor stand towards the given yaw in radians, where south is 0 rad, west is π/2 rad, north is π rad, and east is -π/2 rad.

## Related patterns (same book page)

- [[ShowHide Arms]] `entity → —`
- [[ShowHide Baseplate]] `entity → —`
- [[ShrinkGrow Armor Stand]] `entity → —`
- [[Orient HeadBody]] `entity, vector → —`
- [[Orient LeftRight Arm]] `entity, vector → —`
- [[Orient LeftRight Leg]] `entity, vector → —`

## Sources

- 📖 **Guidebook:** [[Armor Stand Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `yaha:stand_rotate_yaw`
