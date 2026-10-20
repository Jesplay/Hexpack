---
aliases: ["Styx's Gambit"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hextra Patterns"
op_id: "hextrapats:false_halt"
signature: "aqdeeaaqa"
start_direction: "SOUTH_WEST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Hextra Meta-Evaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Styx's Gambit

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Meta-Evaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,29.8 29.8,66.6 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 51.0,103.4 72.3,140.2 93.5,103.4 72.3,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="51.0,29.8 29.8,66.6 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 51.0,103.4 72.3,140.2 93.5,103.4 72.3,66.6 51.0,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="29.8" x2="44.2" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,50.0 40.2,39.2 48.3,43.9" fill="#fc77be"/><circle cx="51.0" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqdeeaaqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hextrapats:false_halt`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Forcibly halts a **Hex** like [[Charon's Gambit]] if the [[Iotas|iota]] at the top of the [[The Stack|stack]] is [[Augur's Purification|considered False by Nature]].

## Related patterns (same book page)

- [[Helios' Gambit]] `any, ([pattern] | pattern) → many`
- [[Selene's Gambit]] `any, ([pattern] | pattern) → many`
- [[Acheron's Gambit]] `any → —`
- [[Djehuty's Gambit]] `[pattern], list → list`
- [[Anubis' Gambit]] `many, [pattern], list → many`
- [[Ra's Gambit]] `many, [pattern], list → many`

## Sources

- 📖 **Guidebook:** [[Hextra Meta-Evaluation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpConditionalHalt` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:false_halt`
