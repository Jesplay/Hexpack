---
aliases: ["Modicum Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hexpose"
op_id: "hexpose:classify"
signature: "edqdeq"
start_direction: "WEST"
input: "any"
output: "identifier"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[Identifier]]"]
entry: "[[Identifiers]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Modicum Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Identifiers]] · Patterns › Scrying

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,103.4 51.0,103.4 29.8,66.6 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="93.5,103.4 51.0,103.4 29.8,66.6 72.3,66.6 93.5,29.8 114.8,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="103.4" x2="79.9" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,103.4 79.9,98.7 79.9,108.1" fill="#fc77be"/><circle cx="93.5" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edqdeq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `identifier` ([[Identifier]])
- **ID:** `hexpose:classify`

```text
before   … │ any         ◂ top
after    … │ identifier  ◂ top
```

## Description

Pushes the identifier of an [[Iotas|iota]]'s type. Can be used to determine whether two iota are the same type.

## Related patterns (same book page)

- [[Detective's Purification]] `identifiable → identifier`

## Sources

- 📖 **Guidebook:** [[Identifiers]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpClassify` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:classify`
