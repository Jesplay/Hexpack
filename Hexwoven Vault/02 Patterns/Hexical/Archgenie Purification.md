---
aliases: ["Archgenie Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/devices]
kind: "Pattern"
family: "Purification"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:has_arch_lamp"
signature: "qaqwddedqeed"
start_direction: "NORTH_EAST"
input: "entity"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Archgenie Lamps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Archgenie Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Archgenie Lamps]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="86" height="170" viewBox="0 0 86.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="35.6,72.2 50.4,46.5 35.6,20.8 20.8,46.5 35.6,72.2 50.4,97.8 20.8,97.8 35.6,72.2 65.3,72.2 50.4,97.8 65.3,123.5 50.4,149.2 20.8,149.2 35.6,123.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:3.6s" points="35.6,72.2 50.4,46.5 35.6,20.8 20.8,46.5 35.6,72.2 50.4,97.8 20.8,97.8 35.6,72.2 65.3,72.2 50.4,97.8 65.3,123.5 50.4,149.2 20.8,149.2 35.6,123.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.6" y1="72.2" x2="40.3" y2="63.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.8,58.0 43.2,65.6 37.5,62.3" fill="#fc77be"/><circle cx="35.6" cy="72.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwddedqeed` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexical:has_arch_lamp`

```text
before   … │ entity   ◂ top
after    … │ boolean  ◂ top
```

## Description

Pushes whether a player currently has an active [[Archgenie Lamps|Archgenie Lamp]].

## Related patterns (same book page)

- [[Archgenie Reflection Spatial]] `— → vector`
- [[Archgenie Reflection Rotational]] `— → vector`
- [[Archgenie Reflection Kinetic]] `— → vector`
- [[Archgenie Reflection Temporal]] `— → number`
- [[Archgenie Reflection Media]] `— → number`
- [[Archgenie Gambit]] `any → —`
- [[Archgenie Reflection Memory]] `— → any`

## Sources

- 📖 **Guidebook:** [[Archgenie Lamps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Archgenie Purification](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/arch_lamps@hexical:has_arch_lamp)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpIsUsingArchLamp` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:has_arch_lamp`
