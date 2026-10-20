---
aliases: ["Introjection"]
tags: [hexcasting/pattern, mod/hexthings, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "HexThings"
op_id: "hexthings:unquote"
signature: "aqqq"
start_direction: "NORTH_EAST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Utilities]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Introjection

◇ **Pattern** · [[HexThings]] · 📖 [[Utilities]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,85.0 119.4,38.0 65.1,38.0 38.0,85.0 65.1,132.0 119.4,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="92.3,85.0 119.4,38.0 65.1,38.0 38.0,85.0 65.1,132.0 119.4,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="85.0" x2="101.0" y2="70.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.2,59.1 106.1,72.9 95.8,67.0" fill="#fc77be"/><circle cx="92.3" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexthings:unquote`

## Description

This pattern can be used to place raw [[Iotas|iota]] directly into an Introspection-Restrospection pair. After drawing this pattern, all subsequent patterns are "unescaped", allowing them to be executed as normal until a [[Retrojection]] is encountered.

## Related patterns (same book page)

- [[Patience]] `— → —`
- [[Patience Patterns]] `— → —`
- [[Shard Reflection]] `— → bool | null`
- [[Retrojection]] `— → —`

## Sources

- 📖 **Guidebook:** [[Utilities]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexthings:unquote`
