---
aliases: ["Clearance Distillation"]
tags: [hexcasting/pattern, mod/hexpose, family/distillation, topic/sensing, topic/effects]
kind: "Pattern"
family: "Distillation"
topics: ["Sensing & Raycasts", "Status Effects"]
mod: "Hexpose"
op_id: "hexpose:get_effect_duration"
signature: "wqqqaqwdd"
start_direction: "SOUTH_WEST"
input: "entity, identifier"
output: "number"
cost: ""
iotas_in: ["[[Entity]]", "[[Identifier]]"]
iotas_out: ["[[Number]]"]
entry: "[[Status Effects]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Clearance Distillation

◇ **Pattern** · [[Hexpose]] · 📖 [[Status Effects]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,24.5 59.4,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 76.9,85.0 59.4,115.3 41.9,145.5 24.5,115.3 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="76.9,24.5 59.4,54.7 41.9,85.0 59.4,115.3 94.4,115.3 111.8,85.0 76.9,85.0 59.4,115.3 41.9,145.5 24.5,115.3 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="24.5" x2="71.3" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,41.1 68.0,32.2 74.6,36.1" fill="#fc77be"/><circle cx="76.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqqqaqwdd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, identifier` ([[Entity]], [[Identifier]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_effect_duration`

```text
before   … │ entity │ identifier  ◂ top
after    … │ number               ◂ top
```

## Description

Pushes how long the effect type will affect the entity in twentieths of a second or Null if the entity doesn't have it.

## Related patterns (same book page)

- [[Diagnosis Purification]] `entity → list of identifiers`
- [[Prescription Purification]] `item → list of identifiers`
- [[Condition Purification]] `identifier → number`
- [[Concentration Distillation]] `entity, identifier → number`

## Sources

- 📖 **Guidebook:** [[Status Effects]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Clearance Distillation](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/status_effects@hexpose:get_effect_duration)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetStatusEffectInstanceData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_effect_duration`
