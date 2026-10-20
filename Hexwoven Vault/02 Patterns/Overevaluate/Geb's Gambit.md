---
aliases: ["Geb's Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Overevaluate"
op_id: "overevaluate:geb"
signature: "aaeaad"
start_direction: "WEST"
input: "many"
output: "many"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Extra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Geb's Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Extra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 35.0,78.3 60.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="85.0,78.3 35.0,78.3 60.0,121.6 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="69.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.5,78.3 69.0,72.8 69.0,83.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaeaad` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many` (—)
- **Leaves:** `many` (—)
- **ID:** `overevaluate:geb`

## Description

Yanks the [[Iotas|iota]] n from the top up to the top, determined by tail length. In the examples above, the iota second, third, and fourth from the top are yanked respectively.

## Related patterns (same book page)

- [[Sekhmet's Gambit]] `— → varies`
- [[Nut's Gambit]] `many → many`
- [[Reflecting Gambit]] `any, any, any → any, any, any`
- [[Bubbling Gambit]] `any, any, any → any, any, any`
- [[Dioscuri Gambit II]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Extra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Geb's Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#extra_stack@overevaluate:geb)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGeb` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:geb`
