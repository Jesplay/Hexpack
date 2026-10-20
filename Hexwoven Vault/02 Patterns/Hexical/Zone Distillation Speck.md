---
aliases: ["Zone Distillation: Speck"]
tags: [hexcasting/pattern, mod/hexical, family/distillation, topic/visuals]
kind: "Pattern"
family: "Distillation"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:zone_specklike"
signature: "qqqqqwdeddwqde"
start_direction: "SOUTH_EAST"
input: "vector, number"
output: "list of entities"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Entity]]", "[[List]]"]
entry: "[[Specks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Zone Distillation: Speck

◇ **Pattern** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="98.3,41.6 111.6,64.6 138.1,64.6 151.4,41.6 138.1,18.6 111.6,18.6 98.3,41.6 85.0,64.6 71.7,41.6 85.0,18.6 98.3,41.6 71.7,41.6 45.2,41.6 31.9,64.6 18.6,41.6 31.9,18.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:398;--dur:4.2s" points="98.3,41.6 111.6,64.6 138.1,64.6 151.4,41.6 138.1,18.6 111.6,18.6 98.3,41.6 85.0,64.6 71.7,41.6 85.0,18.6 98.3,41.6 71.7,41.6 45.2,41.6 31.9,64.6 18.6,41.6 31.9,18.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="98.3" y1="41.6" x2="102.5" y2="49.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.6,54.2 100.0,50.4 105.1,47.5" fill="#fc77be"/><circle cx="98.3" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqwdeddwqde` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, number` ([[Number]], [[Vector]])
- **Leaves:** `list of entities` ([[Entity]], [[List]])
- **ID:** `hexical:zone_specklike`

```text
before   … │ vector │ number   ◂ top
after    … │ list of entities  ◂ top
```

## Description

Returns specks similarly to other [[Zone Distillation Animal|zone distillation]] patterns.

## Related patterns (same book page)

- [[Conjure Speck]] `any, vector, vector → entity`
- [[Move Speck]] `speck entity, vector → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Alter Speck]] `speck entity, any → —`
- [[Time Speck]] `speck entity, number → —`
- [[Resize Speck]] `speck entity, number → —`
- [[Thicken Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Zone Distillation: Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:zone_specklike)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntitiesBy` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:zone_specklike`
