---
aliases: ["Reading Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/text]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat"]
mod: "Hexpose"
op_id: "hexpose:create_text"
signature: "awaqeeeee"
start_direction: "SOUTH_WEST"
input: "any"
output: "text"
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: ["[[String]]"]
entry: "[[Text Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Reading Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Text Iota]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,103.4 29.8,140.2 72.3,140.2 114.8,140.2 93.5,103.4 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="51.0,103.4 29.8,140.2 72.3,140.2 114.8,140.2 93.5,103.4 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="103.4" x2="44.2" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,123.7 40.2,112.9 48.3,117.5" fill="#fc77be"/><circle cx="51.0" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awaqeeeee` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `text` ([[String]])
- **ID:** `hexpose:create_text`

```text
before   … │ any   ◂ top
after    … │ text  ◂ top
```

## Description

Takes an [[Iotas|iota]] and returns a [[Text Iota|text iota]] representing it. If taking a [[Text Iota|text iota]], strips all the styles of it.

## Related patterns (same book page)

- [[Glyph Purification]] `text → list`
- [[Collage Purification]] `list → text`
- [[Lumière Gambit]] `text, vector → text`
- [[Gothic Gambit]] `text, boolean → text`
- [[Manutius' Gambit]] `text, boolean → text`
- [[Notetaker's Gambit]] `text, boolean → text`
- [[Editor's Gambit]] `text, boolean → text`
- [[Censor's Gambit]] `text, boolean → text`

## Sources

- 📖 **Guidebook:** [[Text Iota]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCreateText` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:create_text`
