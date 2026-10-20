---
aliases: ["Epigrapher's Reflection"]
tags: [hexcasting/pattern, mod/lapisworks, family/reflection, topic/items]
kind: "Pattern"
family: "Reflection"
topics: ["Items & Crafting"]
mod: "Lapisworks"
op_id: "lapisworks:writeable_necklace"
signature: "wwadeeeeeqw"
start_direction: "NORTH_WEST"
input: ""
output: "bool"
cost: ""
iotas_in: []
iotas_out: ["[[Boolean]]"]
entry: "[[Trinkets]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Epigrapher's Reflection

◇ **Pattern** · [[Lapisworks]] · 📖 [[Trinkets]] · Lapisworks › Imbuement

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="129.3,145.5 111.8,115.3 94.4,85.0 76.9,54.7 59.4,85.0 41.9,54.7 59.4,24.5 94.4,24.5 111.8,54.7 94.4,85.0 59.4,85.0 41.9,115.3 24.5,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:419;--dur:3.4s" points="129.3,145.5 111.8,115.3 94.4,85.0 76.9,54.7 59.4,85.0 41.9,54.7 59.4,24.5 94.4,24.5 111.8,54.7 94.4,85.0 59.4,85.0 41.9,115.3 24.5,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="129.3" y1="145.5" x2="123.7" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="119.7,128.9 127.1,133.9 120.4,137.8" fill="#fc77be"/><circle cx="129.3" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwadeeeeeqw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `lapisworks:writeable_necklace`

```text
before   …         ◂ top
after    … │ bool  ◂ top
```

## Description

Pushes if attempting to write to my necklace will succeed.

## Related patterns (same book page)

- [[Jeweller's Reflection]] `— → any`
- [[Jeweller's Gambit]] `any → —`
- [[Gemologist's Reflection]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Trinkets]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `WriteableNecklace` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:writeable_necklace`
