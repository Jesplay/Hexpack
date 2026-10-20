---
aliases: ["Reproduction Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:breedable"
signature: "awaaqdqaawa"
start_direction: "EAST"
input: "entity"
output: "boolean/null"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]", "[[Null]]"]
entry: "[[Entities (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Reproduction Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Entities (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 85.0,94.0 65.7,60.5 46.4,27.0 27.0,60.5 65.7,60.5 85.0,27.0 104.3,60.5 143.0,60.5 123.6,27.0 104.3,60.5 85.0,94.0 123.6,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="46.4,94.0 85.0,94.0 65.7,60.5 46.4,27.0 27.0,60.5 65.7,60.5 85.0,27.0 104.3,60.5 143.0,60.5 123.6,27.0 104.3,60.5 85.0,94.0 123.6,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="58.7" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.6,94.0 58.7,98.2 58.7,89.7" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awaaqdqaawa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `boolean/null` ([[Boolean]], [[Null]])
- **ID:** `hexpose:breedable`

```text
before   … │ entity        ◂ top
after    … │ boolean/null  ◂ top
```

## Description

Pushes whether an animal creature is in love and ready to mate, or Null if that creature is not capable of that type of reproduction.

## Related patterns (same book page)

- [[Caliper's Purification]] `entity → number`
- [[Theodolite's Purification]] `entity → vector`
- [[Vitality Purification]] `entity → number`
- [[Fitness Purification]] `entity → number`
- [[Suffocation Purification]] `entity → number`
- [[Lung Purification]] `entity → number`
- [[Inferno Purification]] `entity → number`
- [[Enderman's Purification]] `entity → boolean`
- [[Youth Purification]] `entity → boolean`
- [[Sloth's Purification]] `entity → boolean`
- [[Racer's Purification]] `entity → boolean`
- [[Vehicle Purification]] `entity → entity/null`
- [[Jockey Purification]] `entity → list of entities`
- [[Shooter Purification]] `entity → entity/null`
- [[Adoration Purification]] `entity → entity/null`
- [[Absorption Purification]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Entities (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Reproduction Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:breedable)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetLivingEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:breedable`
