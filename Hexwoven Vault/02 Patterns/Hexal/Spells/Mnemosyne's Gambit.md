---
aliases: ["Mnemosyne's Gambit"]
tags: [hexcasting/spell, mod/hexal, family/gambit, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hexal"
op_id: "hexal:everbook/read"
signature: "eweeewedqdeddw"
start_direction: "NORTH_EAST"
input: "vector, pattern"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Everbook]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mnemosyne's Gambit

✦ **Spell** · [[Hexal]] · 📖 [[Everbook]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 65.7,60.5 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:580;--dur:4.2s" points="27.0,60.5 46.4,27.0 85.0,27.0 123.6,27.0 143.0,60.5 123.6,94.0 85.0,94.0 46.4,94.0 27.0,60.5 65.7,60.5 85.0,27.0 104.3,60.5 85.0,94.0 65.7,60.5 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="33.2" y2="49.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,42.1 36.9,51.9 29.5,47.7" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eweeewedqdeddw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, pattern` ([[Vector]], [[Pattern]])
- **Leaves:** `nothing` (—)
- **ID:** `hexal:everbook/read`

```text
before   … │ vector │ pattern  ◂ top
after    …                     ◂ top
```

## Description

Takes the position of an [[Akashic Libraries|Akashic Record]], as well as a Pattern as a key, and retrieves the [[Iotas|iota]] at that key from your Everbook, storing it in the given [[Akashic Libraries|Akashic Record]] if able.

## Related patterns (same book page)

- [[Elysium's Gambit]] `vector, pattern → —`
- [[Lethe's Gambit]] `pattern → —`
- [[Brigh's Gambit]] `pattern → —`

## Sources

- 📖 **Guidebook:** [[Everbook]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Mnemosyne's Gambit](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/everbook@hexal:everbook/read)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpEverbookRead` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexal:everbook/read`
