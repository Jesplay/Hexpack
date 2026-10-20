---
aliases: ["Nephthys' Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Overevaluate"
op_id: "overevaluate:nephthys"
signature: "deaqqd"
start_direction: "SOUTH_EAST"
input: "pattern/list"
output: "varies"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Advanced Metaevaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Nephthys' Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Advanced Metaevaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 114.8,66.6 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 72.3,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="93.5,29.8 114.8,66.6 72.3,66.6 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 72.3,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="100.3" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.2,50.0 96.3,43.9 104.4,39.2" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deaqqd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern/list` ([[List]], [[Pattern]])
- **Leaves:** `varies` (—)
- **ID:** `overevaluate:nephthys`

## Description

Pops a pattern or pattern list and dives down an amount corresponding to the tail length to cast the pattern list. Useful for surgery-like operations.

It's extremely versatile and I can mend almost any error in my [[The Stack|stack]] using [[Bookkeeper's Gambit]], pushing [[Iotas|iota]] after a dive, and transforming surfaced iota. It was named after a goddess of mourning and rivers, lending to how it buries/dives down into the stack.

*... the unknowns of time travel... is akin to descending blindly into the depths of the freezing water and reappearing as an acorn.*

## Related patterns (same book page)

- [[Athena's Gambit]] `list of patterns → boolean`
- [[Athena's Revelation]] `— → —`
- [[Athena's Reflection]] `— → garbage/text`
- [[Apep's Gambit]] `pattern/list, list → any`
- [[Tutu's Gambit]] `— → —`
- [[Janus' Gambit]] `— → —`
- [[Ma'at's Gambit]] `boolean, any → —`
- [[Atalanta's Gambit]] `— → —`
- [[Castor's Gambit]] `any → —`
- [[Pollux's Gambit]] `any → —`
- [[Sisyphus' Gambit]] `list → —`
- [[Themis' Gambit]] `pattern/list, list → list`

## Sources

- 📖 **Guidebook:** [[Advanced Metaevaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Nephthys' Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#metaevals@overevaluate:nephthys)
- ✏️ **Strokes:** from the book page, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNephthys` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:nephthys`
