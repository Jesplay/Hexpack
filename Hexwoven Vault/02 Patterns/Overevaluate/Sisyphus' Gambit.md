---
aliases: ["Sisyphus' Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Overevaluate"
op_id: "overevaluate:sisyphus"
signature: "qaqwede"
start_direction: "NORTH_EAST"
input: "list"
output: ""
cost: ""
iotas_in: ["[[List]]"]
iotas_out: []
entry: "[[Advanced Metaevaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sisyphus' Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Advanced Metaevaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="84" height="170" viewBox="0 0 83.9 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="41.9,85.0 59.4,54.7 41.9,24.5 24.5,54.7 41.9,85.0 59.4,115.3 41.9,145.5 24.5,115.3 41.9,85.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:280;--dur:2.2s" points="41.9,85.0 59.4,54.7 41.9,24.5 24.5,54.7 41.9,85.0 59.4,115.3 41.9,145.5 24.5,115.3 41.9,85.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="85.0" x2="47.5" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.6,68.4 50.9,77.2 44.2,73.4" fill="#fc77be"/><circle cx="41.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqwede` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `nothing` (—)
- **ID:** `overevaluate:sisyphus`

```text
before   … │ list  ◂ top
after    …         ◂ top
```

## Description

Takes a pattern list and casts it over and over forever until it encounters either a [[Charon's Gambit]] or a [[Janus' Gambit]].

While I have heard of self-producing **Hexes** called *quines* used to make infinite loops, they can be slightly unwieldy to work with. This gambit simplifies that process by just repeatedly casting a list until it runs out of evaluations or encounters a [[Charon's Gambit]]. The [[Reading and Writing|Ravenmind]] and [[The Stack|stack]] are carried across iterations.

*One always finds one's burden again... One must imagine Sisyphus happy.*

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
- [[Themis' Gambit]] `pattern/list, list → list`

## Sources

- 📖 **Guidebook:** [[Advanced Metaevaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Sisyphus' Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#metaevals@overevaluate:sisyphus)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSisyphus` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:sisyphus`
