---
aliases: ["Prospector's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hex Casting"
op_id: "hexcasting:over"
signature: "aaedd"
start_direction: "EAST"
input: "any, any"
output: "any, any, any"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Prospector's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaedd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, any` ([[Any Iota]])
- **Leaves:** `any, any, any` ([[Any Iota]])
- **ID:** `hexcasting:over`

```text
before   … │ any │ any        ◂ top
after    … │ any │ any │ any  ◂ top
```

## Description

Copy the second-to-last [[Iotas|iota]] of the [[The Stack|stack]] to the top. \[0, 1\] becomes \[0, 1, 0\].

## Example

`a, b → a, b, a`: copies the second-from-top iota onto the top.

## Related patterns (same book page)

- [[Novice's Gambit]] `any → —`
- [[Jester's Gambit]] `any, any → any, any`
- [[Rotation Gambit]] `any, any, any → any, any, any`
- [[Rotation Gambit II]] `any, any, any → any, any, any`
- [[Gemini Decomposition]] `any → any, any`
- [[Undertaker's Gambit]] `any, any → any, any, any`
- [[Gemini Gambit]] `any, number → many`
- [[Dioscuri Gambit]] `any, any → any, any, any, any`
- [[Flock's Reflection]] `— → number`
- [[Fisherman's Gambit]] `number → any`
- [[Fisherman's Gambit II]] `number → any`
- [[Bookkeeper's Gambit]] `many → many`
- [[Swindler's Gambit]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Prospector's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/stackmanip@hexcasting:over)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTwiddling` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:over`
