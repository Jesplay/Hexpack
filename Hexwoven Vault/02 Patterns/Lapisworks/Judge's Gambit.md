---
aliases: ["Judge's Gambit"]
tags: [hexcasting/pattern, mod/lapisworks, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:set_thoth_hex"
signature: "qqdqq"
start_direction: "SOUTH_EAST"
input: "[pattern]"
output: ""
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Thoth's-affectors]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Judge's Gambit

◇ **Pattern** · [[Lapisworks]] · 📖 [[Thoth's-affectors]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="71" viewBox="0 0 170.0 71.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/></g><polyline class="hexpat-base" points="22.0,22.0 37.8,49.3 69.3,49.3 85.0,22.0 100.7,49.3 132.2,49.3 148.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:189;--dur:1.7s" points="22.0,22.0 37.8,49.3 69.3,49.3 85.0,22.0 100.7,49.3 132.2,49.3 148.0,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.0" y1="22.0" x2="27.1" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="30.7,37.0 24.1,32.5 30.1,29.0" fill="#fc77be"/><circle cx="22.0" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqdqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern]` ([[List]], [[Pattern]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:set_thoth_hex`

```text
before   … │ [pattern]  ◂ top
after    …              ◂ top
```

## Description

Pops a list of [[Iotas|iotas]] and sets the **instruction list** of the current Thoth's-like to that list.

## Related patterns (same book page)

- [[Queue Reflection]] `— → [any]`
- [[Queue Gambit]] `[any] → —`
- [[Executioner's Reflection]] `— → [pattern]`

## Sources

- 📖 **Guidebook:** [[Thoth's-affectors]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `SetThothHex` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:set_thoth_hex`
