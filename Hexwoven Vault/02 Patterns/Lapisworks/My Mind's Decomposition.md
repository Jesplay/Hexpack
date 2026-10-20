---
aliases: ["My Mind's Decomposition"]
tags: [hexcasting/pattern, mod/lapisworks, family/decomposition, topic/minds, topic/enchanting]
kind: "Pattern"
family: "Decomposition"
topics: ["Minds & Hierophantics", "Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:get_self_amalgams_num"
signature: "qwedewqqaqqa"
start_direction: "NORTH_EAST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Amalgamations]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# My Mind's Decomposition

◇ **Pattern** · [[Lapisworks]] · 📖 [[Amalgamations]] · Lapisworks › Interop › Chariot

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,145.5 76.9,115.3 59.4,85.0 41.9,54.7 59.4,24.5 76.9,54.7 59.4,85.0 41.9,115.3 59.4,145.5 94.4,145.5 76.9,115.3 41.9,115.3 24.5,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:454;--dur:3.6s" points="59.4,145.5 76.9,115.3 59.4,85.0 41.9,54.7 59.4,24.5 76.9,54.7 59.4,85.0 41.9,115.3 59.4,145.5 94.4,145.5 76.9,115.3 41.9,115.3 24.5,145.5 59.4,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="145.5" x2="65.0" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="69.0,128.9 68.3,137.8 61.7,133.9" fill="#fc77be"/><circle cx="59.4" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwedewqqaqqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `lapisworks:get_self_amalgams_num`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Pushes the number of amalgamations on my person.

## Related patterns (same book page)

- [[Amalgamation Purification]] `vec → amalgam`
- [[The Mind's Decomp]] `amalgam → amalgam, bool`
- [[Loudener's Decomposition]] `amalgam → amalgam, int`
- [[Loudener's Distillation]] `amalgam, num → amalgam`
- [[Fisherman's Decomposition]] `amalgam → amalgam, num`
- [[Fisherman's Distillation]] `amalgam, num → amalgam`
- [[Amalgamation Gambit]] `num → —`
- [[Mistake Decomposition]] `amalgam → amalgam, num`
- [[Etched Decomposition]] `amalgam → amalgam, [pattern]`
- [[Etched Distillation]] `amalgam, [pattern] → amalgam`

## Sources

- 📖 **Guidebook:** [[Amalgamations]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetSelfAmalgamsNum` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_self_amalgams_num`
