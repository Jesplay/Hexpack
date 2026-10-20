---
aliases: ["Stacking Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:compare_item/strict"
signature: "qaeaqewqedqde"
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

# Stacking Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Types]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.6" cy="18.6" r="2"/><circle cx="45.2" cy="18.6" r="2"/><circle cx="71.7" cy="18.6" r="2"/><circle cx="98.3" cy="18.6" r="2"/><circle cx="124.8" cy="18.6" r="2"/><circle cx="151.4" cy="18.6" r="2"/><circle cx="5.3" cy="41.6" r="2"/><circle cx="31.9" cy="41.6" r="2"/><circle cx="58.4" cy="41.6" r="2"/><circle cx="85.0" cy="41.6" r="2"/><circle cx="111.6" cy="41.6" r="2"/><circle cx="138.1" cy="41.6" r="2"/><circle cx="164.7" cy="41.6" r="2"/><circle cx="18.6" cy="64.6" r="2"/><circle cx="45.2" cy="64.6" r="2"/><circle cx="71.7" cy="64.6" r="2"/><circle cx="98.3" cy="64.6" r="2"/><circle cx="124.8" cy="64.6" r="2"/><circle cx="151.4" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="58.4,41.6 45.2,18.6 18.6,18.6 31.9,41.6 18.6,64.6 45.2,64.6 58.4,41.6 85.0,41.6 111.6,41.6 124.8,18.6 151.4,18.6 138.1,41.6 151.4,64.6 124.8,64.6 111.6,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:372;--dur:3.9s" points="58.4,41.6 45.2,18.6 18.6,18.6 31.9,41.6 18.6,64.6 45.2,64.6 58.4,41.6 85.0,41.6 111.6,41.6 124.8,18.6 151.4,18.6 138.1,41.6 151.4,64.6 124.8,64.6 111.6,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.4" y1="41.6" x2="54.2" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.1,28.9 56.7,32.8 51.7,35.7" fill="#fc77be"/><circle cx="58.4" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqewqedqde` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, entity` ([[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:compare_item/strict`

```text
before   … │ entity │ entity  ◂ top
after    … │ bool             ◂ top
```

## Description

Like [[Sorter's Distillation]], but push whether they are identical items. Two [[Focus|Foci]] only compare as identical if they hold the same [[Iotas|iota]] and have [[Caster's Glamour (Book Entry)|the same appearance]].

## Related patterns (same book page)

- [[Similarity Distillation (Hex Casting)]] `any, any → bool`
- [[Dissimilarity Distillation]] `any, any → bool`
- [[Sorter's Distillation]] `entity, entity → bool`
- [[Taxonomer's Distillation]] `entity, entity → bool`
- [[Surveyor's Distillation]] `vec, vec → bool`
- [[Inspector's Distillation]] `vec, vec → bool`

## Sources

- 📖 **Guidebook:** [[Types]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Stacking Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/types@hexcasting:compare_item/strict)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpItemEquality` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:compare_item/strict`
