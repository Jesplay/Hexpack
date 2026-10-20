---
aliases: ["Falling Purification II"]
tags: [hexcasting/pattern, mod/hexodus, family/purification, topic/movement]
kind: "Pattern"
family: "Purification"
topics: ["Movement"]
mod: "Hexodus"
op_id: "hexodus:get_true_direction"
signature: "weeeewedewqq"
start_direction: "NORTH_WEST"
input: "entity"
output: "vector"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Gravitation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Falling Purification II

◇ **Pattern** · [[Hexodus]] · 📖 [[Gravitation]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,127.4 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,127.4 27.0,127.4 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="65.7,127.4 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 85.0,94.0 65.7,127.4 27.0,127.4 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="127.4" x2="59.5" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,109.0 63.2,114.6 55.8,118.8" fill="#fc77be"/><circle cx="65.7" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeewedewqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `vector` ([[Vector]])
- **ID:** `hexodus:get_true_direction`

```text
before   … │ entity  ◂ top
after    … │ vector  ◂ top
```

## Description

Pushes the direction of **Gravity** acting on an entity.

## Related patterns (same book page)

- [[Gravitate]] `entity, vector, number → —`
- [[Grasp Purification]] `entity → number`
- [[Falling Purification]] `entity → vector`
- [[Impact Purification]] `entity → number`
- [[Impact Purification II]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Gravitation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexodus on Modrinth](https://modrinth.com/mod/bN6IKmdE)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetTrueGravity` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexodus:get_true_direction`
