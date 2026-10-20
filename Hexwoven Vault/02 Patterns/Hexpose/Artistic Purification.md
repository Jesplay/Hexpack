---
aliases: ["Artistic Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:painting_variant"
signature: "wawwwqwwawwwqadaqeda"
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

# Artistic Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Miscellaneous (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.9" cy="17.2" r="2"/><circle cx="29.6" cy="17.2" r="2"/><circle cx="54.2" cy="17.2" r="2"/><circle cx="78.8" cy="17.2" r="2"/><circle cx="103.5" cy="17.2" r="2"/><circle cx="128.1" cy="17.2" r="2"/><circle cx="152.8" cy="17.2" r="2"/><circle cx="17.2" cy="38.6" r="2"/><circle cx="41.9" cy="38.6" r="2"/><circle cx="66.5" cy="38.6" r="2"/><circle cx="91.2" cy="38.6" r="2"/><circle cx="115.8" cy="38.6" r="2"/><circle cx="140.4" cy="38.6" r="2"/><circle cx="165.1" cy="38.6" r="2"/><circle cx="4.9" cy="59.9" r="2"/><circle cx="29.6" cy="59.9" r="2"/><circle cx="54.2" cy="59.9" r="2"/><circle cx="78.8" cy="59.9" r="2"/><circle cx="103.5" cy="59.9" r="2"/><circle cx="128.1" cy="59.9" r="2"/><circle cx="152.8" cy="59.9" r="2"/><circle cx="17.2" cy="81.3" r="2"/><circle cx="41.9" cy="81.3" r="2"/><circle cx="66.5" cy="81.3" r="2"/><circle cx="91.2" cy="81.3" r="2"/><circle cx="115.8" cy="81.3" r="2"/><circle cx="140.4" cy="81.3" r="2"/><circle cx="165.1" cy="81.3" r="2"/></g><polyline class="hexpat-base" points="41.9,38.6 29.6,59.9 17.2,81.3 41.9,81.3 66.5,81.3 91.2,81.3 115.8,81.3 128.1,59.9 140.4,38.6 152.8,17.2 128.1,17.2 103.5,17.2 78.8,17.2 54.2,17.2 41.9,38.6 66.5,38.6 54.2,59.9 78.8,59.9 91.2,38.6 115.8,38.6 103.5,59.9 128.1,59.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:517;--dur:5.9s" points="41.9,38.6 29.6,59.9 17.2,81.3 41.9,81.3 66.5,81.3 91.2,81.3 115.8,81.3 128.1,59.9 140.4,38.6 152.8,17.2 128.1,17.2 103.5,17.2 78.8,17.2 54.2,17.2 41.9,38.6 66.5,38.6 54.2,59.9 78.8,59.9 91.2,38.6 115.8,38.6 103.5,59.9 128.1,59.9" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.9" y1="38.6" x2="37.9" y2="45.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="35.1,50.3 35.6,44.1 40.3,46.8" fill="#fc77be"/><circle cx="41.9" cy="38.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawwwqwwawwwqadaqeda` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `identifier` ([[Identifier]])
- **ID:** `hexpose:painting_variant`

```text
before   … │ entity      ◂ top
after    … │ identifier  ◂ top
```

## Description

Pushes the variant of the **Painting**.

## Related patterns (same book page)

- [[Feline Purification]] `entity → identifier`
- [[Anger Purification]] `entity → number`
- [[Showcase Purification]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Miscellaneous (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Artistic Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/misc@hexpose:painting_variant)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `HexposeActions$init$78` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:painting_variant`
