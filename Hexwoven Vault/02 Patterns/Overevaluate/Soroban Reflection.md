---
aliases: ["Soroban Reflection"]
tags: [hexcasting/pattern, mod/overevaluate, family/reflection, topic/storage]
kind: "Pattern"
family: "Reflection"
topics: ["Reading & Writing"]
mod: "Overevaluate"
op_id: "overevaluate:soroban_increment"
signature: "wdeaqq"
start_direction: "NORTH_EAST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Soroban]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Soroban Reflection

◇ **Pattern** · [[Overevaluate]] · 📖 [[Soroban]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,106.0 52.3,68.3 74.1,30.5 95.9,68.3 74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="30.5,106.0 52.3,68.3 74.1,30.5 95.9,68.3 74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="106.0" x2="37.5" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,85.3 41.6,96.3 33.3,91.5" fill="#fc77be"/><circle cx="30.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdeaqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `overevaluate:soroban_increment`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes soroban's current value and then increases it. Starts at 0.

## Related patterns (same book page)

- [[Soroban Gambit]] `— → —`
- [[Soroban Gambit II]] `number → —`

## Sources

- 📖 **Guidebook:** [[Soroban]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Soroban Reflection](https://overevaluate.hexxy.media/v/latest/main/en_us/#soroban@overevaluate:soroban_increment)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSorobanIncrement` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:soroban_increment`
