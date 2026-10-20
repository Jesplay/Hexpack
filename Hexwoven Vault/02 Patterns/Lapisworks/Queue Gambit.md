---
aliases: ["Queue Gambit"]
tags: [hexcasting/pattern, mod/lapisworks, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:set_datum"
signature: "qwq"
start_direction: "SOUTH_EAST"
input: "[any]"
output: ""
cost: ""
iotas_in: ["[[List]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Thoth's-affectors]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Queue Gambit

◇ **Pattern** · [[Lapisworks]] · 📖 [[Thoth's-affectors]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="27.0,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:155;--dur:1.6s" points="27.0,27.0 46.4,60.5 85.0,60.5 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="27.0" x2="33.2" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,45.4 29.5,39.9 36.9,35.6" fill="#fc77be"/><circle cx="27.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[any]` ([[List]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:set_datum`

```text
before   … │ [any]  ◂ top
after    …          ◂ top
```

## Description

Pops a list of [[Iotas|iotas]] and sets the datum left to execute by the current Thoth's-like to that list.

## Related patterns (same book page)

- [[Queue Reflection]] `— → [any]`
- [[Executioner's Reflection]] `— → [pattern]`
- [[Judge's Gambit]] `[pattern] → —`

## Sources

- 📖 **Guidebook:** [[Thoth's-affectors]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `SetDatum` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:set_datum`
