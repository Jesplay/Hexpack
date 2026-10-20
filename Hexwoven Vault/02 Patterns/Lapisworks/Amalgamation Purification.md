---
aliases: ["Amalgamation Purification"]
tags: [hexcasting/pattern, mod/lapisworks, family/purification, topic/minds, topic/enchanting]
kind: "Pattern"
family: "Purification"
topics: ["Minds & Hierophantics", "Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:get_amalgamation"
signature: "qaqqaqqa"
start_direction: "NORTH_EAST"
input: "vec"
output: "amalgam"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Amalgam]]"]
entry: "[[Amalgamations]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Amalgamation Purification

◇ **Pattern** · [[Lapisworks]] · 📖 [[Amalgamations]] · Lapisworks › Interop › Chariot

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]] · [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 60.0,78.3 35.0,121.6 85.0,121.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="85.0,121.6 110.0,78.3 85.0,35.0 60.0,78.3 85.0,121.6 135.0,121.6 110.0,78.3 60.0,78.3 35.0,121.6 85.0,121.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="121.6" x2="93.0" y2="107.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="98.8,97.8 97.8,110.5 88.2,105.0" fill="#fc77be"/><circle cx="85.0" cy="121.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqaqqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `amalgam` ([[Amalgam]])
- **ID:** `lapisworks:get_amalgamation`

```text
before   … │ vec      ◂ top
after    … │ amalgam  ◂ top
```

## Description

Returns the **Amalgamation** in the container at the specified position (within range). Any changes to it are reflected in the block it came from.

## Related patterns (same book page)

- [[The Mind's Decomp]] `amalgam → amalgam, bool`
- [[Loudener's Decomposition]] `amalgam → amalgam, int`
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
- 🏷️ **Type (Pattern):** from the mod code: its action class `GetAmalgamation` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:get_amalgamation`
