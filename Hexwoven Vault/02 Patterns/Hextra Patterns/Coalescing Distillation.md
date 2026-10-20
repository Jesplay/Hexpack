---
aliases: ["Coalescing Distillation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/distillation, topic/logic]
kind: "Pattern"
family: "Distillation"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nulls/coalesce"
signature: "wawaa"
start_direction: "SOUTH_EAST"
input: "any, any"
output: "any"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Null Coercion]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Coalescing Distillation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Null Coercion]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="35.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 135.0,35.0 85.0,35.0 110.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="35.0" x2="43.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,58.8 38.2,51.6 47.8,46.1" fill="#fc77be"/><circle cx="35.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, any` ([[Any Iota]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hextrapats:nulls/coalesce`

```text
before   … │ any │ any  ◂ top
after    … │ any        ◂ top
```

## Description

If the first argument is not Null, returns it. Otherwise, returns the second argument.

## Related patterns (same book page)

- [[Coalescing Gambit]] `any, [pattern] | pattern → any`

## Sources

- 📖 **Guidebook:** [[Null Coercion]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNullCoalesce` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:nulls/coalesce`
