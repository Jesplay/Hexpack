---
aliases: ["Shooter Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:shooter"
signature: "aadedade"
start_direction: "EAST"
input: "entity"
output: "entity/null"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Entities (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Shooter Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Entities (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,121.6 110.0,121.6 85.0,78.3 60.0,121.6 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="60.0,121.6 110.0,121.6 85.0,78.3 60.0,121.6 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="121.6" x2="76.0" y2="121.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="87.5,121.6 76.0,127.1 76.0,116.1" fill="#fc77be"/><circle cx="60.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aadedade` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `entity/null` ([[Entity]], [[Null]])
- **ID:** `hexpose:shooter`

```text
before   … │ entity       ◂ top
after    … │ entity/null  ◂ top
```

## Description

Pushes the shooter of the given projectile entity or Null if it has no shooter or isn't a projectile. It also pushes Null if the shooter is outside my ambit.

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
- [[Reproduction Purification]] `entity → boolean/null`
- [[Sloth's Purification]] `entity → boolean`
- [[Racer's Purification]] `entity → boolean`
- [[Vehicle Purification]] `entity → entity/null`
- [[Jockey Purification]] `entity → list of entities`
- [[Adoration Purification]] `entity → entity/null`
- [[Absorption Purification]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Entities (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Shooter Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:shooter)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpShooter` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:shooter`
