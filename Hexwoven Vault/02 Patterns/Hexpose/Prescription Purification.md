---
aliases: ["Prescription Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/effects]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Status Effects"]
mod: "Hexpose"
op_id: "hexpose:get_effects_item"
signature: "wqqqadee"
start_direction: "SOUTH_WEST"
input: "item"
output: "list of identifiers"
cost: ""
iotas_in: ["[[Item Stack]]"]
iotas_out: ["[[List]]", "[[Identifier]]"]
entry: "[[Status Effects]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Prescription Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Status Effects]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 65.7,94.0 85.0,60.5 123.6,60.5 143.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4 85.0,127.4 104.3,94.0 65.7,94.0 85.0,60.5 123.6,60.5 143.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="27.0" x2="59.5" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,45.4 55.8,35.6 63.2,39.9" fill="#fc77be"/><circle cx="65.7" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqadee` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item` ([[Item Stack]])
- **Leaves:** `list of identifiers` ([[List]], [[Identifier]])
- **ID:** `hexpose:get_effects_item`

```text
before   … │ item                 ◂ top
after    … │ list of identifiers  ◂ top
```

## Description

Pushes a list of status effects the food or potion will bestow.

## Related patterns (same book page)

- [[Diagnosis Purification]] `entity → list of identifiers`
- [[Condition Purification]] `identifier → number`
- [[Concentration Distillation]] `entity, identifier → number`
- [[Clearance Distillation]] `entity, identifier → number`

## Sources

- 📖 **Guidebook:** [[Status Effects]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Prescription Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/status_effects@hexpose:get_effects_item)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetPrescription` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_effects_item`
