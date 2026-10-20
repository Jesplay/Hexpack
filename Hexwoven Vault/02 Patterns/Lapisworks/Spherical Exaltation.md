---
aliases: ["Spherical Exaltation"]
tags: [hexcasting/pattern, mod/lapisworks, family/exaltation, topic/control-flow]
kind: "Pattern"
family: "Exaltation"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:spherical_dstl"
signature: "wqwqwqwqwqwaeaqaaeaqaa"
start_direction: "NORTH_WEST"
input: "[pattern], vec, num"
output: "[any]"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]", "[[Pattern]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Enchanted Meta-eval]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Spherical Exaltation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Enchanted Meta-eval]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 116.5,76.6 100.7,49.3 85.0,76.6 100.7,103.8 116.5,76.6 85.0,76.6 69.3,49.3 53.5,76.6 69.3,103.8 85.0,76.6 53.5,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:724;--dur:6.0s" points="148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 116.5,76.6 100.7,49.3 85.0,76.6 100.7,103.8 116.5,76.6 85.0,76.6 69.3,49.3 53.5,76.6 69.3,103.8 85.0,76.6 53.5,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="148.0" y1="76.6" x2="142.9" y2="67.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="139.3,61.6 145.9,66.1 139.9,69.6" fill="#fc77be"/><circle cx="148.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqwqwaeaqaaeaqaa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern], vec, num` ([[Number]], [[Vector]], [[List]], [[Pattern]])
- **Leaves:** `[any]` ([[List]], [[Any Iota]])
- **ID:** `lapisworks:spherical_dstl`

```text
before   … │ [pattern] │ vec │ num  ◂ top
after    … │ [any]                  ◂ top
```

## Description

Evaluates (exactly like [[Thoth's Gambit]]) the given pattern list over a hollow sphere with the given radius (non-decimal, at least 1, at most 64) around the given position.

## Related patterns (same book page)

- [[Cubic Exaltation]] `[pattern], vec, vec, bool → [any]`
- [[Spherical Exaltation II]] `[pattern], vec, num → [any]`
- [[Mintiest Gambit]] `num, num, [pattern] → [any]`
- [[Kitkat's Gambit]] `num, [pattern] → —`
- [[Erebus' Gambit]] `positive num → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Meta-eval]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `SphereDst` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:spherical_dstl`
