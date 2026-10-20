---
aliases: ["Erebus' Gambit"]
tags: [hexcasting/pattern, mod/lapisworks, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Lapisworks"
op_id: "lapisworks:the_cooler_halt"
signature: "wawqwdwewew"
start_direction: "SOUTH_WEST"
input: "positive num"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Enchanted Meta-eval]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Erebus' Gambit

◇ **Pattern** · [[Lapisworks]] · 📖 [[Enchanted Meta-eval]] · Lapisworks

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 85.0,76.6 100.7,49.3 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:378;--dur:3.4s" points="53.5,22.0 37.8,49.3 22.0,76.6 53.5,76.6 85.0,76.6 100.7,49.3 116.5,22.0 132.2,49.3 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="22.0" x2="48.5" y2="30.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,37.0 45.5,29.0 51.5,32.5" fill="#fc77be"/><circle cx="53.5" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawqwdwewew` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `positive num` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:the_cooler_halt`

```text
before   … │ positive num  ◂ top
after    …                 ◂ top
```

## Description

[[Charon's Gambit]], if it could break through more than one layer. This pattern is useful in cases where [[Charon's Gambit]] is insufficient, however I wish not to delve into the complicated beast that is [[Iris' Gambit]].

## Related patterns (same book page)

- [[Cubic Exaltation]] `[pattern], vec, vec, bool → [any]`
- [[Spherical Exaltation]] `[pattern], vec, num → [any]`
- [[Spherical Exaltation II]] `[pattern], vec, num → [any]`
- [[Mintiest Gambit]] `num, num, [pattern] → [any]`
- [[Kitkat's Gambit]] `num, [pattern] → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Meta-eval]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTheCoolerHalt` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:the_cooler_halt`
