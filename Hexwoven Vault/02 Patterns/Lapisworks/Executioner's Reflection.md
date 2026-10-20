---
aliases: ["Executioner's Reflection"]
tags: [hexcasting/pattern, mod/lapisworks, family/reflection, topic/control-flow]
kind: "Pattern"
family: "Reflection"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:get_thoth_hex"
signature: "eeaee"
start_direction: "NORTH_EAST"
input: ""
output: "[pattern]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Pattern]]"]
entry: "[[Thoth's-affectors]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Executioner's Reflection

◇ **Pattern** · [[Lapisworks]] · 📖 [[Thoth's-affectors]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="71" viewBox="0 0 170.0 71.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/></g><polyline class="hexpat-base" points="22.0,49.3 37.8,22.0 69.3,22.0 85.0,49.3 100.7,22.0 132.2,22.0 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:189;--dur:1.7s" points="22.0,49.3 37.8,22.0 69.3,22.0 85.0,49.3 100.7,22.0 132.2,22.0 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="49.3" x2="27.1" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,34.3 30.1,42.3 24.1,38.8" fill="#fc77be"/><circle cx="22.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeaee` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[pattern]` ([[List]], [[Pattern]])
- **ID:** `lapisworks:get_thoth_hex`

```text
before   …              ◂ top
after    … │ [pattern]  ◂ top
```

## Description

Pushes the **instruction list** the current Thoth's-like is executing.

## Related patterns (same book page)

- [[Queue Reflection]] `— → [any]`
- [[Queue Gambit]] `[any] → —`
- [[Judge's Gambit]] `[pattern] → —`

## Sources

- 📖 **Guidebook:** [[Thoth's-affectors]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetThothHex` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_thoth_hex`
