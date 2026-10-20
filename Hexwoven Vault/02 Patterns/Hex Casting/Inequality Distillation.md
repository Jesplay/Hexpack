---
aliases: ["Inequality Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:not_equals"
signature: "da"
start_direction: "EAST"
input: "any, any"
output: "bool"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Logical Operators]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Inequality Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Logical Operators]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="133" viewBox="0 0 170.0 132.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="41.0" cy="41.0" r="2"/><circle cx="99.7" cy="41.0" r="2"/><circle cx="158.3" cy="41.0" r="2"/><circle cx="11.7" cy="91.8" r="2"/><circle cx="70.3" cy="91.8" r="2"/><circle cx="129.0" cy="91.8" r="2"/></g><polyline class="hexpat-base" points="41.0,41.0 99.7,41.0 70.3,91.8 129.0,91.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:176;--dur:1.6s" points="41.0,41.0 99.7,41.0 70.3,91.8 129.0,91.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.0" y1="41.0" x2="59.8" y2="41.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.3,41.0 59.8,47.5 59.8,34.6" fill="#fc77be"/><circle cx="41.0" cy="41.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `da` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, any` ([[Any Iota]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:not_equals`

```text
before   … │ any │ any  ◂ top
after    … │ bool       ◂ top
```

## Description

If the first argument does not equal the second (outside a small tolerance), return True. Otherwise, return False.

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
- [[Augur's Exaltation]] `bool, any, any → any`
- [[Equality Distillation]] `any, any → bool`

## Sources

- 📖 **Guidebook:** [[Logical Operators]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Inequality Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/logic@hexcasting:not_equals)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEquality` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:not_equals`
