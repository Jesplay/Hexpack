---
aliases: ["Circle's Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:const/double/tau"
signature: "eawae"
start_direction: "NORTH_WEST"
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

# Circle's Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Constants]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="135.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="135.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3 35.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="121.6" x2="127.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="121.2,97.8 131.8,105.0 122.2,110.5" fill="#fc77be"/><circle cx="135.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eawae` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:const/double/tau`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Adds τ, the radial representation of a complete circle, to the [[The Stack|stack]].

## Related patterns (same book page)

- [[True Reflection]] `— → bool`
- [[False Reflection]] `— → bool`
- [[Nullary Reflection]] `— → null`
- [[Vector Reflection Zero]] `— → vector`
- [[Vector Rfln. +X-X]] `— → vector`
- [[Vector Rfln. +Y-Y]] `— → vector`
- [[Vector Rfln. +Z-Z]] `— → vector`
- [[Arc's Reflection]] `— → num`
- [[Euler's Reflection]] `— → num`
- [[Harmony Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Constants]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Circle's Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/consts@hexcasting:const/double/tau)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:const/double/tau`
