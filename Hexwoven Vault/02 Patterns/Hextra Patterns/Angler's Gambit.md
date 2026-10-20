---
aliases: ["Angler's Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hextra Patterns"
op_id: "hextrapats:duplicate_at"
signature: "wddad"
start_direction: "WEST"
input: "many"
output: "many"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Hextra Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Angler's Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="135.0,78.3 85.0,78.3 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="135.0,78.3 85.0,78.3 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="78.3" x2="119.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.5,78.3 119.0,72.8 119.0,83.8" fill="#fc77be"/><circle cx="135.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wddad` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many` (—)
- **Leaves:** `many` (—)
- **ID:** `hextrapats:duplicate_at`

## Description

A family of patterns that takes the Nth [[Iotas|iota]] from the top of the [[The Stack|stack]], determined by the length of the head, and duplicates it in place. The copy will be created right above the original iota.

The first pattern is identical to [[Gemini Decomposition]].

The second pattern, with a head length of 1, duplicates the first iota from the top. \[1, 0\] becomes \[1, 1, 0\].

The third pattern follows this. \[2, 1, 0\] becomes \[2, 2, 1, 0\].

## Related patterns (same book page)

- [[Capsizing Gambit]] `any → any`
- [[Capsizing Gambit II]] `any → any`
- [[Dredging Gambit]] `any → any`
- [[Dredging Gambit II]] `any → any`

## Sources

- 📖 **Guidebook:** [[Hextra Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hextrapats:duplicate_at`
