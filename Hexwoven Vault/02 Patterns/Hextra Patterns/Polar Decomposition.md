---
aliases: ["Polar Decomposition"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/decomposition, topic/math]
kind: "Pattern"
family: "Decomposition"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:to_polar"
signature: "qedadeed"
start_direction: "EAST"
input: "vec"
output: "num, num"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hextra Vector Math]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Polar Decomposition

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Vector Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 85.0,27.0 123.6,27.0 104.3,60.5 143.0,60.5 123.6,94.0 85.0,94.0 65.7,60.5 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="27.0,60.5 65.7,60.5 85.0,27.0 123.6,27.0 104.3,60.5 143.0,60.5 123.6,94.0 85.0,94.0 65.7,60.5 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qedadeed` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `num, num` ([[Number]])
- **ID:** `hextrapats:to_polar`

```text
before   … │ vec        ◂ top
after    … │ num │ num  ◂ top
```

## Description

Converts a vector's direction into two polar angles, pitch and yaw, in radians. +Y is up. 0 yaw is +Z, pi/2 yaw is -X. Yet again, mentions of [[Sextant Decomposition|F3]] turn up.

## Related patterns (same book page)

- [[Norm Purification]] `vec → vec`
- [[Length Distillation]] `vec, num → vec`
- [[Chaos Reflection]] `— → vec`
- [[Distance Distillation]] `vec, vec → num`
- [[Polar Distillation]] `num, num → vec`
- [[Pitch Distillation]] `vec, num → vec`
- [[Yaw Distillation]] `vec, num → vec`
- [[Roll Distillation]] `vec, num → vec`
- [[Pitch Purification]] `num → vec`
- [[Yaw Purification]] `num → vec`
- [[Roll Purification]] `num → vec`

## Sources

- 📖 **Guidebook:** [[Hextra Vector Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpToPolar` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:to_polar`
