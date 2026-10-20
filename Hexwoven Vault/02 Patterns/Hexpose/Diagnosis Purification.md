---
aliases: ["Diagnosis Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing, topic/effects]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "Status Effects"]
mod: "Hexpose"
op_id: "hexpose:get_effects_entity"
signature: "wqqq"
start_direction: "SOUTH_WEST"
input: "entity"
output: "list of identifiers"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[List]]", "[[Identifier]]"]
entry: "[[Status Effects]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Diagnosis Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Status Effects]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,29.8 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="72.3,29.8 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="29.8" x2="65.5" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,50.0 61.4,39.2 69.5,43.9" fill="#fc77be"/><circle cx="72.3" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `list of identifiers` ([[List]], [[Identifier]])
- **ID:** `hexpose:get_effects_entity`

```text
before   … │ entity               ◂ top
after    … │ list of identifiers  ◂ top
```

## Description

Pushes a list of status effects the entity is afflicted with.

## Related patterns (same book page)

- [[Prescription Purification]] `item → list of identifiers`
- [[Condition Purification]] `identifier → number`
- [[Concentration Distillation]] `entity, identifier → number`
- [[Clearance Distillation]] `entity, identifier → number`

## Sources

- 📖 **Guidebook:** [[Status Effects]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Diagnosis Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/status_effects@hexpose:get_effects_entity)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetLivingEntityData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_effects_entity`
