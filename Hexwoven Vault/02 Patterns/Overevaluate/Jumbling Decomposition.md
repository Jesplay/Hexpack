---
aliases: ["Jumbling Decomposition"]
tags: [hexcasting/pattern, mod/overevaluate, family/decomposition, topic/stack, topic/data-structures]
kind: "Pattern"
family: "Decomposition"
topics: ["Stack Manipulation", "Sets, Maps & Special Iotas"]
mod: "Overevaluate"
op_id: "overevaluate:dissolve_jumble"
signature: "aedqa"
start_direction: "SOUTH_WEST"
input: "jumble"
output: "number, list"
cost: ""
iotas_in: ["[[Jumble]]"]
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Jumble Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Jumbling Decomposition

◇ **Pattern** · [[Overevaluate]] · 📖 [[Jumble Iota]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,29.8 29.8,66.6 72.3,66.6 93.5,103.4 51.0,103.4 29.8,140.2 72.3,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="51.0,29.8 29.8,66.6 72.3,66.6 93.5,103.4 51.0,103.4 29.8,140.2 72.3,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="29.8" x2="44.2" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,50.0 40.2,39.2 48.3,43.9" fill="#fc77be"/><circle cx="51.0" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aedqa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `jumble` ([[Jumble]])
- **Leaves:** `number, list` ([[Number]], [[List]])
- **ID:** `overevaluate:dissolve_jumble`

```text
before   … │ jumble         ◂ top
after    … │ number │ list  ◂ top
```

## Description

Dissolves a jumble [[Iotas|iota]] back into a number and a list of numbers.

## Related patterns (same book page)

- [[Jumbling Gambit]] `number, list → jumble`

## Sources

- 📖 **Guidebook:** [[Jumble Iota]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Jumbling Decomposition](https://overevaluate.hexxy.media/v/latest/main/en_us/#jumble@overevaluate:dissolve_jumble)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDissolveJumble` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:dissolve_jumble`
