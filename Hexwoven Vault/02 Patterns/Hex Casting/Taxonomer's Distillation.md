---
aliases: ["Taxonomer's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:compare_entity"
signature: "aqaeqded"
start_direction: "NORTH_WEST"
input: "entity, entity"
output: "bool"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Types]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Taxonomer's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Types]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="59.5" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,42.1 63.2,47.7 55.8,51.9" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaeqded` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, entity` ([[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:compare_entity`

```text
before   … │ entity │ entity  ◂ top
after    … │ bool             ◂ top
```

## Description

Compare the given entities and push whether they are of the same type.

## Related patterns (same book page)

- [[Similarity Distillation (Hex Casting)]] `any, any → bool`
- [[Dissimilarity Distillation]] `any, any → bool`
- [[Sorter's Distillation]] `entity, entity → bool`
- [[Stacking Distillation]] `entity, entity → bool`
- [[Surveyor's Distillation]] `vec, vec → bool`
- [[Inspector's Distillation]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Types]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Taxonomer's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/types@hexcasting:compare_entity)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEntityEquality` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:compare_entity`
