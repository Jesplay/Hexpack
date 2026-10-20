---
aliases: ["Media Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:get_media"
signature: "dde"
start_direction: "WEST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Basic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Media Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Basic Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,85.0 38.0,85.0 65.1,38.0 92.3,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="92.3,85.0 38.0,85.0 65.1,38.0 92.3,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="85.0" x2="74.9" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.4,85.0 74.9,79.0 74.9,91.0" fill="#fc77be"/><circle cx="92.3" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dde` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hexcasting:get_media`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Pushes the amount of **[[Media|media]]** available to the current cast, in units of [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Mind's Reflection]] `— → entity | null`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Alidade's Purification]] `entity → vector`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Reveal]] `any → any`
- [[Stadiometer's Purification]] `entity → num`
- [[Pace Purification]] `entity → vector`

## Sources

- 📖 **Guidebook:** [[Basic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Media Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics@hexcasting:get_media)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMedia` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:get_media`
