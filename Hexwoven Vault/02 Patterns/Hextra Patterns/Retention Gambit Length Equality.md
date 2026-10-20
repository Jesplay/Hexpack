---
aliases: ["Retention Gambit: Length Equality"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/logic]
kind: "Pattern"
family: "Gambit"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nocon/len_eq"
signature: "ddqadqqaqw"
start_direction: "WEST"
input: "A, B"
output: "A, B, bool"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Boolean]]", "[[Any Iota]]"]
entry: "[[Non-consuming Logic]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Retention Gambit: Length Equality

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Non-consuming Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,127.4 27.0,127.4 46.4,94.0 65.7,127.4 104.3,127.4 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 123.6,94.0 143.0,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,127.4 27.0,127.4 46.4,94.0 65.7,127.4 104.3,127.4 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 104.3,60.5 123.6,94.0 143.0,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="127.4" x2="53.3" y2="127.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.4,127.4 53.3,123.2 53.3,131.7" fill="#fc77be"/><circle cx="65.7" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddqadqqaqw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `A, B` ([[Any Iota]])
- **Leaves:** `A, B, bool` ([[Boolean]], [[Any Iota]])
- **ID:** `hextrapats:nocon/len_eq`

```text
before   … │ A │ B         ◂ top
after    … │ A │ B │ bool  ◂ top
```

## Description

If the two vectors are the same length (within small tolerance), return True. Otherwise, return False.

## Related patterns (same book page)

- [[Retention Gambit Augur's]] `A → A, bool`
- [[Retention Gambit Equality]] `A, B → A, B, bool`
- [[Retention Gambit Inequality]] `A, B → A, B, bool`
- [[Retention Gambit Maximus]] `A, B → A, B, bool`
- [[Retention Gambit Minimus]] `A, B → A, B, bool`
- [[Retention Gambit Maximus II]] `A, B → A, B, bool`
- [[Retention Gambit Minimus II]] `A, B → A, B, bool`
- [[Retention Gambit Length Inequality]] `A, B → A, B, bool`

## Sources

- 📖 **Guidebook:** [[Non-consuming Logic]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:nocon/len_eq`
