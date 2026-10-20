---
aliases: ["Collage Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/text]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat"]
mod: "Hexpose"
op_id: "hexpose:combine_text"
signature: "dwdeqqqqqawq"
start_direction: "SOUTH_EAST"
input: "list"
output: "text"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: ["[[String]]"]
entry: "[[Text Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Collage Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Text Iota]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,115.3 94.4,145.5 59.4,145.5 24.5,145.5 41.9,115.3 76.9,115.3 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 59.4,85.0 76.9,54.7 59.4,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:454;--dur:3.6s" points="76.9,115.3 94.4,145.5 59.4,145.5 24.5,145.5 41.9,115.3 76.9,115.3 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 59.4,85.0 76.9,54.7 59.4,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="115.3" x2="82.5" y2="125.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.5,131.9 79.2,126.9 85.8,123.0" fill="#fc77be"/><circle cx="76.9" cy="115.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwdeqqqqqawq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `text` ([[String]])
- **ID:** `hexpose:combine_text`

```text
before   … │ list  ◂ top
after    … │ text  ◂ top
```

## Description

Transforms all the elements of a list into text (or keeps them if they were) and pastes it all together.

## Related patterns (same book page)

- [[Reading Purification (Hexpose)]] `any → text`
- [[Glyph Purification]] `text → list`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCombineList` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:combine_text`
