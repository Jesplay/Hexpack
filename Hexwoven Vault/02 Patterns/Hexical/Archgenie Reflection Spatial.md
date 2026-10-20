---
aliases: ["Archgenie Reflection: Spatial"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:get_arch_lamp_position"
signature: "qaqwddedqdd"
start_direction: "NORTH_EAST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Archgenie Lamps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Archgenie Reflection: Spatial

◇ **Pattern** · [[Hexical]] · 📖 [[Archgenie Lamps]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 101.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,85.0 59.4,54.7 41.9,24.5 24.5,54.7 41.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 59.4,115.3 76.9,145.5 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="41.9,85.0 59.4,54.7 41.9,24.5 24.5,54.7 41.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 59.4,115.3 76.9,145.5 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="85.0" x2="47.5" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,68.4 50.9,77.2 44.2,73.4" fill="#fc77be"/><circle cx="41.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwddedqdd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexical:get_arch_lamp_position`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Pushes my original position when the [[Archgenie Lamps|Archgenie Lamp]] started casting.

## Related patterns (same book page)

- [[Archgenie Purification]] `entity → boolean`
- [[Archgenie Reflection Rotational]] `— → vector`
- [[Archgenie Reflection Kinetic]] `— → vector`
- [[Archgenie Reflection Temporal]] `— → number`
- [[Archgenie Reflection Media]] `— → number`
- [[Archgenie Gambit]] `any → —`
- [[Archgenie Reflection Memory]] `— → any`

## Sources

- 📖 **Guidebook:** [[Archgenie Lamps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Archgenie Reflection: Spatial](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/arch_lamps@hexical:get_arch_lamp_position)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetArchLampData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_arch_lamp_position`
