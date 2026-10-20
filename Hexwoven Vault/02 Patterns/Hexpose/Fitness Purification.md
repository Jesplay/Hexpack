---
aliases: ["Fitness Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:get_max_health"
signature: "wddwwawaeqwawq"
start_direction: "SOUTH_EAST"
input: "entity"
output: "number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Entities (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Fitness Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Entities (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,54.7 59.4,85.0 76.9,115.3 41.9,115.3 59.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:524;--dur:4.2s" points="41.9,54.7 59.4,85.0 76.9,115.3 41.9,115.3 59.4,85.0 76.9,54.7 94.4,24.5 59.4,24.5 24.5,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 76.9,115.3 94.4,85.0 76.9,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="54.7" x2="47.5" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,71.4 44.2,66.3 50.9,62.5" fill="#fc77be"/><circle cx="41.9" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wddwwawaeqwawq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_max_health`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes how much health an entity can have at its prime. I can think of these in terms of half-hearts.

## Related patterns (same book page)

- [[Caliper's Purification]] `entity → number`
- [[Theodolite's Purification]] `entity → vector`
- [[Vitality Purification]] `entity → number`
- [[Suffocation Purification]] `entity → number`
- [[Lung Purification]] `entity → number`
- [[Inferno Purification]] `entity → number`
- [[Enderman's Purification]] `entity → boolean`
- [[Youth Purification]] `entity → boolean`
- [[Reproduction Purification]] `entity → boolean/null`
- [[Sloth's Purification]] `entity → boolean`
- [[Racer's Purification]] `entity → boolean`
- [[Vehicle Purification]] `entity → entity/null`
- [[Jockey Purification]] `entity → list of entities`
- [[Shooter Purification]] `entity → entity/null`
- [[Adoration Purification]] `entity → entity/null`
- [[Absorption Purification]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Entities (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Fitness Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:get_max_health)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetLivingEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_max_health`
