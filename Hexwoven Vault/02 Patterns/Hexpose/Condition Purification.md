---
aliases: ["Condition Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/effects]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Status Effects"]
mod: "Hexpose"
op_id: "hexpose:get_effect_category"
signature: "wqqqaawd"
start_direction: "SOUTH_WEST"
input: "identifier"
output: "number"
cost: ""
iotas_in: ["[[Identifier]]"]
iotas_out: ["[[Number]]"]
entry: "[[Status Effects]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Condition Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Status Effects]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 76.9,115.3 94.4,85.0 59.4,85.0 76.9,115.3 94.4,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:2.5s" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 76.9,115.3 94.4,85.0 59.4,85.0 76.9,115.3 94.4,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="24.5" x2="53.8" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,41.1 50.5,32.2 57.2,36.1" fill="#fc77be"/><circle cx="59.4" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqaawd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `identifier` ([[Identifier]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_effect_category`

```text
before   … │ identifier  ◂ top
after    … │ number      ◂ top
```

## Description

Pushes the general "goodness" of the effect towards the afflicted. -1 for harmful, 0 for neutral, and 1 for beneficial.

## Related patterns (same book page)

- [[Diagnosis Purification]] `entity → list of identifiers`
- [[Prescription Purification]] `item → list of identifiers`
- [[Concentration Distillation]] `entity, identifier → number`
- [[Clearance Distillation]] `entity, identifier → number`

## Sources

- 📖 **Guidebook:** [[Status Effects]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Condition Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/status_effects@hexpose:get_effect_category)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetStatusEffectCategory` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_effect_category`
