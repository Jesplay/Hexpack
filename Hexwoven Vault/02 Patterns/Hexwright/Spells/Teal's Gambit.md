---
aliases: ["Teal's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:noneuclidean_window"
signature: "qqwqqqwaqq"
start_direction: "SOUTH_WEST"
input: "[vec, vec], [vec, vec], num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]"]
iotas_out: []
entry: "[[Shaped World Crystal]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Teal's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Shaped World Crystal]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 65.7,60.5 104.3,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 46.4,27.0 65.7,60.5 104.3,60.5 123.6,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="27.0" x2="40.2" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,45.4 36.5,35.6 43.9,39.9" fill="#fc77be"/><circle cx="46.4" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqwqqqwaqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[vec, vec], [vec, vec], num` ([[Number]], [[Vector]], [[List]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:noneuclidean_window`

```text
before   … │ [vec, vec] │ [vec, vec] │ num  ◂ top
after    …                                  ◂ top
```

## Description

Takes two sets of vectors forming two planes, folding them to occupy the same space time for the

amount of time specified in num seconds. This gives the effect of a window which can be stepped through in either direction.
It doesn't come without limits though as each plane must match in size and must exist (be loaded) in the world. It will [[Mishaps|mishap]] if not.

## Sources

- 📖 **Guidebook:** [[Shaped World Crystal]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `PortalActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:noneuclidean_window`
