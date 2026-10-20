---
aliases: ["Librarian's Gambit"]
tags: [hexcasting/spell, mod/hexical, family/gambit, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hexical"
op_id: "hexical:write_shelf"
signature: "edeeedad"
start_direction: "SOUTH_WEST"
input: "vector, pattern, any"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Pattern]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Akashic Utilities]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Librarian's Gambit

✦ **Spell** · [[Hexical]] · 📖 [[Akashic Utilities]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3 60.0,121.6 35.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="110.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 110.0,121.6 85.0,78.3 60.0,121.6 35.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="35.0" x2="102.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.2,58.8 97.2,46.1 106.8,51.6" fill="#fc77be"/><circle cx="110.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeeedad` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, pattern, any` ([[Vector]], [[Pattern]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:write_shelf`

```text
before   … │ vector │ pattern │ any  ◂ top
after    …                           ◂ top
```

## Description

Writes an [[Iotas|iota]] under a pattern key to an [[Akashic Libraries|Akashic Bookshelf]].

## Related patterns (same book page)

- [[Librarian's Purification]] `vector → pattern/null`
- [[Librarian's Purification II]] `vector → any`
- [[Librarian's Gambit II]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Akashic Utilities]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Librarian's Gambit](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/akashic_conveniences@hexical:write_shelf)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpWriteAkashicShelf` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:write_shelf`
