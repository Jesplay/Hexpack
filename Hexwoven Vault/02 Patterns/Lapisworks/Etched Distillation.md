---
aliases: ["Etched Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/minds, topic/enchanting]
kind: "Pattern"
family: "Distillation"
topics: ["Minds & Hierophantics", "Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:set_amalgam_hex"
signature: "edeedeedadeaqq"
start_direction: "NORTH_WEST"
input: "amalgam, [pattern]"
output: "amalgam"
cost: ""
iotas_in: ["[[List]]", "[[Pattern]]", "[[Amalgam]]"]
iotas_out: ["[[Amalgam]]"]
entry: "[[Amalgamations]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Etched Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Amalgamations]] · Lapisworks › Interop › Chariot

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="129.3" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="129.3" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0 41.9,85.0 59.4,54.7 94.4,54.7 111.8,85.0 76.9,85.0 94.4,115.3 59.4,115.3 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:524;--dur:4.2s" points="76.9,85.0 59.4,54.7 76.9,24.5 94.4,54.7 76.9,85.0 41.9,85.0 59.4,54.7 94.4,54.7 111.8,85.0 76.9,85.0 94.4,115.3 59.4,115.3 41.9,85.0 24.5,115.3 41.9,145.5 76.9,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="85.0" x2="71.3" y2="75.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,68.4 74.6,73.4 68.0,77.2" fill="#fc77be"/><circle cx="76.9" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeedeedadeaqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `amalgam, [pattern]` ([[List]], [[Pattern]], [[Amalgam]])
- **Leaves:** `amalgam` ([[Amalgam]])
- **ID:** `lapisworks:set_amalgam_hex`

```text
before   … │ amalgam │ [pattern]  ◂ top
after    … │ amalgam              ◂ top
```

## Description

Sets the hex that the provided amalgamation would execute upon a successful trigger.

## Related patterns (same book page)

- [[Amalgamation Purification]] `vec → amalgam`
- [[The Mind's Decomp]] `amalgam → amalgam, bool`
- [[Loudener's Decomposition]] `amalgam → amalgam, int`
- [[Loudener's Distillation]] `amalgam, num → amalgam`
- [[Fisherman's Decomposition]] `amalgam → amalgam, num`
- [[Fisherman's Distillation]] `amalgam, num → amalgam`
- [[My Mind's Decomposition]] `— → num`
- [[Amalgamation Gambit]] `num → —`
- [[Mistake Decomposition]] `amalgam → amalgam, num`
- [[Etched Decomposition]] `amalgam → amalgam, [pattern]`

## Sources

- 📖 **Guidebook:** [[Amalgamations]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `SetAmalgamHex` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:set_amalgam_hex`
