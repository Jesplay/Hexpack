---
aliases: ["Contemplation"]
tags: [hexcasting/pattern, mod/hex-casting, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:runtime_escape"
signature: "wdeee"
start_direction: "SOUTH_EAST"
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

# Contemplation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Escaping Patterns]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,38.0 92.3,85.0 119.4,132.0 65.1,132.0 38.0,85.0 65.1,38.0 119.4,38.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="65.1,38.0 92.3,85.0 119.4,132.0 65.1,132.0 38.0,85.0 65.1,38.0 119.4,38.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="38.0" x2="73.8" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="80.1,63.8 68.6,56.0 79.0,50.1" fill="#fc77be"/><circle cx="65.1" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdeee` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:runtime_escape`

## Description

To use [[Contemplation]], I draw it, then another arbitrary pattern. That second pattern is added to the [[The Stack|stack]].

One may find it helpful to think of this as "escaping" the pattern onto the stack, if they happen to be familiar with the science of computers.

The usual use for this is to copy the pattern to a [[Scrolls|Scroll]] or [[Slates|Slate]] using [[Scribe's Gambit]], and then perhaps decorating with them.

## Related patterns (same book page)

- [[Introspection]] `— → —`
- [[Retrospection]] `— → —`
- [[Evanition]] `— → —`
- [[Consideration]] `— → —`

## Sources

- 📖 **Guidebook:** [[Escaping Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Contemplation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/patterns_as_iotas@hexcasting:runtime_escape)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRuntimeEscape` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:runtime_escape`
