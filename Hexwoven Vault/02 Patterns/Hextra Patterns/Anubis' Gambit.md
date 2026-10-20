---
aliases: ["Anubis' Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hextra Patterns"
op_id: "hextrapats:main_for_each"
signature: "aawdadad"
start_direction: "WEST"
input: "many, [pattern], list"
output: "many"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Hextra Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Anubis' Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="53.5,49.3 22.0,49.3 37.8,76.6 53.5,49.3 69.3,22.0 85.0,49.3 100.7,22.0 116.5,49.3 132.2,22.0 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:283;--dur:2.5s" points="53.5,49.3 22.0,49.3 37.8,76.6 53.5,49.3 69.3,22.0 85.0,49.3 100.7,22.0 116.5,49.3 132.2,22.0 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="49.3" x2="43.4" y2="49.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="36.2,49.3 43.4,45.8 43.4,52.8" fill="#fc77be"/><circle cx="53.5" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawdadad` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many, [pattern], list` ([[List]], [[Pattern]])
- **Leaves:** `many` (—)
- **ID:** `hextrapats:main_for_each`

## Description

Casts like [[Thoth's Gambit]], however each sub-execution operates on the main [[The Stack|stack]] instead of copies of the stack.

## Related patterns (same book page)

- [[Helios' Gambit]] `any, ([pattern] | pattern) → many`
- [[Selene's Gambit]] `any, ([pattern] | pattern) → many`
- [[Acheron's Gambit]] `any → —`
- [[Styx's Gambit]] `any → —`
- [[Djehuty's Gambit]] `[pattern], list → list`
- [[Ra's Gambit]] `many, [pattern], list → many`

## Sources

- 📖 **Guidebook:** [[Hextra Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMainForEach` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:main_for_each`
