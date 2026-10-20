---
aliases: ["Aspect Distillation"]
tags: [hexcasting/pattern, mod/hexwright, family/distillation, topic/hexwright, topic/items]
kind: "Pattern"
family: "Distillation"
topics: ["Hex Wright Magic", "Items & Crafting"]
mod: "Hexwright"
op_id: "hexwright:aspect_distillation"
signature: "ewedadadeade"
start_direction: "WEST"
input: "vec, num"
output: "num"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: ["[[Number]]"]
entry: "[[Endless Pouch]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Aspect Distillation

◇ **Pattern** · [[Hexwright]] · 📖 [[Endless Pouch]] · Hex Wright › Fire & Essence

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="93.7,114.4 59.0,114.4 41.6,84.4 24.3,54.3 41.6,24.3 59.0,54.3 76.3,24.3 93.7,54.3 111.0,24.3 128.4,54.3 111.0,84.4 145.7,84.4 128.4,114.4 93.7,114.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:451;--dur:3.6s" points="93.7,114.4 59.0,114.4 41.6,84.4 24.3,54.3 41.6,24.3 59.0,54.3 76.3,24.3 93.7,54.3 111.0,24.3 128.4,54.3 111.0,84.4 145.7,84.4 128.4,114.4 93.7,114.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.7" y1="114.4" x2="82.6" y2="114.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.6,114.4 82.6,110.6 82.6,118.2" fill="#fc77be"/><circle cx="93.7" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewedadadeade` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, num` ([[Number]], [[Vector]])
- **Leaves:** `num` ([[Number]])
- **ID:** `hexwright:aspect_distillation`

```text
before   … │ vec │ num  ◂ top
after    … │ num        ◂ top
```

## Description

Returns the amount of Essence in the pouch at vec location. Num is the aspect's place in pouch order from [[Essence Purification]].

## Related patterns (same book page)

- [[Essence Purification]] `vec → [string | null]`

## Sources

- 📖 **Guidebook:** [[Endless Pouch]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `HexwrightActions$2` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:aspect_distillation`
