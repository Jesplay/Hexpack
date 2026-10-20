---
aliases: ["Ewer's Purification"]
tags: [hexcasting/pattern, mod/caduceus, family/purification, topic/control-flow]
kind: "Pattern"
family: "Purification"
topics: ["Control Flow & Meta-evaluation"]
mod: "Caduceus"
op_id: "caduceus:read/mark/iota"
signature: "adaaddad"
start_direction: "EAST"
input: "jump"
output: "any"
cost: ""
iotas_in: ["[[Jump (Continuation)]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Jump Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Ewer's Purification

◇ **Pattern** · [[Caduceus]] · 📖 [[Jump Manipulation]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 72.3,140.2 51.0,103.4 93.5,103.4 72.3,66.6 51.0,103.4 29.8,66.6 72.3,66.6 51.0,29.8 93.5,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="29.8,140.2 72.3,140.2 51.0,103.4 93.5,103.4 72.3,66.6 51.0,103.4 29.8,66.6 72.3,66.6 51.0,29.8 93.5,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="43.4" y2="140.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="53.2,140.2 43.4,144.9 43.4,135.6" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `adaaddad` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `jump` ([[Jump (Continuation)]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `caduceus:read/mark/iota`

```text
before   … │ jump  ◂ top
after    … │ any   ◂ top
```

## Description

Like [[Ewer's Reflection]], but the [[Iotas|iota]] is read out of the **jump tag** of the top frame in a Jump iota.

## Related patterns (same book page)

- [[Ewer's Reflection]] `— → any`
- [[Ewer's Gambit]] `any → —`

## Sources

- 📖 **Guidebook:** [[Jump Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Ewer's Purification](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/jump_manipulation@caduceus:read/mark/iota)
- 🧩 **Mod page:** [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `caduceus:read/mark/iota`
