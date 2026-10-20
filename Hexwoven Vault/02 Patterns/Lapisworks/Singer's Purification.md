---
aliases: ["Singer's Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/minds]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics"]
mod: "Lapisworks"
op_id: "lapisworks:song_purification"
signature: "aqawwd"
start_direction: "WEST"
input: "vec"
output: "([0-24], 0-20)"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[List]]"]
entry: "[[Imbue Mind (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Singer's Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Imbue Mind (Book Entry)]] · Lapisworks › The Mage's Spells

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 29.8,103.4 51.0,140.2 93.5,140.2 72.3,103.4 51.0,66.6 29.8,29.8 72.3,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="72.3,103.4 29.8,103.4 51.0,140.2 93.5,140.2 72.3,103.4 51.0,66.6 29.8,29.8 72.3,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="58.7" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.9,103.4 58.7,98.7 58.7,108.1" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawwd` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `([0-24], 0-20)` ([[List]])
- **ID:** `lapisworks:song_purification`

```text
before   … │ vec             ◂ top
after    … │ ([0-24], 0-20)  ◂ top
```

## Description

Divines from the specified **Live Jukebox** (within ambit) it's properties, and then returns a list of two elements. The first being the notes it plays, and the second being the frequency it plays them at. Free.

## Related patterns (same book page)

- [[Imbue Mind]] `vec, vec → —`
- [[Teach Song]] `vec, [0-24], 0-20 → —`
- [[Teach Simple Impetus]] `vec, pattern → —`
- [[Interviewer's Prfn]] `vec → pattern|null`

## Sources

- 📖 **Guidebook:** [[Imbue Mind (Book Entry)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `SongPrfn` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:song_purification`
