---
aliases: ["Reviewer's Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/storage]
kind: "Pattern"
family: "Purification"
topics: ["Reading & Writing"]
mod: "Lapisworks"
op_id: "lapisworks:readable_spechand"
signature: "qqqqadww"
start_direction: "NORTH_WEST"
input: "num"
output: "bool"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Specific Read-write]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Reviewer's Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Specific Read-write]] · Lapisworks

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 53.5,49.3 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:283;--dur:2.5s" points="85.0,49.3 69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 53.5,49.3 85.0,49.3 116.5,49.3 148.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="80.0" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,34.3 83.0,38.8 77.0,42.3" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqadww` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:readable_spechand`

```text
before   … │ num   ◂ top
after    … │ bool  ◂ top
```

## Description

Specific hand version of [[Auditor's Reflection]]. Pushes whether the item in the specified hand can be read from or not.

## Related patterns (same book page)

- [[Secretary's Purification]] `num → any`
- [[Amanuensis' Gambit]] `any, num → —`
- [[Sanctioner's Purification]] `num → bool`

## Sources

- 📖 **Guidebook:** [[Specific Read-write]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `ReadableInHand` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:readable_spechand`
