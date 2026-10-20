---
aliases: ["Retrospection"]
tags: [hexcasting/pattern, mod/hex-casting, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:close_paren"
signature: "eee"
start_direction: "EAST"
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

# Retrospection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Escaping Patterns]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,38.0 92.3,38.0 119.4,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="38.0,38.0 92.3,38.0 119.4,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="38.0" x2="55.4" y2="38.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.8,38.0 55.4,44.0 55.4,32.0" fill="#fc77be"/><circle cx="38.0" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:close_paren`

## Description

If I draw another [[Retrospection|Introspection]], it'll still be saved to the list, but I'll then have to draw two [[Retrospection|Retrospections]] to get back to normal casting.

## Example

Closes an [[Introspection]] block. Everything drawn in between becomes one [[List]] of patterns.

## Related patterns (same book page)

- [[Contemplation]] `— → —`
- [[Introspection]] `— → —`
- [[Evanition]] `— → —`
- [[Consideration]] `— → —`

## Sources

- 📖 **Guidebook:** [[Escaping Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Retrospection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/patterns_as_iotas@hexcasting:close_paren)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCloseParen` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:close_paren`
