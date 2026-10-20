---
aliases: ["Negation Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/logic]
kind: "Pattern"
family: "Purification"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:not"
signature: "dw"
start_direction: "NORTH_WEST"
input: "bool"
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

# Negation Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Logical Operators]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="60.0,78.3 35.0,35.0 85.0,35.0 135.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:150;--dur:1.6s" points="60.0,78.3 35.0,35.0 85.0,35.0 135.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="78.3" x2="52.0" y2="64.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.2,54.5 56.8,61.7 47.2,67.2" fill="#fc77be"/><circle cx="60.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `bool` ([[Boolean]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:not`

```text
before   … │ bool  ◂ top
after    … │ bool  ◂ top
```

## Description

If the argument is True, return False; if it is False, return True.

## Other uses (same pattern, other inputs)

### [[Complex Hex]] · 📖 [[Longs]]

`long, long` → `long`

Perform Bitwise NOT

### [[Hex Casting]] · 📖 [[Sets]]

`num` → `num`

Takes the inversion of a bitset, changing all "on" bits to "off" and vice versa. In my experience, this will take the form of that number negated and decreased by one. For example, 0 will become -1, and -100 will become 99.

## Related patterns (same book page)

- [[Conjunction Distillation]] `bool, bool → bool`
- [[Disjunction Distillation]] `bool, bool → bool`
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
- 🌐 **Official online docs:** [Negation Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/logic@hexcasting:not)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:not`
