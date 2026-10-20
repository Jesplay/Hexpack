---
aliases: ["Range Exaltation"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/exaltation, topic/logic]
kind: "Pattern"
family: "Exaltation"
topics: ["Logic & Comparison"]
mod: "Hextra Patterns"
op_id: "hextrapats:in_range"
signature: "qqqq"
start_direction: "SOUTH_WEST"
input: "num/vec, num, num, num"
output: "bool"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Hextra Logic]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Range Exaltation

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Logic]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:1.6s" points="60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="35.0" x2="52.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.2,58.8 47.2,46.1 56.8,51.6" fill="#fc77be"/><circle cx="60.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num/vec, num, num, num` ([[Number]], [[Vector]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hextrapats:in_range`

```text
before   … │ num/vec │ num │ num │ num  ◂ top
after    … │ bool                       ◂ top
```

## Description

If the first number is between the second and third, return True. Otherwise, return False. If the first value is a vector, compare its length instead.

The fourth number seems to describe how to check if the value is in the range.
- 0: min &lt; val &lt; max
- 1: min &lt;= val &lt; max
- 2: min &lt; val &lt;= max
- 3: min &lt;= val &lt;= max

## Related patterns (same book page)

- [[Length Equality Distillation]] `vec, vec → bool`
- [[Length Inequality Distillation]] `vec, vec → bool`
- [[Range Exaltation II]] `num/vec, num, num, num → bool`

## Sources

- 📖 **Guidebook:** [[Hextra Logic]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:in_range`
