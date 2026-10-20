---
aliases: ["Genie Reflection: Spatial"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:get_hand_lamp_position"
signature: "qwddedqdd"
start_direction: "SOUTH_WEST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Hand Lamps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Genie Reflection: Spatial

◇ **Pattern** · [[Hexical]] · 📖 [[Hand Lamps]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 101.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,24.5 24.5,54.7 41.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 59.4,115.3 76.9,145.5 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="41.9,24.5 24.5,54.7 41.9,85.0 59.4,115.3 24.5,115.3 41.9,85.0 76.9,85.0 59.4,115.3 76.9,145.5 41.9,145.5 59.4,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="24.5" x2="36.3" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="32.3,41.1 33.0,32.2 39.7,36.1" fill="#fc77be"/><circle cx="41.9" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwddedqdd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexical:get_hand_lamp_position`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Pushes my original position when I began using the **Hand Lamp**.

## Related patterns (same book page)

- [[Wish]] `list of patterns → —`
- [[Genie Reflection Rotational]] `— → vector`
- [[Genie Reflection Kinetic]] `— → vector`
- [[Genie Reflection Temporal]] `— → number`
- [[Genie Reflection Media]] `— → number`
- [[Genie Gambit]] `any → —`
- [[Genie Reflection Memory]] `— → any`
- [[Finale Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[Hand Lamps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Genie Reflection: Spatial](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/hand_lamps@hexical:get_hand_lamp_position)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetHandLampData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_hand_lamp_position`
