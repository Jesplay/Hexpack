---
aliases: ["Elimination Gambit II"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/lists]
kind: "Pattern"
family: "Gambit"
topics: ["Lists"]
mod: "Hextra Patterns"
op_id: "hextrapats:del_element/all"
signature: "dedwqaeaqaw"
start_direction: "NORTH_EAST"
input: "list, any"
output: "list, num"
cost: ""
iotas_in: ["[[List]]", "[[Any Iota]]"]
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Hextra Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Elimination Gambit II

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 46.4,94.0 85.0,94.0 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5 85.0,27.0 46.4,27.0 65.7,60.5 46.4,94.0 85.0,94.0 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="104.3" y1="60.5" x2="110.5" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="114.9,42.1 114.2,51.9 106.8,47.7" fill="#fc77be"/><circle cx="104.3" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedwqaeaqaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, any` ([[List]], [[Any Iota]])
- **Leaves:** `list, num` ([[Number]], [[List]])
- **ID:** `hextrapats:del_element/all`

```text
before   … │ list │ any  ◂ top
after    … │ list │ num  ◂ top
```

## Description

Remove the [[Iotas|iota]] at the top of the [[The Stack|stack]], then remove all instances of it from the list. Returns the number of removed occurrences.

## Related patterns (same book page)

- [[Speaker's Decomposition II]] `list → list, any`
- [[Derivation Decomposition II]] `list → list, any`
- [[Viewer's Gambit]] `list, num → list, any`
- [[Separation Gambit]] `list, num → list, list`
- [[Elimination Gambit]] `list, any → list, bool`
- [[Swindler's Distillation]] `list, num → list`
- [[Shuffling Purification]] `list → list`
- [[Birdkeeper's Gambit]] `many → many, list`

## Sources

- 📖 **Guidebook:** [[Hextra Lists]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDeleteAll` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:del_element/all`
