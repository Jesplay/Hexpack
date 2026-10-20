---
aliases: ["Soroban Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/storage]
kind: "Pattern"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Overevaluate"
op_id: "overevaluate:soroban_reset"
signature: "qdeeaae"
start_direction: "NORTH_EAST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Soroban]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Soroban Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Soroban]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,121.6 60.0,78.3 35.0,35.0 85.0,35.0 110.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 135.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="35.0,121.6 60.0,78.3 35.0,35.0 85.0,35.0 110.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 135.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="121.6" x2="43.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,97.8 47.8,110.5 38.2,105.0" fill="#fc77be"/><circle cx="35.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qdeeaae` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `overevaluate:soroban_reset`

## Description

Resets the soroban to 0.

## Related patterns (same book page)

- [[Soroban Reflection]] `— → number`
- [[Soroban Gambit II]] `number → —`

## Sources

- 📖 **Guidebook:** [[Soroban]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Soroban Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#soroban@overevaluate:soroban_reset)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSorobanReset` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:soroban_reset`
