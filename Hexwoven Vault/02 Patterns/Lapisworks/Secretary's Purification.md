---
aliases: ["Secretary's Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/storage]
kind: "Pattern"
family: "Purification"
topics: ["Reading & Writing"]
mod: "Lapisworks"
op_id: "lapisworks:read_spechand"
signature: "aqqqqa"
start_direction: "EAST"
input: "num"
output: "any"
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Specific Read-write]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Secretary's Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Specific Read-write]] · Lapisworks

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="85.0,78.3 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 85.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqqqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `lapisworks:read_spechand`

```text
before   … │ num  ◂ top
after    … │ any  ◂ top
```

## Description

Specific hand version of [[Scribe's Reflection]]. Pushes the [[Iotas|Iota]] stored in the Iota holder in the specified hand to the [[The Stack|stack]].

## Related patterns (same book page)

- [[Reviewer's Purification]] `num → bool`
- [[Amanuensis' Gambit]] `any, num → —`
- [[Sanctioner's Purification]] `num → bool`

## Sources

- 📖 **Guidebook:** [[Specific Read-write]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `ReadFromHand` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:read_spechand`
