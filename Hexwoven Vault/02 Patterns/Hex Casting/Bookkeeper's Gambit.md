---
aliases: ["Bookkeeper's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hex Casting"
op_id: "hexcasting:mask"
signature: "aeea"
start_direction: "SOUTH_EAST"
input: "many"
output: "many"
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Bookkeeper's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="88" viewBox="0 0 170.0 87.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/></g><polyline class="hexpat-base" points="27.0,27.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:193;--dur:1.6s" points="27.0,27.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5 143.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="27.0" x2="33.2" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.7,45.4 29.5,39.9 36.9,35.6" fill="#fc77be"/><circle cx="27.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeea` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many` (—)
- **Leaves:** `many` (—)
- **ID:** `hexcasting:mask`

## Description

An infinite family of actions that keep or remove elements at the top of the [[The Stack|stack]] based on the sequence of dips and lines.

Assuming that I draw a Bookkeeper's Gambit pattern left-to-right, the number of [[Iotas|iotas]] the action will require is determined by the horizontal distance covered by the pattern. From deepest in the stack to shallowest, a flat line will keep the iota, whereas a triangle dipping down will remove it.

If my stack contains 0, 1, 2 from deepest to shallowest, drawing the first pattern opposite will give me 1, the second will give me 0, and the third will give me 0, 2 (the 0 at the bottom is left untouched).

## Example

**Bookkeeper's Gambit** is a whole *family* of patterns. Read it left to right, one segment per stack slot, deepest first: a **flat line keeps** that iota, a **dip removes** it.
- stack `0, 1, 2`: `—v` → `0, 1` · `v—` → `0, 2` · `v` alone → drops the top (that's [[Novice's Gambit]])

Use it to throw away leftovers, especially inside [[Thoth's Gambit]] where every leftover lands in the result list. More in [[Gambits Explained]].

## Related patterns (same book page)

- [[Novice's Gambit]] `any → —`
- [[Jester's Gambit]] `any, any → any, any`
- [[Rotation Gambit]] `any, any, any → any, any, any`
- [[Rotation Gambit II]] `any, any, any → any, any, any`
- [[Gemini Decomposition]] `any → any, any`
- [[Prospector's Gambit]] `any, any → any, any, any`
- [[Undertaker's Gambit]] `any, any → any, any, any`
- [[Gemini Gambit]] `any, number → many`
- [[Dioscuri Gambit]] `any, any → any, any, any, any`
- [[Flock's Reflection]] `— → number`
- [[Fisherman's Gambit]] `number → any`
- [[Fisherman's Gambit II]] `number → any`
- [[Swindler's Gambit]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Bookkeeper's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/stackmanip@hexcasting:mask)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:mask`
