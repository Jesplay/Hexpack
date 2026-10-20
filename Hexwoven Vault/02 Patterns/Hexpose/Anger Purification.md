---
aliases: ["Anger Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:creeper_fuse"
signature: "dedwaqwede"
start_direction: "WEST"
input: "entity"
output: "number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Miscellaneous (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Anger Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Miscellaneous (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="59.0,54.3 24.3,54.3 41.6,24.3 76.3,24.3 59.0,54.3 41.6,84.4 76.3,84.4 93.7,54.3 111.0,24.3 145.7,24.3 128.4,54.3 93.7,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:382;--dur:3.1s" points="59.0,54.3 24.3,54.3 41.6,24.3 76.3,24.3 59.0,54.3 41.6,84.4 76.3,84.4 93.7,54.3 111.0,24.3 145.7,24.3 128.4,54.3 93.7,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.0" y1="54.3" x2="47.9" y2="54.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.9,54.3 47.9,50.5 47.9,58.1" fill="#fc77be"/><circle cx="59.0" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedwaqwede` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:creeper_fuse`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the progress of a **Creeper** to explosion.

## Related patterns (same book page)

- [[Feline Purification]] `entity → identifier`
- [[Showcase Purification]] `entity → number`
- [[Artistic Purification]] `entity → identifier`

## Sources

- 📖 **Guidebook:** [[Miscellaneous (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Anger Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/misc@hexpose:creeper_fuse)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `HexposeActions$init$76` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:creeper_fuse`
