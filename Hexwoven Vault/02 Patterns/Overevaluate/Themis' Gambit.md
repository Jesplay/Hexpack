---
aliases: ["Themis' Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Overevaluate"
op_id: "overevaluate:themis"
signature: "dwaad"
start_direction: "WEST"
input: "pattern/list, list"
output: "list"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: ["[[List]]"]
entry: "[[Advanced Metaevaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Themis' Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Advanced Metaevaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="146.5" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="119.4,132.0 65.1,132.0 92.3,85.0 119.4,38.0 65.1,38.0 92.3,85.0 38.0,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:326;--dur:1.7s" points="119.4,132.0 65.1,132.0 92.3,85.0 119.4,38.0 65.1,38.0 92.3,85.0 38.0,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="119.4" y1="132.0" x2="102.0" y2="132.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="89.6,132.0 102.0,126.0 102.0,138.0" fill="#fc77be"/><circle cx="119.4" cy="132.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwaad` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern/list, list` ([[List]], [[Pattern]])
- **Leaves:** `list` ([[List]])
- **ID:** `overevaluate:themis`

```text
before   … │ pattern/list │ list  ◂ top
after    … │ list                 ◂ top
```

## Description

Takes a pattern or pattern list and casts over every element the second list similar to [[Thoth's Gambit]] in order to sort it.

After each iteration, it demands a number at the top of the [[The Stack|stack]] which will correspond to that [[Iotas|iota]]. At the end of the gambit, a list is pushed with the iota sorted by that number in increasing order. The order of identically numbered iota are kept. If it encounters [[Charon's Gambit]], it will create a sorted list with only the iota it has come across.

*Shuffling is the only thing which Nature cannot undo.*

## Related patterns (same book page)

- [[Athena's Gambit]] `list of patterns → boolean`
- [[Athena's Revelation]] `— → —`
- [[Athena's Reflection]] `— → garbage/text`
- [[Apep's Gambit]] `pattern/list, list → any`
- [[Tutu's Gambit]] `— → —`
- [[Janus' Gambit]] `— → —`
- [[Ma'at's Gambit]] `boolean, any → —`
- [[Nephthys' Gambit]] `pattern/list → varies`
- [[Atalanta's Gambit]] `— → —`
- [[Castor's Gambit]] `any → —`
- [[Pollux's Gambit]] `any → —`
- [[Sisyphus' Gambit]] `list → —`

## Sources

- 📖 **Guidebook:** [[Advanced Metaevaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Themis' Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#metaevals@overevaluate:themis)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpThemis` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:themis`
