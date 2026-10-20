---
aliases: ["Retention Gambit: Inequality"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/logic]
kind: "Pattern"
family: "Gambit"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nocon/neq"
signature: "ddqda"
start_direction: "WEST"
input: "A, B"
output: "A, B, bool"
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

# Retention Gambit: Inequality

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Non-consuming Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="74.1,68.3 30.5,68.3 52.3,30.5 74.1,68.3 117.7,68.3 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:262;--dur:1.7s" points="74.1,68.3 30.5,68.3 52.3,30.5 74.1,68.3 117.7,68.3 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="74.1" y1="68.3" x2="60.2" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="50.1,68.3 60.2,63.5 60.2,73.1" fill="#fc77be"/><circle cx="74.1" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddqda` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `A, B` ([[Any Iota]])
- **Leaves:** `A, B, bool` ([[Boolean]], [[Any Iota]])
- **ID:** `hextrapats:nocon/neq`

```text
before   … │ A │ B         ◂ top
after    … │ A │ B │ bool  ◂ top
```

## Description

If the first argument does not equal the second (outside a small tolerance), return True. Otherwise, return False.

## Related patterns (same book page)

- [[Retention Gambit Augur's]] `A → A, bool`
- [[Retention Gambit Equality]] `A, B → A, B, bool`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNoConsEquality` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:nocon/neq`
