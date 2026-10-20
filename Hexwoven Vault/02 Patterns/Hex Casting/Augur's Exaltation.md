---
aliases: ["Augur's Exaltation"]
tags: [hexcasting/pattern, mod/hex-casting, family/exaltation, topic/logic]
kind: "Pattern"
family: "Exaltation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:if"
signature: "awdd"
start_direction: "SOUTH_EAST"
input: "bool, any, any"
output: "any"
cost: ""
iotas_in: ["[[Boolean]]", "[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Logical Operators]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Augur's Exaltation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Logical Operators]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:1.6s" points="35.0,78.3 60.0,121.6 85.0,78.3 110.0,35.0 135.0,78.3 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="43.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,102.1 38.2,94.9 47.8,89.4" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awdd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `bool, any, any` ([[Boolean]], [[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexcasting:if`

```text
before   … │ bool │ any │ any  ◂ top
after    … │ any               ◂ top
```

## Description

If the first argument is True, keeps the second and discards the third; otherwise discards the second and keeps the third.

## Example

`bool, a, b → a` if the boolean is true, otherwise `b`. To *branch*, pick between two lists of patterns and run the result with [[Hermes' Gambit]].

## Related patterns (same book page)

- [[Conjunction Distillation]] `bool, bool → bool`
- [[Disjunction Distillation]] `bool, bool → bool`
- [[Negation Purification]] `bool → bool`
- [[Exclusion Distillation]] `bool, bool → bool`
- [[Maximus Distillation]] `number, number → bool`
- [[Minimus Distillation]] `number, number → bool`
- [[Maximus Distillation II]] `number, number → bool`
- [[Minimus Distillation II]] `number, number → bool`
- [[Augur's Purification]] `any → bool`
- [[Equality Distillation]] `any, any → bool`
- [[Inequality Distillation]] `any, any → bool`

## Sources

- 📖 **Guidebook:** [[Logical Operators]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Augur's Exaltation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/logic@hexcasting:if)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpBoolIf` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:if`
