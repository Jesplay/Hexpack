---
aliases: ["Inverse Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "MoreIotas"
op_id: "moreiotas:matrix/inverse"
signature: "wwdqdwwdqaq"
start_direction: "WEST"
input: "mat"
output: "mat"
cost: ""
iotas_in: ["[[Matrix]]"]
iotas_out: ["[[Matrix]]"]
entry: "[[Matrices]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Inverse Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Matrices]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="128.4,84.4 93.7,84.4 59.0,84.4 24.3,84.4 41.6,54.3 24.3,24.3 59.0,24.3 93.7,24.3 128.4,24.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:416;--dur:3.4s" points="128.4,84.4 93.7,84.4 59.0,84.4 24.3,84.4 41.6,54.3 24.3,24.3 59.0,24.3 93.7,24.3 128.4,24.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="84.4" x2="117.3" y2="84.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="109.3,84.4 117.3,80.6 117.3,88.2" fill="#fc77be"/><circle cx="128.4" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwdqdwwdqaq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mat` ([[Matrix]])
- **Leaves:** `mat` ([[Matrix]])
- **ID:** `moreiotas:matrix/inverse`

```text
before   … │ mat  ◂ top
after    … │ mat  ◂ top
```

## Description

Accepts a matrix of any shappe and returns its inverse. For square matrices with a nonzero determinant, the inverse is the matrix such that in\*out = identity.

## Related patterns (same book page)

- [[Transformation Purification]] `num | vec | list → mat`
- [[Restoration Purification]] `mat → num | vec | list`
- [[Restoration Purification II]] `mat → [[num]]`
- [[Identity Purification]] `int ≥ 0 → mat`
- [[Zero Distillation]] `int ≥ 0, int ≥ 0 → mat`
- [[Rotation Distillation]] `vec, num → mat`
- [[Multiplicative Distillation II]] `mat, mat → mat`
- [[Division Distillation II]] `mat, mat → mat`
- [[Power Distillation II]] `mat, mat → mat`
- [[Determinant Purification]] `mat → num`
- [[Tower Distillation]] `mat, mat → mat`
- [[Sprawling Distillation]] `mat, mat → mat`
- [[Toppling Gambit]] `mat, num → mat, mat`
- [[Mitosis Gambit]] `mat, num → mat, mat`

## Sources

- 📖 **Guidebook:** [[Matrices]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Inverse Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/matrices@moreiotas:matrix/inverse)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpInverseMatrix` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:matrix/inverse`
