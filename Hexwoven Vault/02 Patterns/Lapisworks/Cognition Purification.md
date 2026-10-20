---
aliases: ["Cognition Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Lapisworks"
op_id: "lapisworks:check_mind"
signature: "aawqqwqqq"
start_direction: "WEST"
input: ""
output: "vec"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Simple Artificial Minds]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cognition Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Simple Artificial Minds]] · Lapisworks › Imbuement

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="123.6,94.0 85.0,94.0 104.3,127.4 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="123.6,94.0 85.0,94.0 104.3,127.4 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="94.0" x2="111.3" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.4,94.0 111.3,89.7 111.3,98.2" fill="#fc77be"/><circle cx="123.6" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawqqwqqq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vec` ([[Vector]])
- **ID:** `lapisworks:check_mind`

```text
before   …        ◂ top
after    … │ vec  ◂ top
```

## Description

Pushes the percentage filled of the mind container at the given position to the [[The Stack|stack]].

## Related patterns (same book page)

- [[Sieve Thoughts]] `vec → —`
- [[Mind Liquefaction]] `vec → —`

## Sources

- 📖 **Guidebook:** [[Simple Artificial Minds]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CognitionPrfn` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:check_mind`
