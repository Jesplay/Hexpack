---
aliases: ["Brigh's Gambit"]
tags: [hexcasting/pattern, mod/hexal, family/gambit, topic/storage]
kind: "Pattern"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hexal"
op_id: "hexal:everbook/toggle_macro"
signature: "eweeewedww"
start_direction: "SOUTH_WEST"
input: "pattern"
output: ""
cost: ""
iotas_in: ["[[Pattern]]"]
iotas_out: []
entry: "[[Everbook]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Brigh's Gambit

◇ **Pattern** · [[Hexal]] · 📖 [[Everbook]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 104.3,60.5 65.7,60.5 27.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="143.0" y1="60.5" x2="136.8" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="132.3,78.9 133.1,69.1 140.5,73.3" fill="#fc77be"/><circle cx="143.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eweeewedww` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern` ([[Pattern]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:everbook/toggle_macro`

```text
before   … │ pattern  ◂ top
after    …            ◂ top
```

## Description

Takes a pattern and marks that entry of your Everbook as a Pattern that when drawn by your hand will be treated as any other of Nature's patterns.

When you draw the key of an Everbook entry that has been marked as a Pattern, if that entry is a list Nature will execute each element of the list as it would with [[Hermes' Gambit]], though it does not use up an evaluation. Attempting to mark an entry as a Pattern if it is non-existent or infinitely recursive will fail.

## Related patterns (same book page)

- [[Mnemosyne's Gambit]] `vector, pattern → —`
- [[Elysium's Gambit]] `vector, pattern → —`
- [[Lethe's Gambit]] `pattern → —`

## Sources

- 📖 **Guidebook:** [[Everbook]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Brigh's Gambit](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/everbook@hexal:everbook/toggle_macro)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpToggleMacro` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:everbook/toggle_macro`
