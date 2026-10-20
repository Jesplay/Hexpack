---
aliases: ["Uniqueness Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/lists]
kind: "Pattern"
family: "Purification"
topics: ["Lists"]
mod: "Hex Casting"
op_id: "hexcasting:unique"
signature: "aweaqa"
start_direction: "NORTH_EAST"
input: "list"
output: "list"
cost: ""
iotas_in: ["[[List]]"]
iotas_out: ["[[List]]"]
entry: "[[Sets]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "page"
in_book: true
---

# Uniqueness Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Sets]] · Patterns

**Topics:** [[Lists (Topic)|Lists]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:2.0s" points="123.6,94.0 143.0,60.5 104.3,60.5 65.7,60.5 46.4,27.0 27.0,60.5 46.4,94.0 65.7,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="94.0" x2="129.8" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="134.3,75.6 133.5,85.4 126.1,81.1" fill="#fc77be"/><circle cx="123.6" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aweaqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list` ([[List]])
- **Leaves:** `list` ([[List]])
- **ID:** `hexcasting:unique`

```text
before   … │ list  ◂ top
after    … │ list  ◂ top
```

## Description

Removes duplicate entries from a list.

## Other uses (same pattern, other inputs)

### [[Hexpose]] · 📖 [[Text Iota]]

`text` → `text`

Keeps only the first instance of any glyph in the text. Styling is enough to distinguish a glyph.

## Sources

- 📖 **Guidebook:** [[Sets]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Uniqueness Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/sets@hexcasting:unique)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:unique`
