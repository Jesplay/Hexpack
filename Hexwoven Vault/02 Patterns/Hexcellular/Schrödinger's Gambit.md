---
aliases: ["Schrödinger's Gambit"]
tags: [hexcasting/pattern, mod/hexcellular, family/gambit, topic/data-structures]
kind: "Pattern"
family: "Gambit"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hexcellular"
op_id: "hexcellular:set_property"
signature: "aawq"
start_direction: "SOUTH_WEST"
input: "property, any"
output: ""
cost: "Costs about a tenth of an Amethyst Dust."
iotas_in: ["[[Property]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Property Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Schrödinger's Gambit

◇ **Pattern** · [[Hexcellular]] · 📖 [[Property Iota]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 85.0,35.0 35.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:1.6s" points="110.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 85.0,35.0 35.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="78.3" x2="102.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="96.3,102.1 97.2,89.4 106.8,94.9" fill="#fc77be"/><circle cx="110.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `property, any` ([[Property]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a tenth of an Amethyst Dust.
- **ID:** `hexcellular:set_property`

```text
before   … │ property │ any  ◂ top
after    …                   ◂ top
```

## Description

Sets the property [[Iotas|iota]]. Costs about a tenth of an [[Amethyst|Amethyst Dust]].

Occasionally, I may want to pass out property iota without the risk of someone else overwriting their contents. In such cases, I can create read-only copies. They can still be read using [[Observation Purification]] and will refer to the **Property** as usual but they can not be written to with [[Schrödinger's Gambit]].

To distinguish, read-only property iotas are bolden.

## Related patterns (same book page)

- [[Schrödinger's Reflection]] `— → property`
- [[Observation Purification]] `property → any`
- [[Schrödinger's Purification]] `property → property`

## Sources

- 📖 **Guidebook:** [[Property Iota]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Schrödinger's Gambit](https://hexcellular.hexxy.media/v/1.1.1/1.0.0/en_us/#properties@hexcellular:set_property)
- 🧩 **Mod page:** [Hexcellular on Modrinth](https://modrinth.com/mod/mN6t9Bt4)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetProperty` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcellular:set_property`
