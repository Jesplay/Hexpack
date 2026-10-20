---
aliases: ["Meterologist's Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:get_weather"
signature: "eweweweweweeeaedqdqde"
start_direction: "WEST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[World]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Meterologist's Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[World]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 69.3,103.8 85.0,76.6 53.5,76.6 37.8,49.3 69.3,49.3 85.0,22.0 100.7,49.3 132.2,49.3 116.5,76.6 85.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 69.3,103.8 85.0,76.6 53.5,76.6 37.8,49.3 69.3,49.3 85.0,22.0 100.7,49.3 132.2,49.3 116.5,76.6 85.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="131.1" x2="74.9" y2="131.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.7,131.1 74.9,127.6 74.9,134.6" fill="#fc77be"/><circle cx="85.0" cy="131.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eweweweweweeeaedqdqde` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_weather`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes a number corresponding to the weather. 0 for clear, 1 for rain, and 2 for thundering.

## Related patterns (same book page)

- [[Luminance Purification]] `vector → number`
- [[Battery Purification]] `vector → number`
- [[Peripheral Purification]] `vector → number/null`
- [[Circadian Reflection]] `— → number`
- [[Temporal Reflection]] `— → number`
- [[Geographical Purification]] `— → identifier`
- [[Plane Reflection]] `— → identifier`
- [[Lunar Reflection]] `— → number`
- [[Distortion Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[World]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Meterologist's Reflection](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/world@hexpose:get_weather)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetWorldData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_weather`
