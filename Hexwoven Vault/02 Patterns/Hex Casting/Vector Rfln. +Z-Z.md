---
aliases: ["Vector Rfln. +Z/-Z"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:const/vec/z"
signature: "qqqqqed"
start_direction: "NORTH_WEST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Constants]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Vector Rfln. +Z/-Z

◇ **Pattern** · [[Hex Casting]] · 📖 [[Constants]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5 123.6,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 143.0,60.5 123.6,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="60.5" x2="98.1" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="93.7,42.1 101.8,47.7 94.5,51.9" fill="#fc77be"/><circle cx="104.3" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqed` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:const/vec/z`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

The left-hand counter-clockwise pattern adds (0, 0, 1); the right-hand clockwise pattern adds (0, 0, -1).

## Related patterns (same book page)

- [[True Reflection]] `— → bool`
- [[False Reflection]] `— → bool`
- [[Nullary Reflection]] `— → null`
- [[Vector Reflection Zero]] `— → vector`
- [[Vector Rfln. +X-X]] `— → vector`
- [[Vector Rfln. +Y-Y]] `— → vector`
- [[Circle's Reflection]] `— → num`
- [[Arc's Reflection]] `— → num`
- [[Euler's Reflection]] `— → num`
- [[Harmony Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Constants]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Vector Rfln. +Z/-Z](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/consts@hexcasting:const/vec/z)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:const/vec/z`
