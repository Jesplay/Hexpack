---
aliases: ["Edibility Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:edible"
signature: "adaqqqdd"
start_direction: "WEST"
input: "identifier"
output: "boolean"
cost: ""
iotas_in: ["[[Identifier]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Food]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Edibility Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Food]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,30.5 52.3,30.5 74.1,68.3 30.5,68.3 52.3,106.0 95.9,106.0 117.7,68.3 95.9,30.5 139.5,30.5 117.7,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:392;--dur:2.5s" points="95.9,30.5 52.3,30.5 74.1,68.3 30.5,68.3 52.3,106.0 95.9,106.0 117.7,68.3 95.9,30.5 139.5,30.5 117.7,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="30.5" x2="81.9" y2="30.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="71.9,30.5 81.9,25.7 81.9,35.3" fill="#fc77be"/><circle cx="95.9" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaqqqdd` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:edible`

```text
before   … │ identifier  ◂ top
after    … │ boolean     ◂ top
```

## Description

Takes in an item identifier and pushes whether I can eat it.

## Related patterns (same book page)

- [[Hunger Purification]] `player entity → number`
- [[Stamina Purification]] `entity entity → number`
- [[Calorie Purification]] `identifier → number`
- [[Satiation Purification]] `identifier → number`
- [[Flesh Purification]] `identifier → boolean`
- [[Dessert Purification]] `identifier → boolean`

## Sources

- 📖 **Guidebook:** [[Food]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Edibility Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/food@hexpose:edible)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetItemTypeData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:edible`
