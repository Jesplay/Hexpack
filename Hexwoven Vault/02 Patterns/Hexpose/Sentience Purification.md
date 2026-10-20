---
aliases: ["Sentience Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:is_brainswept"
signature: "qqqaqqq"
start_direction: "SOUTH_EAST"
input: "entity"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Enlightened Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sentience Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Enlightened Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 85.0,27.0 65.7,60.5 85.0,94.0 123.6,94.0 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,78.9 29.5,73.3 36.9,69.1" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqaqqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:is_brainswept`

```text
before   … │ entity   ◂ top
after    … │ boolean  ◂ top
```

## Description

Pushes whether an entity has been "magically relieved of its mind". I feel this is strongly related to [[Flay Mind (Book Entry)|Flay Mind]] somehow...

## Related patterns (same book page)

- [[Epiphany Purification]] `player entity → boolean`

## Sources

- 📖 **Guidebook:** [[Enlightened Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sentience Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/enlightened_patterns@hexpose:is_brainswept)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetLivingEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:is_brainswept`
