---
aliases: ["Transformation Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "MoreIotas"
op_id: "moreiotas:matrix/make"
signature: "awwaeawwaadwa"
start_direction: "SOUTH_WEST"
input: "num | vec | list"
output: "mat"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]"]
iotas_out: ["[[Matrix]]"]
entry: "[[Matrices]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Transformation Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Matrices]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 27.0,27.0 46.4,60.5 65.7,27.0 85.0,60.5 104.3,94.0 123.6,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="60.5" x2="40.2" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.7,78.9 36.5,69.1 43.9,73.3" fill="#fc77be"/><circle cx="46.4" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awwaeawwaadwa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num | vec | list` ([[Number]], [[Vector]], [[List]])
- **Leaves:** `mat` ([[Matrix]])
- **ID:** `moreiotas:matrix/make`

## Description

Converts the [[Iotas|iota]] on top of the [[The Stack|stack]] into a matrix. This iota can be a number, vector, or list.

If it is a number, it becomes a 1x1 matrix. If it's a vector, it becomes a 3x1 matrix. If it's a list, it must be a list of purely numbers, purely vectors, or purely lists of numbers (each of which must be the same length). If it's a list of numbers it'll be 1 by n, if it's a list of vectors it'll be 3 by n, and if it's a list of lists it'll be n by m, where n is the inner list length and m is the outer list length (each inner list becomes a column).

## Related patterns (same book page)

- [[Restoration Purification]] `mat → num | vec | list`
- [[Restoration Purification II]] `mat → [[num]]`
- [[Identity Purification]] `int ≥ 0 → mat`
- [[Zero Distillation]] `int ≥ 0, int ≥ 0 → mat`
- [[Rotation Distillation]] `vec, num → mat`
- [[Multiplicative Distillation II]] `mat, mat → mat`
- [[Division Distillation II]] `mat, mat → mat`
- [[Power Distillation II]] `mat, mat → mat`
- [[Inverse Purification]] `mat → mat`
- [[Determinant Purification]] `mat → num`
- [[Tower Distillation]] `mat, mat → mat`
- [[Sprawling Distillation]] `mat, mat → mat`
- [[Toppling Gambit]] `mat, num → mat, mat`
- [[Mitosis Gambit]] `mat, num → mat, mat`

## Sources

- 📖 **Guidebook:** [[Matrices]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Transformation Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/matrices@moreiotas:matrix/make)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMakeMatrix` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:matrix/make`
