---
aliases: ["Mindsplice Purification"]
tags: [hexcasting/pattern, mod/hexdebug, family/purification, topic/debugging, hexcasting/needs-enlightenment]
kind: "Pattern"
family: "Purification"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:splicing/enlightened/hex/read"
signature: "wqaeaqwqqqwqwqqwq"
start_direction: "NORTH_WEST"
input: "vec"
output: "list | null"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[List]]", "[[Null]]"]
entry: "[[Mindsplice Table Patterns]]"
needs_enlightenment: true
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mindsplice Purification

◇ **Pattern** · [[HexDebug]] · 📖 [[Mindsplice Table Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** 🔒 needs **enlightenment** ([[Progression - Enlightenment & Great Spells|how]])

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="111.8,115.3 94.4,85.0 76.9,54.7 41.9,54.7 59.4,85.0 41.9,115.3 76.9,115.3 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 94.4,145.5 111.8,115.3 129.3,85.0 111.8,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="111.8,115.3 94.4,85.0 76.9,54.7 41.9,54.7 59.4,85.0 41.9,115.3 76.9,115.3 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 94.4,145.5 111.8,115.3 129.3,85.0 111.8,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.8" y1="115.3" x2="106.2" y2="105.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.2,98.6 109.6,103.7 102.9,107.5" fill="#fc77be"/><circle cx="111.8" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqaeaqwqqqwqwqqwq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `list | null` ([[List]], [[Null]])
- **ID:** `hexdebug:splicing/enlightened/hex/read`

## Description

Remove a vector from the [[The Stack|stack]], then push the **Hex** currently imbued to the [[Mindsplice Table]] at that position, or **Null** if it has not yet been imbued.

In addition to these patterns, I can also cast [[Waystone Reflection]] and [[Lodestone Reflection]] from within a [[Mindsplice Table]]. However, trying to cast them through a [[Staves|Staff]], or trying to cast other [[Spell Circles|Spell Circle]]-only patterns through a [[Mindsplice Table]], will still fail rather spectacularly.

## Related patterns (same book page)

- [[Imbue Mindsplice Table]] `vec, list → —`

## Sources

- 📖 **Guidebook:** [[Mindsplice Table Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Mindsplice Purification](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/enlightened_splicing_table@hexdebug:splicing/enlightened/hex/read)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadEnlightenedHex` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexdebug:splicing/enlightened/hex/read`
