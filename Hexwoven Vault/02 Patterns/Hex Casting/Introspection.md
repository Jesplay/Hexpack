---
aliases: ["Introspection"]
tags: [hexcasting/pattern, mod/hex-casting, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:open_paren"
signature: "qqq"
start_direction: "WEST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Escaping Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Introspection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Escaping Patterns]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="119.4,38.0 65.1,38.0 38.0,85.0 65.1,132.0 119.4,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="119.4,38.0 65.1,38.0 38.0,85.0 65.1,132.0 119.4,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="119.4" y1="38.0" x2="102.0" y2="38.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="89.6,38.0 102.0,32.0 102.0,44.0" fill="#fc77be"/><circle cx="119.4" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:open_paren`

## Description

Drawing [[Introspection]] makes my drawing of patterns act differently, for a time. Until I draw [[Retrospection]], the patterns I draw are saved. Then, when I draw [[Retrospection]], they are added to the [[The Stack|stack]] as a list [[Iotas|iota]].

## Example

**Introspection** → [[Mind's Reflection]] → [[Reveal]] → [[Retrospection]] leaves a **list of two patterns** on the stack, a little program you can run with [[Hermes' Gambit]] or save with [[Scribe's Gambit]].

## Related patterns (same book page)

- [[Contemplation]] `— → —`
- [[Retrospection]] `— → —`
- [[Evanition]] `— → —`
- [[Consideration]] `— → —`

## Sources

- 📖 **Guidebook:** [[Escaping Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Introspection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/patterns_as_iotas@hexcasting:open_paren)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpOpenParen` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:open_paren`
