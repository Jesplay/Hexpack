---
aliases: ["Seshat's Reflection"]
tags: [hexcasting/pattern, mod/ephemera, family/reflection, topic/control-flow]
kind: "Pattern"
family: "Reflection"
topics: ["Control Flow & Meta-evaluation"]
mod: "Ephemera"
op_id: "ephemera:thothcount"
signature: "wqaqwadad"
start_direction: "NORTH_EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Accumulator Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Seshat's Reflection

◇ **Pattern** · [[Ephemera]] · 📖 [[Accumulator Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,127.4 46.4,94.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,127.4 85.0,94.0 104.3,127.4 123.6,94.0 143.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="27.0,127.4 46.4,94.0 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,127.4 85.0,94.0 104.3,127.4 123.6,94.0 143.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="127.4" x2="33.2" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,109.0 36.9,118.8 29.5,114.6" fill="#fc77be"/><circle cx="27.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaqwadad` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `ephemera:thothcount`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Returns how many [[Iotas|iotas]] are currently accessible via [[Seshat's Gambit]]. Returns -1 if used outside of [[Thoth's Gambit]].

## Related patterns (same book page)

- [[Seshat's Gambit]] `— → any | null`

## Sources

- 📖 **Guidebook:** [[Accumulator Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Seshat's Reflection](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/metathoth@ephemera:thothcount)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpThothCount` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:thothcount`
