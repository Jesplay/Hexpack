---
aliases: ["Disjunction Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:or"
signature: "waw"
start_direction: "SOUTH_EAST"
input: "bool, bool"
output: "bool"
cost: ""
iotas_in: ["[[Boolean]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Logical Operators]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Disjunction Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Logical Operators]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="35.0" x2="43.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,58.8 38.2,51.6 47.8,46.1" fill="#fc77be"/><circle cx="35.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `bool, bool` ([[Boolean]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:or`

```text
before   … │ bool │ bool  ◂ top
after    … │ bool         ◂ top
```

## Description

Returns True if at least one of the arguments are True; otherwise returns False.

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Longs]]

`long, long` → `long`

Perform Bitwise OR

### [[Hex Casting]] · 📖 [[Sets]]

`(num, num)|(list, list)` → `num|list`

Unifies two sets.

## Related patterns (same book page)

- [[Conjunction Distillation]] `bool, bool → bool`
- [[Negation Purification]] `bool → bool`
- [[Exclusion Distillation]] `bool, bool → bool`
- [[Maximus Distillation]] `number, number → bool`
- [[Minimus Distillation]] `number, number → bool`
- [[Maximus Distillation II]] `number, number → bool`
- [[Minimus Distillation II]] `number, number → bool`
- [[Augur's Purification]] `any → bool`
- [[Augur's Exaltation]] `bool, any, any → any`
- [[Equality Distillation]] `any, any → bool`
- [[Inequality Distillation]] `any, any → bool`

## Sources

- 📖 **Guidebook:** [[Logical Operators]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Disjunction Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/logic@hexcasting:or)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:or`
