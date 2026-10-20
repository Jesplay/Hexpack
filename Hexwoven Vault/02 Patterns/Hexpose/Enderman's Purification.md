---
aliases: ["Enderman's Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:is_wet"
signature: "qqqqwaadq"
start_direction: "SOUTH_WEST"
input: "entity"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Entities (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enderman's Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Entities (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 72.3,29.8 51.0,66.6 93.5,66.6 72.3,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 72.3,29.8 51.0,66.6 93.5,66.6 72.3,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="66.6" x2="44.2" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,86.8 40.2,76.0 48.3,80.7" fill="#fc77be"/><circle cx="51.0" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqwaadq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:is_wet`

```text
before   … │ entity   ◂ top
after    … │ boolean  ◂ top
```

## Description

Pushes whether the entity is touching rain, water, or in a bubble column.

## Related patterns (same book page)

- [[Caliper's Purification]] `entity → number`
- [[Theodolite's Purification]] `entity → vector`
- [[Vitality Purification]] `entity → number`
- [[Fitness Purification]] `entity → number`
- [[Suffocation Purification]] `entity → number`
- [[Lung Purification]] `entity → number`
- [[Inferno Purification]] `entity → number`
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
- 🌐 **Official online docs:** [Enderman's Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:is_wet)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:is_wet`
