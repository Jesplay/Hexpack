---
aliases: ["Genie Reflection: Rotational"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Casting Devices"]
mod: "Hexical"
op_id: "hexical:get_hand_lamp_rotation"
signature: "qwddedadw"
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

# Genie Reflection: Rotational

◇ **Pattern** · [[Hexical]] · 📖 [[Hand Lamps]] · Genie Lamp

**Topics:** [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="101" height="170" viewBox="0 0 100.9 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="35.6,20.8 20.8,46.5 35.6,72.2 50.4,97.8 20.8,97.8 35.6,72.2 65.3,72.2 50.4,97.8 80.1,97.8 65.3,123.5 50.4,149.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:297;--dur:2.8s" points="35.6,20.8 20.8,46.5 35.6,72.2 50.4,97.8 20.8,97.8 35.6,72.2 65.3,72.2 50.4,97.8 80.1,97.8 65.3,123.5 50.4,149.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.6" y1="20.8" x2="30.9" y2="29.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="27.4,34.9 28.0,27.4 33.7,30.6" fill="#fc77be"/><circle cx="35.6" cy="20.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwddedadw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexical:get_hand_lamp_rotation`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Pushes my original rotation when I began using the **Hand Lamp**.

## Related patterns (same book page)

- [[Wish]] `list of patterns → —`
- [[Genie Reflection Spatial]] `— → vector`
- [[Genie Reflection Kinetic]] `— → vector`
- [[Genie Reflection Temporal]] `— → number`
- [[Genie Reflection Media]] `— → number`
- [[Genie Gambit]] `any → —`
- [[Genie Reflection Memory]] `— → any`
- [[Finale Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[Hand Lamps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Genie Reflection: Rotational](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#lamp/hand_lamps@hexical:get_hand_lamp_rotation)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetHandLampData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:get_hand_lamp_rotation`
