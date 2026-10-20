---
aliases: ["Inspector's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:compare_block/strict"
signature: "qwawqwadadwewdwe"
start_direction: "NORTH_WEST"
input: "vec, vec"
output: "bool"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Types]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Inspector's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Types]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="72" viewBox="0 0 170.0 72.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.1" cy="16.1" r="2"/><circle cx="39.1" cy="16.1" r="2"/><circle cx="62.0" cy="16.1" r="2"/><circle cx="85.0" cy="16.1" r="2"/><circle cx="108.0" cy="16.1" r="2"/><circle cx="130.9" cy="16.1" r="2"/><circle cx="153.9" cy="16.1" r="2"/><circle cx="4.6" cy="36.0" r="2"/><circle cx="27.6" cy="36.0" r="2"/><circle cx="50.5" cy="36.0" r="2"/><circle cx="73.5" cy="36.0" r="2"/><circle cx="96.5" cy="36.0" r="2"/><circle cx="119.5" cy="36.0" r="2"/><circle cx="142.4" cy="36.0" r="2"/><circle cx="165.4" cy="36.0" r="2"/><circle cx="16.1" cy="55.9" r="2"/><circle cx="39.1" cy="55.9" r="2"/><circle cx="62.0" cy="55.9" r="2"/><circle cx="85.0" cy="55.9" r="2"/><circle cx="108.0" cy="55.9" r="2"/><circle cx="130.9" cy="55.9" r="2"/><circle cx="153.9" cy="55.9" r="2"/></g><polyline class="hexpat-base" points="73.5,36.0 62.0,16.1 39.1,16.1 16.1,16.1 27.6,36.0 39.1,55.9 62.0,55.9 85.0,55.9 73.5,36.0 96.5,36.0 85.0,16.1 108.0,16.1 130.9,16.1 142.4,36.0 153.9,55.9 130.9,55.9 108.0,55.9 96.5,36.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:391;--dur:4.8s" points="73.5,36.0 62.0,16.1 39.1,16.1 16.1,16.1 27.6,36.0 39.1,55.9 62.0,55.9 85.0,55.9 73.5,36.0 96.5,36.0 85.0,16.1 108.0,16.1 130.9,16.1 142.4,36.0 153.9,55.9 130.9,55.9 108.0,55.9 96.5,36.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.5" y1="36.0" x2="69.8" y2="29.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.2,25.0 72.0,28.3 67.6,30.9" fill="#fc77be"/><circle cx="73.5" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwawqwadadwewdwe` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec` ([[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:compare_block/strict`

```text
before   … │ vec │ vec  ◂ top
after    … │ bool       ◂ top
```

## Description

Compare the blocks specified by the given vectors and push whether they appear to be identical. Two placed [[Edified Trees|Edified Stairs]] will only compare as equal if they appear exactly the same.

Nature seems a bit arbitrary as to when blocks appear identical to each other. Placed [[Slates]] appear identical if they face the same direction and share waterlogging state, even if they have different patterns written to them. Similarly, chests appear identical if their outward appearance is the same, even if their contents differ.

## Related patterns (same book page)

- [[Similarity Distillation (Hex Casting)]] `any, any → bool`
- [[Dissimilarity Distillation]] `any, any → bool`
- [[Sorter's Distillation]] `entity, entity → bool`
- [[Stacking Distillation]] `entity, entity → bool`
- [[Taxonomer's Distillation]] `entity, entity → bool`
- [[Surveyor's Distillation]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Types]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Inspector's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/types@hexcasting:compare_block/strict)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpBlockEquality` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:compare_block/strict`
