---
aliases: ["Dioscuri Gambit II"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Overevaluate"
op_id: "overevaluate:dup_many"
signature: "waadadaa"
start_direction: "EAST"
input: "many, number"
output: "many"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Extra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Dioscuri Gambit II

◇ **Pattern** · [[Overevaluate]] · 📖 [[Extra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 104.3,60.5 143.0,60.5 123.6,27.0 104.3,60.5 85.0,27.0 65.7,60.5 46.4,27.0 27.0,60.5 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="65.7,60.5 104.3,60.5 143.0,60.5 123.6,27.0 104.3,60.5 85.0,27.0 65.7,60.5 46.4,27.0 27.0,60.5 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="78.0" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.9,60.5 78.0,64.8 78.0,56.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waadadaa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many, number` ([[Number]])
- **Leaves:** `many` (—)
- **ID:** `overevaluate:dup_many`

## Description

Takes a number and duplicates that many elements of the [[The Stack|stack]] while preserving order.

## Related patterns (same book page)

- [[Sekhmet's Gambit]] `— → varies`
- [[Geb's Gambit]] `many → many`
- [[Nut's Gambit]] `many → many`
- [[Reflecting Gambit]] `any, any, any → any, any, any`
- [[Bubbling Gambit]] `any, any, any → any, any, any`

## Sources

- 📖 **Guidebook:** [[Extra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Dioscuri Gambit II](https://overevaluate.hexxy.media/v/latest/main/en_us/#extra_stack@overevaluate:dup_many)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDioscuriII` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:dup_many`
