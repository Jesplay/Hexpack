---
aliases: ["Hermes' Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:eval"
signature: "deaqq"
start_direction: "SOUTH_EAST"
input: "evaluatable"
output: "many"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Hermes' Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,35.0 135.0,78.3 85.0,78.3 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="110.0,35.0 135.0,78.3 85.0,78.3 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="35.0" x2="118.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="123.8,58.8 113.2,51.6 122.8,46.1" fill="#fc77be"/><circle cx="110.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deaqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `evaluatable` ([[List]], [[Pattern]])
- **Leaves:** `many` (—)
- **ID:** `hexcasting:eval`

## Description

Remove something evaluatable (usually a pattern or an [[What is a Hex|entire Hex]]) from the [[The Stack|stack]], then cast it as if I had drawn the relevant patterns with my [[Staves|Staff]].

This can be very powerful in tandem with [[Focus|Foci]], and according to one esoteric scroll I found it also makes the bureaucracy of Nature a "Turing-complete" system.

Note that any [[Iotas|iotas]] other than patterns in a **Hex** cast this way will cause a [[Mishaps|mishap]] unless [[Escaping Patterns|escaped]]. In addition, with the energies of the patterns occurring without me to guide them, *any* mishap will cause the remaining actions in the **Hex** to immediately unravel.

## Example

[[Introspection]] → [[Mind's Reflection]] → [[Reveal]] → [[Retrospection]] → **Hermes' Gambit** runs the list, so your name appears in chat. Combined with [[Scribe's Reflection]] you can run a hex stored in a [[Focus]].

## Related patterns (same book page)

- [[Iris' Gambit]] `evaluatable → many`
- [[Thoth's Gambit]] `evaluatable, list → list`
- [[Charon's Gambit]] `— → —`
- [[Thanatos' Reflection]] `— → number`

## Sources

- 📖 **Guidebook:** [[Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Hermes' Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/meta@hexcasting:eval)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEval` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:eval`
