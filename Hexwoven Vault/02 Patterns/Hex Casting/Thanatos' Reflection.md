---
aliases: ["Thanatos' Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/control-flow]
kind: "Pattern"
family: "Reflection"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:thanatos"
signature: "qqaed"
start_direction: "SOUTH_EAST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Thanatos' Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 85.0,78.3 60.0,35.0 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 85.0,78.3 60.0,35.0 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="43.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,102.1 38.2,94.9 47.8,89.4" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqaed` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexcasting:thanatos`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

It seems there is a limit to how many patterns can be evaluated within a single **Hex**. I can use this pattern to deterime how many patterns remain before I reach this limit.

## Example

How many more patterns this hex may run before hitting the evaluation limit. Use it as a safety stop in recursive hexes: check it with [[Minimus Distillation]] and bail out with [[Charon's Gambit]].

## Related patterns (same book page)

- [[Hermes' Gambit]] `evaluatable → many`
- [[Iris' Gambit]] `evaluatable → many`
- [[Thoth's Gambit]] `evaluatable, list → list`
- [[Charon's Gambit]] `— → —`

## Sources

- 📖 **Guidebook:** [[Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Thanatos' Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/meta@hexcasting:thanatos)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpThanos` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:thanatos`
