---
aliases: ["Reveal"]
tags: [hexcasting/pattern, mod/hex-casting, family/other, topic/debugging, topic/text]
kind: "Pattern"
family: "Other"
topics: ["Debugging", "Text, Strings & Chat"]
mod: "Hex Casting"
op_id: "hexcasting:print"
signature: "de"
start_direction: "NORTH_EAST"
input: "any"
output: "any"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Reveal

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]] · [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,85.0 65.1,38.0 92.3,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:163;--dur:1.6s" points="38.0,85.0 65.1,38.0 92.3,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="85.0" x2="46.7" y2="70.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,59.1 51.8,72.9 41.5,67.0" fill="#fc77be"/><circle cx="38.0" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `de` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexcasting:print`

```text
before   … │ any  ◂ top
after    … │ any  ◂ top
```

## Description

Displays the top [[Iotas|iota]] of the [[The Stack|stack]] to me.

## Example

Shows the top iota in chat **without removing it**. Your best debugging tool: drop it in anywhere to see what's on top of the stack.

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Alidade's Purification]] `entity → vector`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Stadiometer's Purification]] `entity → num`
- [[Pace Purification]] `entity → vector`
- [[Media Reflection (Hex Casting)]] `— → num`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Reveal](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:print)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPrint` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:print`
