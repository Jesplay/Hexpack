---
aliases: ["Loudener's Decomposition"]
tags: [hexcasting/pattern, mod/lapisworks, family/decomposition, topic/minds, topic/enchanting]
kind: "Pattern"
family: "Decomposition"
topics: ["Minds & Hierophantics", "Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:get_amalgam_notiflevel"
signature: "waqaaqawqqwqqwq"
start_direction: "NORTH_WEST"
input: "amalgam"
output: "amalgam, int"
cost: ""
iotas_in: ["[[Amalgam]]"]
iotas_out: ["[[Number]]", "[[Amalgam]]"]
entry: "[[Amalgamations]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Loudener's Decomposition

◇ **Pattern** · [[Lapisworks]] · 📖 [[Amalgamations]] · Lapisworks › Interop › Chariot

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Decomposition
> This pattern takes **one** iota and gives back **two**: its name marks it as a **Decomposition**. See [[Pattern Naming Guide]] · [[Patterns by Family#Decomposition|all Decomposition patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="123.6,127.4 104.3,94.0 85.0,60.5 65.7,94.0 85.0,127.4 104.3,94.0 65.7,94.0 46.4,127.4 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:618;--dur:4.5s" points="123.6,127.4 104.3,94.0 85.0,60.5 65.7,94.0 85.0,127.4 104.3,94.0 65.7,94.0 46.4,127.4 85.0,127.4 123.6,127.4 143.0,94.0 123.6,60.5 104.3,27.0 65.7,27.0 46.4,60.5 27.0,94.0 46.4,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="127.4" x2="117.5" y2="116.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="113.0,109.0 121.1,114.6 113.8,118.8" fill="#fc77be"/><circle cx="123.6" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqaaqawqqwqqwq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `amalgam` ([[Amalgam]])
- **Leaves:** `amalgam, int` ([[Number]], [[Amalgam]])
- **ID:** `lapisworks:get_amalgam_notiflevel`

```text
before   … │ amalgam        ◂ top
after    … │ amalgam │ int  ◂ top
```

## Description

Returns the **Amalgamation** in the container at the specified position (within range). Any changes to it are reflected in the block it came from.

## Related patterns (same book page)

- [[Amalgamation Purification]] `vec → amalgam`
- [[The Mind's Decomp]] `amalgam → amalgam, bool`
- [[Loudener's Distillation]] `amalgam, num → amalgam`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetAmalgamAlert` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_amalgam_notiflevel`
