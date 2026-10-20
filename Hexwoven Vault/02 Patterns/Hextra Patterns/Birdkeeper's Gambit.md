---
aliases: ["Birdkeeper's Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/lists]
kind: "Pattern"
family: "Gambit"
topics: ["Lists"]
mod: "Hextra Patterns"
op_id: "hextrapats:mask_list"
signature: "ewdqdwedww"
start_direction: "SOUTH_WEST"
input: "many"
output: "many, list"
cost: ""
iotas_in: []
iotas_out: ["[[List]]"]
entry: "[[Hextra Lists]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Birdkeeper's Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Lists]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 65.7,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 65.7,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="60.5" x2="136.8" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,78.9 133.1,69.1 140.5,73.3" fill="#fc77be"/><circle cx="143.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewdqdwedww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many` (—)
- **Leaves:** `many, list` ([[List]])
- **ID:** `hextrapats:mask_list`

## Description

An infinite family of patterns that creates a list out of [[Iotas|iotas]] on the [[The Stack|stack]] based on a sequence of dips and lines.

Each dip represents an iota to be drawn into the list. Leftover iotas remain where they are.

The first pattern will produce an empty list when drawn.

The second pattern will make a list from the 3rd and 2nd iota on the stack.

The third pattern will take in the 7th, 5th, 4th, and 1st iota on the stack.

## Related patterns (same book page)

- [[Speaker's Decomposition II]] `list → list, any`
- [[Derivation Decomposition II]] `list → list, any`
- [[Viewer's Gambit]] `list, num → list, any`
- [[Separation Gambit]] `list, num → list, list`
- [[Elimination Gambit]] `list, any → list, bool`
- [[Elimination Gambit II]] `list, any → list, num`
- [[Swindler's Distillation]] `list, num → list`
- [[Shuffling Purification]] `list → list`

## Sources

- 📖 **Guidebook:** [[Hextra Lists]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:mask_list`
