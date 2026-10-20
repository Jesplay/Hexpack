---
aliases: ["Charon's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:halt"
signature: "aqdee"
start_direction: "SOUTH_WEST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Charon's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="60.0,35.0 35.0,78.3 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 60.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="60.0,35.0 35.0,78.3 85.0,78.3 110.0,35.0 135.0,78.3 110.0,121.6 60.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="35.0" x2="52.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="46.2,58.8 47.2,46.1 56.8,51.6" fill="#fc77be"/><circle cx="60.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqdee` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:halt`

## Description

This pattern forcibly halts a **Hex**. This is mostly useless on its own, as I could simply just stop writing patterns, or put down my [[Staves|staff]].

But when combined with [[Hermes' Gambit|Hermes']] or [[Thoth's Gambit|Thoth's Gambits]], it becomes far more interesting. Those patterns serve to 'contain' that halting, and rather than ending the entire **Hex**, those gambits end instead. This can be used to cause [[Thoth's Gambit]] not to operate on every [[Iotas|iota]] it's given. An escape from the madness, as it were.

## Example

Stops the hex immediately. Inside [[Hermes' Gambit]] or [[Thoth's Gambit]] only *that* gambit ends, not the whole hex, so in Thoth's it works like a "skip this element".

## Related patterns (same book page)

- [[Hermes' Gambit]] `evaluatable → many`
- [[Iris' Gambit]] `evaluatable → many`
- [[Thoth's Gambit]] `evaluatable, list → list`
- [[Thanatos' Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Charon's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/meta@hexcasting:halt)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpHalt` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:halt`
