---
aliases: ["Inverse Tangent Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/math]
kind: "Pattern"
family: "Purification"
topics: ["Math & Vectors"]
mod: "Hex Casting"
op_id: "hexcasting:arctan"
signature: "eadeeeeew"
start_direction: "NORTH_EAST"
input: "num"
output: "num"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Number]]"]
entry: "[[Advanced Mathematics]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Inverse Tangent Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Advanced Mathematics]] · Patterns

**Topics:** [[Math & Vectors (Topic)|Math & Vectors]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 93.5,103.4 72.3,66.6 114.8,66.6 136.1,103.4 114.8,140.2 72.3,140.2 51.0,103.4 72.3,66.6 93.5,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="29.8,140.2 51.0,103.4 93.5,103.4 72.3,66.6 114.8,66.6 136.1,103.4 114.8,140.2 72.3,140.2 51.0,103.4 72.3,66.6 93.5,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eadeeeeew` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:arctan`

```text
before   … │ num  ◂ top
after    … │ num  ◂ top
```

## Description

Takes the inverse tangent of a value, yielding the angle whose tangent is that value. Related to the values of [[Arc's Reflection|π]] and [[Circle's Reflection|τ]].

## Related patterns (same book page)

- [[Sine Purification]] `num → num`
- [[Cosine Purification]] `num → num`
- [[Tangent Purification]] `num → num`
- [[Inverse Sine Purification]] `num → num`
- [[Inverse Cosine Purification]] `num → num`
- [[Inverse Tangent Distillation]] `num, num → num`
- [[Logarithmic Distillation]] `num, num → num`

## Sources

- 📖 **Guidebook:** [[Advanced Mathematics]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Inverse Tangent Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_math@hexcasting:arctan)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OperationAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:arctan`
