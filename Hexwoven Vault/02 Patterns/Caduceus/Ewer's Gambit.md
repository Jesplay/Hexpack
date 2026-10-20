---
aliases: ["Ewer's Gambit"]
tags: [hexcasting/pattern, mod/caduceus, family/gambit, topic/control-flow]
kind: "Pattern"
family: "Gambit"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:write/mark/local"
signature: "dqddedwa"
start_direction: "WEST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Jump Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Ewer's Gambit

◇ **Pattern** · [[Caduceus]] · 📖 [[Jump Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,140.2 51.0,140.2 72.3,103.4 51.0,66.6 93.5,66.6 72.3,103.4 29.8,103.4 51.0,66.6 72.3,29.8 29.8,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="93.5,140.2 51.0,140.2 72.3,103.4 51.0,66.6 93.5,66.6 72.3,103.4 29.8,103.4 51.0,66.6 72.3,29.8 29.8,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="140.2" x2="79.9" y2="140.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,140.2 79.9,135.6 79.9,144.9" fill="#fc77be"/><circle cx="93.5" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqddedwa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `caduceus:write/mark/local`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Remove the top [[Iotas|iota]] from the [[The Stack|stack]], and write it to the **jump tag** of the current evaluation.
As with [[Chronicler's Gambit]], I cannot write my own Name using this spell.

I should be careful if using [[Ewer's Reflection]] or [[Ewer's Gambit]] at the very end of an invocation. The tag that it reads or writes may not be the one I expect.

## Related patterns (same book page)

- [[Ewer's Reflection]] `— → any`
- [[Ewer's Purification]] `jump → any`

## Sources

- 📖 **Guidebook:** [[Jump Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Ewer's Gambit](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/jump_manipulation@caduceus:write/mark/local)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `caduceus:write/mark/local`
