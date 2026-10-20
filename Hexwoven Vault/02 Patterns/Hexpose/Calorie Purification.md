---
aliases: ["Calorie Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:get_hunger"
signature: "adaqqqddqe"
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

# Calorie Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Food]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 51.0,29.8 72.3,66.6 29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 136.1,29.8 114.8,66.6 136.1,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="93.5,29.8 51.0,29.8 72.3,66.6 29.8,66.6 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 136.1,29.8 114.8,66.6 136.1,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="79.9" y2="29.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,29.8 79.9,25.1 79.9,34.4" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaqqqddqe` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_hunger`

```text
before   … │ identifier  ◂ top
after    … │ number      ◂ top
```

## Description

Takes in an item identifier and pushes the amount it will fill me.

## Related patterns (same book page)

- [[Hunger Purification]] `player entity → number`
- [[Stamina Purification]] `entity entity → number`
- [[Edibility Purification]] `identifier → boolean`
- [[Satiation Purification]] `identifier → number`
- [[Flesh Purification]] `identifier → boolean`
- [[Dessert Purification]] `identifier → boolean`

## Sources

- 📖 **Guidebook:** [[Food]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Calorie Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/food@hexpose:get_hunger)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetFoodTypeData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_hunger`
