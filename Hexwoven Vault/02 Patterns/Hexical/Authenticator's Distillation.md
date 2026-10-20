---
aliases: ["Authenticator's Distillation"]
tags: [hexcasting/pattern, mod/hexical, family/distillation, topic/items]
kind: "Pattern"
family: "Distillation"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:has_autograph"
signature: "wwqqqqqaw"
start_direction: "NORTH_EAST"
input: "item stack, entity"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]", "[[Item Stack]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Autographs]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Authenticator's Distillation

◇ **Pattern** · [[Hexical]] · 📖 [[Autographs]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,145.5 59.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 76.9,85.0 59.4,54.7 41.9,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="41.9,145.5 59.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 41.9,24.5 24.5,54.7 41.9,85.0 76.9,85.0 59.4,54.7 41.9,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="145.5" x2="47.5" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,128.9 50.9,137.8 44.2,133.9" fill="#fc77be"/><circle cx="41.9" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwqqqqqaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item stack, entity` ([[Entity]], [[Item Stack]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexical:has_autograph`

```text
before   … │ item stack │ entity  ◂ top
after    … │ boolean              ◂ top
```

## Description

Pushes whether a given player has signed a given item [[The Stack|stack]].

## Related patterns (same book page)

- [[Autograph]] `— → —`
- [[Unautograph]] `— → —`

## Sources

- 📖 **Guidebook:** [[Autographs]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Authenticator's Distillation](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/autographs@hexical:has_autograph)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpHasAutograph` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:has_autograph`
