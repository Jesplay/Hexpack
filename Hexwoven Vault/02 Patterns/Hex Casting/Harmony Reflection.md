---
aliases: ["Harmony Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:const/double/phi"
signature: "wdded"
start_direction: "NORTH_EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Constants]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Harmony Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Constants]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,132.0 65.1,85.0 92.3,38.0 119.4,85.0 65.1,85.0 38.0,38.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="38.0,132.0 65.1,85.0 92.3,38.0 119.4,85.0 65.1,85.0 38.0,38.0 92.3,38.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="132.0" x2="46.7" y2="117.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,106.2 51.8,119.9 41.5,114.0" fill="#fc77be"/><circle cx="38.0" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdded` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:const/double/phi`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Adds φ, the golden ratio, to the [[The Stack|stack]]. This is the ratio between two values such that (A+B)/A = A/B.

## Related patterns (same book page)

- [[True Reflection]] `— → bool`
- [[False Reflection]] `— → bool`
- [[Nullary Reflection]] `— → null`
- [[Vector Reflection Zero]] `— → vector`
- [[Vector Rfln. +X-X]] `— → vector`
- [[Vector Rfln. +Y-Y]] `— → vector`
- [[Vector Rfln. +Z-Z]] `— → vector`
- [[Circle's Reflection]] `— → num`
- [[Arc's Reflection]] `— → num`
- [[Euler's Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Constants]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Harmony Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/consts@hexcasting:const/double/phi)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:const/double/phi`
