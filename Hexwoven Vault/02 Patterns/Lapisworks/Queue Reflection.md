---
aliases: ["Queue Reflection"]
tags: [hexcasting/pattern, mod/lapisworks, family/reflection, topic/control-flow]
kind: "Pattern"
family: "Reflection"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:get_datum"
signature: "ewe"
start_direction: "NORTH_EAST"
input: ""
output: "[any]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Thoth's-affectors]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Queue Reflection

◇ **Pattern** · [[Lapisworks]] · 📖 [[Thoth's-affectors]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:155;--dur:1.6s" points="27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,42.1 36.9,51.9 29.5,47.7" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewe` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[any]` ([[List]], [[Any Iota]])
- **ID:** `lapisworks:get_datum`

```text
before   …          ◂ top
after    … │ [any]  ◂ top
```

## Description

Pushes a list of the datum left to iterate over by the current Thoth's-like.

## Related patterns (same book page)

- [[Queue Gambit]] `[any] → —`
- [[Executioner's Reflection]] `— → [pattern]`
- [[Judge's Gambit]] `[pattern] → —`

## Sources

- 📖 **Guidebook:** [[Thoth's-affectors]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetDatum` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_datum`
