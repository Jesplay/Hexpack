---
aliases: ["Fisherman's Gambit II"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/stack]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation"]
mod: "Hex Casting"
op_id: "hexcasting:fisherman/copy"
signature: "aada"
start_direction: "EAST"
input: "number"
output: "any"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Stack Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Fisherman's Gambit II

◇ **Pattern** · [[Hex Casting]] · 📖 [[Stack Manipulation]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,35.0 35.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:1.6s" points="85.0,78.3 135.0,78.3 110.0,35.0 85.0,78.3 60.0,35.0 35.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aada` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number` ([[Number]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexcasting:fisherman/copy`

```text
before   … │ number  ◂ top
after    … │ any     ◂ top
```

## Description

Like [[Fisherman's Gambit]], but instead of moving the [[Iotas|iota]], copies it.

## Example

Same as [[Fisherman's Gambit]], but **copies** the iota instead of moving it, so the original stays where it was. Perfect for "constants" you set up at the start of a hex and reuse several times.

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
- [[Bookkeeper's Gambit]] `many → many`
- [[Swindler's Gambit]] `many, number → many`

## Sources

- 📖 **Guidebook:** [[Stack Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Fisherman's Gambit II](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/stackmanip@hexcasting:fisherman/copy)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFishermanButItCopies` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:fisherman/copy`
