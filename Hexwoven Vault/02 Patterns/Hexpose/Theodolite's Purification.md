---
aliases: ["Theodolite's Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:theodolite"
signature: "wqaa"
start_direction: "EAST"
input: "entity"
output: "vector"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Entities (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Theodolite's Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Entities (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/></g><polyline class="hexpat-base" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,30.5 95.9,30.5 117.7,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:218;--dur:1.6s" points="30.5,68.3 74.1,68.3 117.7,68.3 139.5,30.5 95.9,30.5 117.7,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="68.3" x2="44.5" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,68.3 44.5,73.1 44.5,63.5" fill="#fc77be"/><circle cx="30.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexpose:theodolite`

```text
before   … │ entity  ◂ top
after    … │ vector  ◂ top
```

## Description

Similar to [[Alidade's Purification]] but pushes the vector that points upwards from their head rather than the entity's facing vector.

## Related patterns (same book page)

- [[Caliper's Purification]] `entity → number`
- [[Vitality Purification]] `entity → number`
- [[Fitness Purification]] `entity → number`
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
- 🌐 **Official online docs:** [Theodolite's Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:theodolite)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:theodolite`
