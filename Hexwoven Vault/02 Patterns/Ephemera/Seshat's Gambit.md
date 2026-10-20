---
aliases: ["Seshat's Gambit"]
tags: [hexcasting/pattern, mod/ephemera, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Ephemera"
op_id: "ephemera:thothyoink"
signature: "qaeaqdadad"
start_direction: "NORTH_WEST"
input: ""
output: "any | null"
cost: ""
iotas_in: []
iotas_out: ["[[Null]]", "[[Any Iota]]"]
entry: "[[Accumulator Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Seshat's Gambit

◇ **Pattern** · [[Ephemera]] · 📖 [[Accumulator Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 85.0,76.6 100.7,49.3 116.5,76.6 132.2,49.3 148.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:346;--dur:3.1s" points="69.3,49.3 53.5,22.0 22.0,22.0 37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 85.0,76.6 100.7,49.3 116.5,76.6 132.2,49.3 148.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="49.3" x2="64.2" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,34.3 67.2,38.8 61.2,42.3" fill="#fc77be"/><circle cx="69.3" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaeaqdadad` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `any | null` ([[Null]], [[Any Iota]])
- **ID:** `ephemera:thothyoink`

## Description

Pops an [[Iotas|iota]] from the accumulator and pushes it to the current [[The Stack|stack]]. [[Mishaps|Mishaps]] if used outside of [[Thoth's Gambit]].

## Related patterns (same book page)

- [[Seshat's Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Accumulator Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Seshat's Gambit](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/metathoth@ephemera:thothyoink)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpThothYoink` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:thothyoink`
