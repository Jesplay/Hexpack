---
aliases: ["Librarian's Purification II"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/storage]
kind: "Pattern"
family: "Purification"
topics: ["Reading & Writing"]
mod: "Hexical"
op_id: "hexical:read_shelf"
signature: "qaqqqada"
start_direction: "EAST"
input: "vector"
output: "any"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Akashic Utilities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Librarian's Purification II

◇ **Pattern** · [[Hexical]] · 📖 [[Akashic Utilities]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="35.0,78.3 85.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqada` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexical:read_shelf`

```text
before   … │ vector  ◂ top
after    … │ any     ◂ top
```

## Description

Reads the [[Iotas|iota]] from an [[Akashic Libraries|Akashic Bookshelf]].

## Related patterns (same book page)

- [[Librarian's Purification]] `vector → pattern/null`
- [[Librarian's Gambit]] `vector, pattern, any → —`
- [[Librarian's Gambit II]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Akashic Utilities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Librarian's Purification II](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/akashic_conveniences@hexical:read_shelf)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadAkashicShelf` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:read_shelf`
