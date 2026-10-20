---
aliases: ["Numerical Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/math]
kind: "Pattern"
family: "Reflection"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:number"
signature: "aqaa"
start_direction: "SOUTH_EAST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Number Literals]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Numerical Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Number Literals]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,85.0 65.1,132.0 92.3,85.0 65.1,38.0 38.0,85.0 92.3,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:271;--dur:1.6s" points="38.0,85.0 65.1,132.0 92.3,85.0 65.1,38.0 38.0,85.0 92.3,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="85.0" x2="46.7" y2="100.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="52.9,110.9 41.5,103.0 51.8,97.1" fill="#fc77be"/><circle cx="38.0" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexcasting:number`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Irritatingly, there is no easy way to draw numbers. Here is the method Nature deigned to give us.

First, I draw one of the two shapes shown on the other page. Next, the angles following will modify a running count starting at 0.
- Forward: Add 1
- Left: Add 5
- Right: Add 10
- Sharp Left: Multiply by 2
- Sharp Right: Divide by 2.

The clockwise version of the pattern, on the right of the other page, will negate the value at the very end. (The left-hand counter-clockwise version keeps the number positive).

Once I finish drawing, the number's pushed to the top of the [[The Stack|stack]].

## Example

Numbers are drawn, not typed. Start with `aqaa` (positive) or `dedd` (negative), then each extra stroke changes the value, starting from 0:
- `w` +1 · `q` +5 · `e` +10 · `a` ×2 · `d` ÷2

Examples: **10** = `aqaae` · **4** = `aqaawaa` (1, ×2, ×2) · **−5** = `deddq`

## Sources

- 📖 **Guidebook:** [[Number Literals]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:number`
