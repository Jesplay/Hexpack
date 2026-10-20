---
aliases: ["Prometheus' Gambit"]
tags: [hexcasting/pattern, mod/caduceus, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:eval/goto"
signature: "wdaawdaw"
start_direction: "NORTH_WEST"
input: "number"
output: "many"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Numeric Jumps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Prometheus' Gambit

◇ **Pattern** · [[Caduceus]] · 📖 [[Numeric Jumps]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="143.0,127.4 123.6,94.0 104.3,60.5 143.0,60.5 123.6,27.0 104.3,60.5 85.0,94.0 65.7,60.5 46.4,94.0 27.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="143.0,127.4 123.6,94.0 104.3,60.5 143.0,60.5 123.6,27.0 104.3,60.5 85.0,94.0 65.7,60.5 46.4,94.0 27.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="127.4" x2="136.8" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,109.0 140.5,114.6 133.1,118.8" fill="#fc77be"/><circle cx="143.0" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdaawdaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number` ([[Number]])
- **Leaves:** `many` (—)
- **ID:** `caduceus:eval/goto`

## Description

Remove a number from the [[The Stack|stack]], then jump to that index in the list cast by the enclosing [[Elbrus' Gambit]]. If the number is negative, the index counts from the end of the list instead.

## Related patterns (same book page)

- [[Elbrus' Gambit]] `[pattern] → many`

## Sources

- 📖 **Guidebook:** [[Numeric Jumps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Prometheus' Gambit](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/goto@caduceus:eval/goto)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGoto` is not a `SpellAction`
- 🔧 **Pattern ID:** `caduceus:eval/goto`
