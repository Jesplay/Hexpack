---
aliases: ["Minimus Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:less"
signature: "q"
start_direction: "SOUTH_WEST"
input: "number, number"
output: "bool"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Logical Operators]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Minimus Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Logical Operators]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="103" height="170" viewBox="0 0 103.1 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="65.1,38.0 38.0,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:109;--dur:1.6s" points="65.1,38.0 38.0,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.1" y1="38.0" x2="56.4" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.2,63.8 51.3,50.1 61.6,56.0" fill="#fc77be"/><circle cx="65.1" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `q` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number, number` ([[Number]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:less`

```text
before   … │ number │ number  ◂ top
after    … │ bool             ◂ top
```

## Description

If the first argument is less than the second, return True. Otherwise, return False.

## Other uses (same pattern, other inputs)

### [[Hextra Patterns]] · 📖 [[Hextra Logic]]

`vec, vec` → `bool`

If the first vector is shorter than the second, return True. Otherwise, return False.

## Related patterns (same book page)

- [[Conjunction Distillation]] `bool, bool → bool`
- [[Disjunction Distillation]] `bool, bool → bool`
- [[Negation Purification]] `bool → bool`
- [[Exclusion Distillation]] `bool, bool → bool`
- [[Maximus Distillation]] `number, number → bool`
- [[Maximus Distillation II]] `number, number → bool`
- [[Minimus Distillation II]] `number, number → bool`
- [[Augur's Purification]] `any → bool`
- [[Augur's Exaltation]] `bool, any, any → any`
- [[Equality Distillation]] `any, any → bool`
- [[Inequality Distillation]] `any, any → bool`

## Sources

- 📖 **Guidebook:** [[Logical Operators]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Minimus Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/logic@hexcasting:less)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:less`
