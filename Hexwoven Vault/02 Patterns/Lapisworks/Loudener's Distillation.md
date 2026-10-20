---
aliases: ["Loudener's Distillation"]
tags: [hexcasting/pattern, mod/lapisworks, family/distillation, topic/minds, topic/enchanting]
kind: "Pattern"
family: "Distillation"
topics: ["Minds & Hierophantics", "Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:set_amalgam_notiflevel"
signature: "wdeddedweeweewe"
start_direction: "NORTH_EAST"
input: "amalgam, num"
output: "amalgam"
cost: ""
iotas_in: ["[[Number]]", "[[Amalgam]]"]
iotas_out: ["[[Amalgam]]"]
entry: "[[Amalgamations]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Loudener's Distillation

◇ **Pattern** · [[Lapisworks]] · 📖 [[Amalgamations]] · Lapisworks › Interop › Chariot

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="46.4,127.4 65.7,94.0 85.0,60.5 104.3,94.0 85.0,127.4 65.7,94.0 104.3,94.0 123.6,127.4 85.0,127.4 46.4,127.4 27.0,94.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 123.6,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:618;--dur:4.5s" points="46.4,127.4 65.7,94.0 85.0,60.5 104.3,94.0 85.0,127.4 65.7,94.0 104.3,94.0 123.6,127.4 85.0,127.4 46.4,127.4 27.0,94.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 123.6,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="127.4" x2="52.5" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="57.0,109.0 56.2,118.8 48.9,114.6" fill="#fc77be"/><circle cx="46.4" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdeddedweeweewe` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `amalgam, num` ([[Number]], [[Amalgam]])
- **Leaves:** `amalgam` ([[Amalgam]])
- **ID:** `lapisworks:set_amalgam_notiflevel`

```text
before   … │ amalgam │ num  ◂ top
after    … │ amalgam        ◂ top
```

## Description

Sets the provided amalgamation's loudness to the specified number (which must be positive and below 4).

## Related patterns (same book page)

- [[Amalgamation Purification]] `vec → amalgam`
- [[The Mind's Decomp]] `amalgam → amalgam, bool`
- [[Loudener's Decomposition]] `amalgam → amalgam, int`
- [[Fisherman's Decomposition]] `amalgam → amalgam, num`
- [[Fisherman's Distillation]] `amalgam, num → amalgam`
- [[My Mind's Decomposition]] `— → num`
- [[Amalgamation Gambit]] `num → —`
- [[Mistake Decomposition]] `amalgam → amalgam, num`
- [[Etched Decomposition]] `amalgam → amalgam, [pattern]`
- [[Etched Distillation]] `amalgam, [pattern] → amalgam`

## Sources

- 📖 **Guidebook:** [[Amalgamations]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `SetAmalgamAlert` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:set_amalgam_notiflevel`
