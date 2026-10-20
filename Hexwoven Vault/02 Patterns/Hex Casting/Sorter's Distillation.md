---
aliases: ["Sorter's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:compare_item/lenient"
signature: "qaeaqeqedqde"
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

# Sorter's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Types]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 116.5,22.0 148.0,22.0 132.2,49.3 148.0,76.6 116.5,76.6 100.7,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:409;--dur:3.6s" points="69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 116.5,22.0 148.0,22.0 132.2,49.3 148.0,76.6 116.5,76.6 100.7,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="49.3" x2="64.2" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,34.3 67.2,38.8 61.2,42.3" fill="#fc77be"/><circle cx="69.3" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqeqedqde` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, entity` ([[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:compare_item/lenient`

```text
before   … │ entity │ entity  ◂ top
after    … │ bool             ◂ top
```

## Description

Compare the items represented or held by the given entities and push whether they are the same type of item. Two [[Focus|Foci]] will always compare as equal.

## Related patterns (same book page)

- [[Similarity Distillation (Hex Casting)]] `any, any → bool`
- [[Dissimilarity Distillation]] `any, any → bool`
- [[Stacking Distillation]] `entity, entity → bool`
- [[Taxonomer's Distillation]] `entity, entity → bool`
- [[Surveyor's Distillation]] `vec, vec → bool`
- [[Inspector's Distillation]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Types]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sorter's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/types@hexcasting:compare_item/lenient)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpItemEquality` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:compare_item/lenient`
