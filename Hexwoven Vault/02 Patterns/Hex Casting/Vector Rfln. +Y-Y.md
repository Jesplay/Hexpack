---
aliases: ["Vector Rfln. +Y/-Y"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:const/vec/y"
signature: "qqqqqew"
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

# Vector Rfln. +Y/-Y

◇ **Pattern** · [[Hex Casting]] · 📖 [[Constants]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:252;--dur:2.2s" points="85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="80.0" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,34.3 83.0,38.8 77.0,42.3" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqew` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexcasting:const/vec/y`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

The left-hand counter-clockwise pattern adds (0, 1, 0) to the [[The Stack|stack]]; the right-hand clockwise pattern adds (0, -1, 0).

## Related patterns (same book page)

- [[True Reflection]] `— → bool`
- [[False Reflection]] `— → bool`
- [[Nullary Reflection]] `— → null`
- [[Vector Reflection Zero]] `— → vector`
- [[Vector Rfln. +X-X]] `— → vector`
- [[Vector Rfln. +Z-Z]] `— → vector`
- [[Circle's Reflection]] `— → num`
- [[Arc's Reflection]] `— → num`
- [[Euler's Reflection]] `— → num`
- [[Harmony Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Constants]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Vector Rfln. +Y/-Y](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/consts@hexcasting:const/vec/y)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:const/vec/y`
