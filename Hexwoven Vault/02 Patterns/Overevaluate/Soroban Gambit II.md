---
aliases: ["Soroban Gambit II"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/storage]
kind: "Pattern"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Overevaluate"
op_id: "overevaluate:soroban_set"
signature: "waqdee"
start_direction: "SOUTH_EAST"
input: "number"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Soroban]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Soroban Gambit II

◇ **Pattern** · [[Overevaluate]] · 📖 [[Soroban]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,30.5 52.3,68.3 74.1,106.0 95.9,68.3 74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="30.5,30.5 52.3,68.3 74.1,106.0 95.9,68.3 74.1,30.5 117.7,30.5 139.5,68.3 117.7,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="30.5" x2="37.5" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,51.3 33.3,45.0 41.6,40.2" fill="#fc77be"/><circle cx="30.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqdee` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `overevaluate:soroban_set`

```text
before   … │ number  ◂ top
after    …           ◂ top
```

## Description

Pops a number from the [[The Stack|stack]] and sets the soroban to it.

## Related patterns (same book page)

- [[Soroban Reflection]] `— → number`
- [[Soroban Gambit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Soroban]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Soroban Gambit II](https://overevaluate.hexxy.media/v/latest/main/en_us/#soroban@overevaluate:soroban_set)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSorobanSet` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:soroban_set`
