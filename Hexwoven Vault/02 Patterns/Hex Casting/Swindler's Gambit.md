---
aliases: ["Swindler's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hex Casting"
op_id: "hexcasting:swizzle"
signature: "qaawdde"
start_direction: "SOUTH_EAST"
input: "many, number"
output: "many"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Swindler's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,35.0 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,121.6 35.0,78.3 85.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="60.0,35.0 85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,121.6 35.0,78.3 85.0,78.3 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="35.0" x2="68.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.8,58.8 63.2,51.6 72.8,46.1" fill="#fc77be"/><circle cx="60.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaawdde` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `many, number` ([[Number]])
- **Leaves:** `many` (—)
- **ID:** `hexcasting:swizzle`

## Description

Rearranges the top elements of the [[The Stack|stack]] based on the given numerical code, which is the index of the permutation wanted.

## Example

Give it a **permutation code** and it reorders the top few [[Iotas|iotas]] into any order you want. `0` does nothing, `1` swaps the top two (like [[Jester's Gambit]]); larger codes reach deeper. The book links a full table of codes. It replaces long chains of [[Jester's Gambit]] / [[Rotation Gambit]]. More in [[Gambits Explained]].

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
- [[Bookkeeper's Gambit]] `many → many`

## Sources

- 📖 **Guidebook:** [[Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Swindler's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/stackmanip@hexcasting:swizzle)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAlwinfyHasAscendedToABeingOfPureMath` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:swizzle`
