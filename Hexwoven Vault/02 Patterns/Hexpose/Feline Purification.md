---
aliases: ["Feline Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:cat_variant"
signature: "wqwqqwqwawaaw"
start_direction: "SOUTH_WEST"
input: "entity"
output: "identifier"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Identifier]]"]
entry: "[[Miscellaneous (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Feline Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Miscellaneous (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 94.4,145.5 111.8,115.3 129.3,85.0 111.8,54.7 94.4,24.5 76.9,54.7 59.4,85.0 94.4,85.0 76.9,54.7 59.4,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:489;--dur:3.9s" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 59.4,145.5 94.4,145.5 111.8,115.3 129.3,85.0 111.8,54.7 94.4,24.5 76.9,54.7 59.4,85.0 94.4,85.0 76.9,54.7 59.4,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="24.5" x2="53.8" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,41.1 50.5,32.2 57.2,36.1" fill="#fc77be"/><circle cx="59.4" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqqwqwawaaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `identifier` ([[Identifier]])
- **ID:** `hexpose:cat_variant`

```text
before   … │ entity      ◂ top
after    … │ identifier  ◂ top
```

## Description

Pushes the variant of the **Cat**.

## Related patterns (same book page)

- [[Anger Purification]] `entity → number`
- [[Showcase Purification]] `entity → number`
- [[Artistic Purification]] `entity → identifier`

## Sources

- 📖 **Guidebook:** [[Miscellaneous (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Feline Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/misc@hexpose:cat_variant)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `HexposeActions$init$75` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:cat_variant`
