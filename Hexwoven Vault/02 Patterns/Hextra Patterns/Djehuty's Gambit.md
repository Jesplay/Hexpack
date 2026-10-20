---
aliases: ["Djehuty's Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hextra Patterns"
op_id: "hextrapats:index_for_each"
signature: "dadaddqdq"
start_direction: "NORTH_EAST"
input: "[pattern], list"
output: "list"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: ["[[List]]"]
entry: "[[Hextra Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Djehuty's Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 104.3,60.5 85.0,94.0 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="27.0,60.5 46.4,27.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,27.0 143.0,60.5 104.3,60.5 85.0,94.0 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,42.1 36.9,51.9 29.5,47.7" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dadaddqdq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern], list` ([[List]], [[Pattern]])
- **Leaves:** `list` ([[List]])
- **ID:** `hextrapats:index_for_each`

```text
before   … │ [pattern] │ list  ◂ top
after    … │ list              ◂ top
```

## Description

Casts like [[Thoth's Gambit]], however the new [[The Stack|stack]] for each element additionally has the index of that element added to the top.

## Related patterns (same book page)

- [[Helios' Gambit]] `any, ([pattern] | pattern) → many`
- [[Selene's Gambit]] `any, ([pattern] | pattern) → many`
- [[Acheron's Gambit]] `any → —`
- [[Styx's Gambit]] `any → —`
- [[Anubis' Gambit]] `many, [pattern], list → many`
- [[Ra's Gambit]] `many, [pattern], list → many`

## Sources

- 📖 **Guidebook:** [[Hextra Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpForEachIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:index_for_each`
