---
aliases: ["Dessert Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:is_snack"
signature: "adaqqqddaq"
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

# Dessert Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Food]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="76.3,24.3 41.6,24.3 59.0,54.3 24.3,54.3 41.6,84.4 76.3,84.4 93.7,54.3 76.3,24.3 111.0,24.3 93.7,54.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:382;--dur:3.1s" points="76.3,24.3 41.6,24.3 59.0,54.3 24.3,54.3 41.6,84.4 76.3,84.4 93.7,54.3 76.3,24.3 111.0,24.3 93.7,54.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="24.3" x2="65.2" y2="24.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.2,24.3 65.2,20.5 65.2,28.1" fill="#fc77be"/><circle cx="76.3" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaqqqddaq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:is_snack`

```text
before   … │ identifier  ◂ top
after    … │ boolean     ◂ top
```

## Description

Takes in an item identifier and pushes whether it's a snack.

## Related patterns (same book page)

- [[Hunger Purification]] `player entity → number`
- [[Stamina Purification]] `entity entity → number`
- [[Edibility Purification]] `identifier → boolean`
- [[Calorie Purification]] `identifier → number`
- [[Satiation Purification]] `identifier → number`
- [[Flesh Purification]] `identifier → boolean`

## Sources

- 📖 **Guidebook:** [[Food]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Dessert Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/food@hexpose:is_snack)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetFoodTypeData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:is_snack`
