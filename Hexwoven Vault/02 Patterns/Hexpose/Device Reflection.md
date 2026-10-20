---
aliases: ["Device Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:env_packaged_hex"
signature: "waaqwwaqqqqq"
start_direction: "NORTH_EAST"
input: ""
output: "boolean"
cost: ""
iotas_in: []
iotas_out: ["[[Boolean]]"]
entry: "[[Environment]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Device Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[Environment]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,84.4 41.6,54.3 59.0,24.3 24.3,24.3 41.6,54.3 76.3,54.3 111.0,54.3 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:451;--dur:3.6s" points="24.3,84.4 41.6,54.3 59.0,24.3 24.3,24.3 41.6,54.3 76.3,54.3 111.0,54.3 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="84.4" x2="29.8" y2="74.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="33.8,67.9 33.1,76.7 26.5,72.9" fill="#fc77be"/><circle cx="24.3" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waaqwwaqqqqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:env_packaged_hex`

```text
before   …            ◂ top
after    … │ boolean  ◂ top
```

## Description

Pushes whether any form of [[Casting Items|casting device]] is being used to cast this pattern.

## Related patterns (same book page)

- [[Ambit Purification]] `entity/vector → boolean`
- [[Staff Reflection]] `— → boolean`
- [[Dexterity Reflection]] `— → boolean`
- [[Constructed Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[Environment]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Device Reflection](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/environment@hexpose:env_packaged_hex)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEnvData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:env_packaged_hex`
