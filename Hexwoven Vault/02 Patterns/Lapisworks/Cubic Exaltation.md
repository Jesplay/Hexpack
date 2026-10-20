---
aliases: ["Cubic Exaltation"]
tags: [hexcasting/pattern, mod/lapisworks, family/exaltation, topic/control-flow]
kind: "Pattern"
family: "Exaltation"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:cubic_exalt"
signature: "wqwawqwqqwqwq"
start_direction: "NORTH_WEST"
input: "[pattern], vec, vec, bool"
output: "[any]"
cost: ""
iotas_in: ["[[Vector]]", "[[Boolean]]", "[[List]]", "[[Pattern]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Enchanted Meta-eval]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cubic Exaltation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Enchanted Meta-eval]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="128.4,114.4 111.0,84.4 93.7,54.3 59.0,54.3 24.3,54.3 41.6,84.4 59.0,114.4 93.7,114.4 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 76.3,24.3 41.6,24.3 24.3,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:486;--dur:3.9s" points="128.4,114.4 111.0,84.4 93.7,54.3 59.0,54.3 24.3,54.3 41.6,84.4 59.0,114.4 93.7,114.4 128.4,114.4 145.7,84.4 128.4,54.3 111.0,24.3 76.3,24.3 41.6,24.3 24.3,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="114.4" x2="122.8" y2="104.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="118.8,97.9 126.1,102.9 119.5,106.7" fill="#fc77be"/><circle cx="128.4" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwawqwqqwqwq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern], vec, vec, bool` ([[Vector]], [[Boolean]], [[List]], [[Pattern]])
- **Leaves:** `[any]` ([[List]], [[Any Iota]])
- **ID:** `lapisworks:cubic_exalt`

```text
before   … │ [pattern] │ vec │ vec │ bool  ◂ top
after    … │ [any]                         ◂ top
```

## Description

Evaluates (exactly like [[Thoth's Gambit]]) the given pattern list over a (hollow if given True) cube-ish area from the given points A to B.

## Related patterns (same book page)

- [[Spherical Exaltation]] `[pattern], vec, num → [any]`
- [[Spherical Exaltation II]] `[pattern], vec, num → [any]`
- [[Mintiest Gambit]] `num, num, [pattern] → [any]`
- [[Kitkat's Gambit]] `num, [pattern] → —`
- [[Erebus' Gambit]] `positive num → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Meta-eval]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `CubeExalt` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:cubic_exalt`
