---
aliases: ["Epiphany Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:am_enlightened"
signature: "awqaqqq"
start_direction: "SOUTH_EAST"
input: "player entity"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Enlightened Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Epiphany Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Enlightened Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,103.4 51.0,140.2 72.3,103.4 93.5,66.6 72.3,29.8 51.0,66.6 72.3,103.4 114.8,103.4 136.1,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:340;--dur:2.2s" points="29.8,103.4 51.0,140.2 72.3,103.4 93.5,66.6 72.3,29.8 51.0,66.6 72.3,103.4 114.8,103.4 136.1,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="103.4" x2="36.6" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,123.7 32.5,117.5 40.6,112.9" fill="#fc77be"/><circle cx="29.8" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awqaqqq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player entity` ([[Entity]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexpose:am_enlightened`

```text
before   … │ player entity  ◂ top
after    … │ boolean        ◂ top
```

## Description

Pushes whether the given player has achieved **Enlightenment**.

## Related patterns (same book page)

- [[Sentience Purification]] `entity → boolean`

## Sources

- 📖 **Guidebook:** [[Enlightened Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetPlayerData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:am_enlightened`
