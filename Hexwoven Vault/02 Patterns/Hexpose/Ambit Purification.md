---
aliases: ["Ambit Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:env_ambit"
signature: "wawaw"
start_direction: "EAST"
input: "entity/vector"
output: "boolean"
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Environment]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Ambit Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Environment]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,121.6 85.0,121.6 135.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="35.0,121.6 85.0,121.6 135.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="121.6" x2="51.0" y2="121.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,121.6 51.0,127.1 51.0,116.1" fill="#fc77be"/><circle cx="35.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity/vector` ([[Vector]], [[Entity]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:env_ambit`

```text
before   … │ entity/vector  ◂ top
after    … │ boolean        ◂ top
```

## Description

Pushes whether a location or entity is in ambit.

## Related patterns (same book page)

- [[Staff Reflection]] `— → boolean`
- [[Dexterity Reflection]] `— → boolean`
- [[Device Reflection]] `— → boolean`
- [[Constructed Reflection]] `— → boolean`

## Sources

- 📖 **Guidebook:** [[Environment]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Ambit Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/environment@hexpose:env_ambit)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetAmbit` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:env_ambit`
