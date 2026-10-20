---
aliases: ["Jumbling Gambit"]
tags: [hexcasting/pattern, mod/overevaluate, family/gambit, topic/stack, topic/data-structures]
kind: "Pattern"
family: "Gambit"
topics: ["Stack Manipulation", "Sets, Maps & Special Iotas"]
mod: "Overevaluate"
op_id: "overevaluate:create_jumble"
signature: "deaqd"
start_direction: "WEST"
input: "number, list"
output: "jumble"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[Jumble]]"]
entry: "[[Jumble Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Jumbling Gambit

◇ **Pattern** · [[Overevaluate]] · 📖 [[Jumble Iota]] · Patterns

**Topics:** [[Stack Manipulation (Topic)|Stack Manipulation]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,140.2 29.8,140.2 51.0,103.4 93.5,103.4 72.3,66.6 29.8,66.6 51.0,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="72.3,140.2 29.8,140.2 51.0,103.4 93.5,103.4 72.3,66.6 29.8,66.6 51.0,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="140.2" x2="58.7" y2="140.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.9,140.2 58.7,135.6 58.7,144.9" fill="#fc77be"/><circle cx="72.3" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deaqd` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `number, list` ([[Number]], [[List]])
- **Leaves:** `jumble` ([[Jumble]])
- **ID:** `overevaluate:create_jumble`

```text
before   … │ number │ list  ◂ top
after    … │ jumble         ◂ top
```

## Description

Creates a jumble [[Iotas|iota]] corresponding to the argument count and the list of numbers.

## Related patterns (same book page)

- [[Jumbling Decomposition]] `jumble → number, list`

## Sources

- 📖 **Guidebook:** [[Jumble Iota]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Jumbling Gambit](https://overevaluate.hexxy.media/v/latest/main/en_us/#jumble@overevaluate:create_jumble)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCreateJumble` is not a `SpellAction`
- 🔧 **Pattern ID:** `overevaluate:create_jumble`
