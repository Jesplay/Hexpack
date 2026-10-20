---
aliases: ["Satiation Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:get_saturation"
signature: "adaqqqddqw"
start_direction: "WEST"
input: "identifier"
output: "number"
cost: ""
iotas_in: ["[[Identifier]]"]
iotas_out: ["[[Number]]"]
entry: "[[Food]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Satiation Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Food]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 46.4,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 123.6,27.0 104.3,60.5 123.6,94.0 143.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="85.0,27.0 46.4,27.0 65.7,60.5 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 123.6,27.0 104.3,60.5 123.6,94.0 143.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="72.6" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="63.8,27.0 72.6,22.8 72.6,31.3" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaqqqddqw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_saturation`

```text
before   … │ identifier  ◂ top
after    … │ number      ◂ top
```

## Description

Takes in an item identifier and pushes the saturation value, which roughly affects how long it will fill me.

## Related patterns (same book page)

- [[Hunger Purification]] `player entity → number`
- [[Stamina Purification]] `entity entity → number`
- [[Edibility Purification]] `identifier → boolean`
- [[Calorie Purification]] `identifier → number`
- [[Flesh Purification]] `identifier → boolean`
- [[Dessert Purification]] `identifier → boolean`

## Sources

- 📖 **Guidebook:** [[Food]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Satiation Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/food@hexpose:get_saturation)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetFoodTypeData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_saturation`
