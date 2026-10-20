---
aliases: ["Retention Gambit: Length Inequality"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/logic]
kind: "Pattern"
family: "Gambit"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nocon/len_neq"
signature: "ddqdaeedew"
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

# Retention Gambit: Length Inequality

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Non-consuming Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="146.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="146.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/><circle cx="146.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,54.7 24.5,54.7 41.9,24.5 59.4,54.7 94.4,54.7 76.9,85.0 111.8,85.0 129.3,115.3 111.8,145.5 94.4,115.3 111.8,85.0 129.3,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="59.4,54.7 24.5,54.7 41.9,24.5 59.4,54.7 94.4,54.7 76.9,85.0 111.8,85.0 129.3,115.3 111.8,145.5 94.4,115.3 111.8,85.0 129.3,54.7" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="54.7" x2="48.2" y2="54.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="40.2,54.7 48.2,50.9 48.2,58.6" fill="#fc77be"/><circle cx="59.4" cy="54.7" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddqdaeedew` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `A, B` ([[Any Iota]])
- **Leaves:** `A, B, bool` ([[Boolean]], [[Any Iota]])
- **ID:** `hextrapats:nocon/len_neq`

```text
before   … │ A │ B         ◂ top
after    … │ A │ B │ bool  ◂ top
```

## Description

If the two vectors are not the same length (outside small tolerance), return True. Otherwise, return False.

## Related patterns (same book page)

- [[Retention Gambit Augur's]] `A → A, bool`
- [[Retention Gambit Equality]] `A, B → A, B, bool`
- [[Retention Gambit Inequality]] `A, B → A, B, bool`
- [[Retention Gambit Maximus]] `A, B → A, B, bool`
- [[Retention Gambit Minimus]] `A, B → A, B, bool`
- [[Retention Gambit Maximus II]] `A, B → A, B, bool`
- [[Retention Gambit Minimus II]] `A, B → A, B, bool`
- [[Retention Gambit Length Equality]] `A, B → A, B, bool`

## Sources

- 📖 **Guidebook:** [[Non-consuming Logic]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:nocon/len_neq`
