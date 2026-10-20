---
aliases: ["Interjection"]
tags: [hexcasting/pattern, mod/hex-casting, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:read_into_parens"
signature: "aqqqqqwded"
start_direction: "EAST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Advanced Escaping]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Interjection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Advanced Escaping]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,60.5 104.3,60.5 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="78.0" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.9,60.5 78.0,64.8 78.0,56.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqqqqwded` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:read_into_parens`

## Description

Only works while [[Introspection|introspecting]]. Reads an [[Iotas|iota]] from my offhand, and adds it to the in-progress list. For iotas other than patterns, I may need to [[Escaping Patterns|take steps]] to avoid a [[Mishaps|mishap]] when executing the list.

## Related patterns (same book page)

- [[Meditation]] `num → —`
- [[Recollection]] `— → num, list`

## Sources

- 📖 **Guidebook:** [[Advanced Escaping]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Interjection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_escaping@hexcasting:read_into_parens)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadIntoParens` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:read_into_parens`
