---
aliases: ["Gulliver's Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/effects]
kind: "Pattern"
family: "Purification"
topics: ["Status Effects"]
mod: "Hex Casting"
op_id: "hexcasting:interop/pehkui/get"
signature: "aawawwawwa"
start_direction: "NORTH_WEST"
input: "entity"
output: "num"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Pehkui]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gulliver's Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Pehkui]] · Cross-Mod Compatibility

**Topics:** [[Status Effects (Topic)|Status Effects]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="104.3,127.4 85.0,94.0 65.7,127.4 104.3,127.4 143.0,127.4 123.6,94.0 104.3,60.5 85.0,27.0 65.7,60.5 46.4,94.0 27.0,127.4 65.7,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="104.3,127.4 85.0,94.0 65.7,127.4 104.3,127.4 143.0,127.4 123.6,94.0 104.3,60.5 85.0,27.0 65.7,60.5 46.4,94.0 27.0,127.4 65.7,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="127.4" x2="98.1" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,109.0 101.8,114.6 94.5,118.8" fill="#fc77be"/><circle cx="104.3" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawawwawwa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:interop/pehkui/get`

```text
before   … │ entity  ◂ top
after    … │ num     ◂ top
```

## Description

Get the scale of the entity, as a proportion of their normal size. For most entities, this will be 1.

## Related patterns (same book page)

- [[Alter Scale]] `entity, num → —`

## Sources

- 📖 **Guidebook:** [[Pehkui]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Gulliver's Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#interop/pehkui@hexcasting:interop/pehkui/get)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetScale` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:interop/pehkui/get`
