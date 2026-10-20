---
aliases: ["Wristpocket Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/items]
kind: "Pattern"
family: "Reflection"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:wristpocket_item"
signature: "aaqqada"
start_direction: "WEST"
input: ""
output: "item"
cost: ""
iotas_in: []
iotas_out: ["[[Item Stack]]"]
entry: "[[Wristpocket (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Wristpocket Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Wristpocket (Book Entry)]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="135.0,78.3 85.0,78.3 110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 85.0,78.3 35.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="135.0,78.3 85.0,78.3 110.0,121.6 135.0,78.3 110.0,35.0 60.0,35.0 85.0,78.3 35.0,78.3 60.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="78.3" x2="119.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.5,78.3 119.0,72.8 119.0,83.8" fill="#fc77be"/><circle cx="135.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqqada` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `item` ([[Item Stack]])
- **ID:** `hexical:wristpocket_item`

```text
before   …         ◂ top
after    … │ item  ◂ top
```

## Description

Returns the item in my wristpocket, or Null if it is empty.

## Related patterns (same book page)

- [[Wristpocket]] `— → —`
- [[Sleight]] `item entity/vector → —`
- [[Mage Hand]] `entity/vector → —`
- [[Mage Mouth]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wristpocket (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Wristpocket Reflection](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/wristpocket@hexical:wristpocket_item)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetWristpocket` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:wristpocket_item`
