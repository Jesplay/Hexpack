---
aliases: ["Radian Purification"]
tags: [hexcasting/pattern, mod/hextra-patterns, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hextra Patterns"
op_id: "hextrapats:deg_to_rad"
signature: "qqqqqdwdq"
start_direction: "WEST"
input: "num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Hextra Math]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Radian Purification

◇ **Pattern** · [[Hextra Patterns]] · 📖 [[Hextra Math]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="76.3,24.3 41.6,24.3 24.3,54.3 41.6,84.4 76.3,84.4 93.7,54.3 76.3,24.3 111.0,24.3 145.7,24.3 128.4,54.3 145.7,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="76.3,24.3 41.6,24.3 24.3,54.3 41.6,84.4 76.3,84.4 93.7,54.3 76.3,24.3 111.0,24.3 145.7,24.3 128.4,54.3 145.7,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="24.3" x2="65.2" y2="24.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.2,24.3 65.2,20.5 65.2,28.1" fill="#fc77be"/><circle cx="76.3" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqdwdq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hextrapats:deg_to_rad`

```text
before   … │ num  ◂ top
after    … │ num  ◂ top
```

## Description

Converts an angle in degrees to an angle in radians. Essentially multiplies by [[Arc's Reflection|pi]] then divides by 180.

## Related patterns (same book page)

- [[Degree Purification]] `num → num`
- [[Reflection Purification]] `num/vec → num/vec`
- [[Incrementation Purification]] `num → num`
- [[Decrementation Purification]] `num → num`
- [[Rounding Purification]] `num/vec → num/vec`
- [[Rounding Distillation]] `num/vec, num → num/vec`
- [[Entropy Purification]] `num → num`
- [[Entropy Distillation]] `num, num → num`
- [[Scientific Purification]] `num/vec → num/vec`
- [[Approach Exaltation]] `num, num, num → num`
- [[Theta Distillation]] `num, num → num`
- [[Turning Exaltation]] `num, num, num → num`

## Sources

- 📖 **Guidebook:** [[Hextra Math]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDegRad` is not a `SpellAction`
- 🔧 **Pattern ID:** `hextrapats:deg_to_rad`
