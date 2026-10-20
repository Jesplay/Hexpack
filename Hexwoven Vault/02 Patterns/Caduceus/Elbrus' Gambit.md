---
aliases: ["Elbrus' Gambit"]
tags: [hexcasting/pattern, mod/caduceus, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:eval/setup_goto"
signature: "qqdqdqdqq"
start_direction: "EAST"
input: "[pattern]"
output: "many"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Numeric Jumps]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Elbrus' Gambit

◇ **Pattern** · [[Caduceus]] · 📖 [[Numeric Jumps]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="106" viewBox="0 0 170.0 106.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/><circle cx="18.6" cy="87.6" r="2"/><circle cx="45.2" cy="87.6" r="2"/><circle cx="71.7" cy="87.6" r="2"/><circle cx="98.3" cy="87.6" r="2"/><circle cx="124.8" cy="87.6" r="2"/><circle cx="151.4" cy="87.6" r="2"/></g><polyline class="hexpat-base" points="18.6,87.6 45.2,87.6 58.4,64.6 45.2,41.6 71.7,41.6 85.0,18.6 98.3,41.6 124.8,41.6 111.6,64.6 124.8,87.6 151.4,87.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:266;--dur:2.8s" points="18.6,87.6 45.2,87.6 58.4,64.6 45.2,41.6 71.7,41.6 85.0,18.6 98.3,41.6 124.8,41.6 111.6,64.6 124.8,87.6 151.4,87.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.6" y1="87.6" x2="27.1" y2="87.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="33.2,87.6 27.1,90.5 27.1,84.7" fill="#fc77be"/><circle cx="18.6" cy="87.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqdqdqdqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern]` ([[List]], [[Pattern]])
- **Leaves:** `many` (—)
- **ID:** `caduceus:eval/setup_goto`

## Description

Cast a list of patterns from the [[The Stack|stack]] exactly like [[Hermes' Gambit]], except that a [[Prometheus' Gambit]] within this cast will jump to an index within this list of patterns.

## Related patterns (same book page)

- [[Prometheus' Gambit]] `number → many`

## Sources

- 📖 **Guidebook:** [[Numeric Jumps]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Elbrus' Gambit](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/goto@caduceus:eval/setup_goto)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetupGoto` is not a `SpellAction`
- 🔧 **Pattern ID:** `caduceus:eval/setup_goto`
