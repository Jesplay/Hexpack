---
aliases: ["Retention Gambit: Minimus"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/logic]
kind: "Pattern"
family: "Gambit"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:nocon/less"
signature: "ddeq"
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

# Retention Gambit: Minimus

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Non-consuming Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,66.6 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 72.3,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="72.3,66.6 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 72.3,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="66.6" x2="58.7" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.9,66.6 58.7,61.9 58.7,71.3" fill="#fc77be"/><circle cx="72.3" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddeq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `A, B` ([[Any Iota]])
- **Leaves:** `A, B, bool` ([[Boolean]], [[Any Iota]])
- **ID:** `hextrapats:nocon/less`

```text
before   … │ A │ B         ◂ top
after    … │ A │ B │ bool  ◂ top
```

## Description

If the first argument is less than the second, return True. Otherwise, return False.

## Related patterns (same book page)

- [[Retention Gambit Augur's]] `A → A, bool`
- [[Retention Gambit Equality]] `A, B → A, B, bool`
- [[Retention Gambit Inequality]] `A, B → A, B, bool`
- [[Retention Gambit Maximus]] `A, B → A, B, bool`
- [[Retention Gambit Maximus II]] `A, B → A, B, bool`
- [[Retention Gambit Minimus II]] `A, B → A, B, bool`
- [[Retention Gambit Length Equality]] `A, B → A, B, bool`
- [[Retention Gambit Length Inequality]] `A, B → A, B, bool`

## Sources

- 📖 **Guidebook:** [[Non-consuming Logic]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:nocon/less`
