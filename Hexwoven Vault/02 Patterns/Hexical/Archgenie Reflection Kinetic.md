---
aliases: ["Archgenie Reflection: Kinetic"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:get_arch_lamp_velocity"
signature: "qaqwddedqew"
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

# Archgenie Reflection: Kinetic

◇ **Pattern** · [[Hexical]] · 📖 [[Archgenie Lamps]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="75" height="170" viewBox="0 0 74.7 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.2" cy="18.0" r="2"/><circle cx="30.9" cy="18.0" r="2"/><circle cx="56.7" cy="18.0" r="2"/><circle cx="18.0" cy="40.4" r="2"/><circle cx="43.8" cy="40.4" r="2"/><circle cx="69.6" cy="40.4" r="2"/><circle cx="5.2" cy="62.7" r="2"/><circle cx="30.9" cy="62.7" r="2"/><circle cx="56.7" cy="62.7" r="2"/><circle cx="18.0" cy="85.0" r="2"/><circle cx="43.8" cy="85.0" r="2"/><circle cx="69.6" cy="85.0" r="2"/><circle cx="5.2" cy="107.3" r="2"/><circle cx="30.9" cy="107.3" r="2"/><circle cx="56.7" cy="107.3" r="2"/><circle cx="18.0" cy="129.6" r="2"/><circle cx="43.8" cy="129.6" r="2"/><circle cx="69.6" cy="129.6" r="2"/><circle cx="5.2" cy="152.0" r="2"/><circle cx="30.9" cy="152.0" r="2"/><circle cx="56.7" cy="152.0" r="2"/></g><polyline class="hexpat-base" points="30.9,62.7 43.8,40.4 30.9,18.0 18.0,40.4 30.9,62.7 43.8,85.0 18.0,85.0 30.9,62.7 56.7,62.7 43.8,85.0 56.7,107.3 43.8,129.6 30.9,152.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:3.4s" points="30.9,62.7 43.8,40.4 30.9,18.0 18.0,40.4 30.9,62.7 43.8,85.0 18.0,85.0 30.9,62.7 56.7,62.7 43.8,85.0 56.7,107.3 43.8,129.6 30.9,152.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.9" y1="62.7" x2="35.1" y2="55.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="38.0,50.4 37.5,57.0 32.6,54.1" fill="#fc77be"/><circle cx="30.9" cy="62.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwddedqew` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexical:get_arch_lamp_velocity`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Pushes my original velocity when the [[Archgenie Lamps|Archgenie Lamp]] started casting.

## Related patterns (same book page)

- [[Archgenie Purification]] `entity → boolean`
- [[Archgenie Reflection Spatial]] `— → vector`
- [[Archgenie Reflection Rotational]] `— → vector`
- [[Archgenie Reflection Temporal]] `— → number`
- [[Archgenie Reflection Media]] `— → number`
- [[Archgenie Gambit]] `any → —`
- [[Archgenie Reflection Memory]] `— → any`

## Sources

- 📖 **Guidebook:** [[Archgenie Lamps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Archgenie Reflection: Kinetic](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/arch_lamps@hexical:get_arch_lamp_velocity)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetArchLampData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_arch_lamp_velocity`
