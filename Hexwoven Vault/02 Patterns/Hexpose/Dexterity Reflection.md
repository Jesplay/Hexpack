---
aliases: ["Dexterity Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:env_offhand"
signature: "qaqqqwaaq"
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

# Dexterity Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[Environment]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 123.6,27.0 85.0,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="46.4,94.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 123.6,27.0 85.0,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="52.5" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,75.6 56.2,85.4 48.9,81.1" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqwaaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:env_offhand`

```text
before   …            ◂ top
after    … │ boolean  ◂ top
```

## Description

Pushes whether the dominant hand of the caster is being used to cast this pattern. May return strange results for casters without hands.

## Related patterns (same book page)

- [[Ambit Purification]] `entity/vector → boolean`
- [[Staff Reflection]] `— → boolean`
- [[Device Reflection]] `— → boolean`
- [[Constructed Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[Environment]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Dexterity Reflection](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/environment@hexpose:env_offhand)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEnvData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:env_offhand`
