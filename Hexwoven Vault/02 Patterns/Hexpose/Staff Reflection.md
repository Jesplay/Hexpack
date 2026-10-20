---
aliases: ["Staff Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:env_staff"
signature: "waaq"
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

# Staff Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[Environment]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="38.0,132.0 65.1,85.0 92.3,38.0 38.0,38.0 65.1,85.0 119.4,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="132.0" x2="46.7" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,106.2 51.8,119.9 41.5,114.0" fill="#fc77be"/><circle cx="38.0" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:env_staff`

```text
before   …            ◂ top
after    … │ boolean  ◂ top
```

## Description

Pushes whether a [[Staves|Staff]] is being used to cast this pattern.

## Related patterns (same book page)

- [[Ambit Purification]] `entity/vector → boolean`
- [[Dexterity Reflection]] `— → boolean`
- [[Device Reflection]] `— → boolean`
- [[Constructed Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[Environment]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Staff Reflection](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/environment@hexpose:env_staff)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEnvData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:env_staff`
