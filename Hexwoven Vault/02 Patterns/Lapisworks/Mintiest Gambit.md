---
aliases: ["Mintiest Gambit"]
tags: [hexcasting/pattern, mod/lapisworks, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:for_n_in_range"
signature: "aqadadad"
start_direction: "NORTH_WEST"
input: "num, num, [pattern]"
output: "[any]"
cost: ""
iotas_in: ["[[Number]]", "[[List]]", "[[Pattern]]"]
iotas_out: ["[[List]]", "[[Any Iota]]"]
entry: "[[Enchanted Meta-eval]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mintiest Gambit

◇ **Pattern** · [[Lapisworks]] · 📖 [[Enchanted Meta-eval]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="59.0,54.3 41.6,24.3 24.3,54.3 41.6,84.4 59.0,54.3 76.3,84.4 93.7,54.3 111.0,84.4 128.4,54.3 145.7,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:312;--dur:2.5s" points="59.0,54.3 41.6,24.3 24.3,54.3 41.6,84.4 59.0,54.3 76.3,84.4 93.7,54.3 111.0,84.4 128.4,54.3 145.7,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.0" y1="54.3" x2="53.4" y2="44.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.4,37.8 56.7,42.8 50.1,46.6" fill="#fc77be"/><circle cx="59.0" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqadadad` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num, num, [pattern]` ([[Number]], [[List]], [[Pattern]])
- **Leaves:** `[any]` ([[List]], [[Any Iota]])
- **ID:** `lapisworks:for_n_in_range`

```text
before   … │ num │ num │ [pattern]  ◂ top
after    … │ [any]                  ◂ top
```

## Description

*A oney and a twoey and a you-know-what-to-doey!*

Executes the given pattern list over a list of in(or de)crementing numbers, ranging from the first argument (inclusive) to the second (exclusive) as if [[Thoth's Gambit]] were used.

## Related patterns (same book page)

- [[Cubic Exaltation]] `[pattern], vec, vec, bool → [any]`
- [[Spherical Exaltation]] `[pattern], vec, num → [any]`
- [[Spherical Exaltation II]] `[pattern], vec, num → [any]`
- [[Kitkat's Gambit]] `num, [pattern] → —`
- [[Erebus' Gambit]] `positive num → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Meta-eval]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpForNInRange` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:for_n_in_range`
