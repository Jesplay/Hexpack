---
aliases: ["Inferno Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:burning"
signature: "eewdead"
start_direction: "WEST"
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

# Inferno Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Entities (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 72.3,29.8 93.5,66.6 72.3,103.4 114.8,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:340;--dur:2.2s" points="93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 72.3,29.8 93.5,66.6 72.3,103.4 114.8,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="140.2" x2="79.9" y2="140.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,140.2 79.9,135.6 79.9,144.9" fill="#fc77be"/><circle cx="93.5" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eewdead` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:burning`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the number of seconds the burning entity will continue burning if not put out. -1 if the entity is not burning currently.

## Related patterns (same book page)

- [[Caliper's Purification]] `entity → number`
- [[Theodolite's Purification]] `entity → vector`
- [[Vitality Purification]] `entity → number`
- [[Fitness Purification]] `entity → number`
- [[Suffocation Purification]] `entity → number`
- [[Lung Purification]] `entity → number`
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
- 🌐 **Official online docs:** [Inferno Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/entities@hexpose:burning)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:burning`
