---
aliases: ["Apep's Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Overevaluate"
op_id: "overevaluate:apep"
signature: "dqd"
start_direction: "EAST"
input: "pattern/list, list"
output: "any"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Advanced Metaevaluation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Apep's Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Advanced Metaevaluation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="38.0,38.0 92.3,38.0 65.1,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="38.0,38.0 92.3,38.0 65.1,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.0" y1="38.0" x2="55.4" y2="38.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.8,38.0 55.4,44.0 55.4,32.0" fill="#fc77be"/><circle cx="38.0" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern/list, list` ([[List]], [[Pattern]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `overevaluate:apep`

```text
before   … │ pattern/list │ list  ◂ top
after    … │ any                  ◂ top
```

## Description

Folds across the second list using the **Hex** or pattern in the first argument.

Starts with the first two elements of the list and casts the **Hex** on them. Then, the top [[Iotas|iota]] remaining after casting and the next element in the list are placed on the [[The Stack|stack]] and casted upon, repeating until the entire list has been exhausted. The top iota after all that is finally pushed onto the stack.

*Euler did music too, you know*

## Related patterns (same book page)

- [[Athena's Gambit]] `list of patterns → boolean`
- [[Athena's Revelation]] `— → —`
- [[Athena's Reflection]] `— → garbage/text`
- [[Tutu's Gambit]] `— → —`
- [[Janus' Gambit]] `— → —`
- [[Ma'at's Gambit]] `boolean, any → —`
- [[Nephthys' Gambit]] `pattern/list → varies`
- [[Atalanta's Gambit]] `— → —`
- [[Castor's Gambit]] `any → —`
- [[Pollux's Gambit]] `any → —`
- [[Sisyphus' Gambit]] `list → —`
- [[Themis' Gambit]] `pattern/list, list → list`

## Sources

- 📖 **Guidebook:** [[Advanced Metaevaluation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Apep's Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#metaevals@overevaluate:apep)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpApep` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:apep`
