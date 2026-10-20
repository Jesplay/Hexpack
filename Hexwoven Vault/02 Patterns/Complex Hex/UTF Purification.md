---
aliases: ["UTF Purification"]
tags: [hexcasting/pattern, mod/complex-hex, family/purification, topic/text]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat"]
mod: "Complex Hex"
op_id: "complexhex:ascii"
signature: "eawdwae"
start_direction: "EAST"
input: "num/string"
output: "string/num"
cost: ""
iotas_in: ["[[Number]]", "[[String]]"]
iotas_out: ["[[Number]]", "[[String]]"]
entry: "[[Miscellaneous Complexes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# UTF Purification

◇ **Pattern** · [[Complex Hex]] · 📖 [[Miscellaneous Complexes]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="83" viewBox="0 0 170.0 83.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/></g><polyline class="hexpat-base" points="18.6,41.6 45.2,41.6 58.4,64.6 71.7,41.6 85.0,18.6 98.3,41.6 111.6,64.6 124.8,41.6 151.4,41.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="18.6,41.6 45.2,41.6 58.4,64.6 71.7,41.6 85.0,18.6 98.3,41.6 111.6,64.6 124.8,41.6 151.4,41.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.6" y1="41.6" x2="27.1" y2="41.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="33.2,41.6 27.1,44.5 27.1,38.7" fill="#fc77be"/><circle cx="18.6" cy="41.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eawdwae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num/string` ([[Number]], [[String]])
- **Leaves:** `string/num` ([[Number]], [[String]])
- **ID:** `complexhex:ascii`

```text
before   … │ num/string  ◂ top
after    … │ string/num  ◂ top
```

## Description

Swaps between a string of a single character and an integer representing that character.&lt;br&gt;Uses the **UTF-16** character map.

## Sources

- 📖 **Guidebook:** [[Miscellaneous Complexes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [UTF Purification](https://complexhex.hexxy.media/v/latest/0.1.x/en_us/#patterns/misc@complexhex:ascii)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpASCIIValue` is not a `SpellAction`
- 🔧 **Pattern ID:** `complexhex:ascii`
