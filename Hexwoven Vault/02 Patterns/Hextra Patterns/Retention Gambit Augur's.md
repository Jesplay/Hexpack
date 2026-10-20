---
aliases: ["Retention Gambit: Augur's"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/logic]
kind: "Pattern"
family: "Gambit"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nocon/bool_coerce"
signature: "ddaw"
start_direction: "SOUTH_EAST"
input: "A"
output: "A, bool"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Boolean]]", "[[Any Iota]]"]
entry: "[[Non-consuming Logic]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Retention Gambit: Augur's

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Non-consuming Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/></g><polyline class="hexpat-base" points="117.7,30.5 139.5,68.3 95.9,68.3 117.7,30.5 74.1,30.5 30.5,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:218;--dur:1.6s" points="117.7,30.5 139.5,68.3 95.9,68.3 117.7,30.5 74.1,30.5 30.5,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="117.7" y1="30.5" x2="124.7" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="129.7,51.3 120.5,45.0 128.8,40.2" fill="#fc77be"/><circle cx="117.7" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `A` ([[Any Iota]])
- **Leaves:** `A, bool` ([[Boolean]], [[Any Iota]])
- **ID:** `hextrapats:nocon/bool_coerce`

```text
before   … │ A         ◂ top
after    … │ A │ bool  ◂ top
```

## Description

Convert an argument to a boolean. The number **0**, **Null**, False, the vector (0, 0, 0), and the empty list become False; everything else becomes True.

## Related patterns (same book page)

- [[Retention Gambit Equality]] `A, B → A, B, bool`
- [[Retention Gambit Inequality]] `A, B → A, B, bool`
- [[Retention Gambit Maximus]] `A, B → A, B, bool`
- [[Retention Gambit Minimus]] `A, B → A, B, bool`
- [[Retention Gambit Maximus II]] `A, B → A, B, bool`
- [[Retention Gambit Minimus II]] `A, B → A, B, bool`
- [[Retention Gambit Length Equality]] `A, B → A, B, bool`
- [[Retention Gambit Length Inequality]] `A, B → A, B, bool`

## Sources

- 📖 **Guidebook:** [[Non-consuming Logic]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNoConsBoolCoerce` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:nocon/bool_coerce`
