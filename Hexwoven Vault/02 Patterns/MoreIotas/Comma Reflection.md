---
aliases: ["Comma Reflection"]
tags: [hexcasting/pattern, mod/moreiotas, family/reflection, topic/text]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/comma"
signature: "qa"
start_direction: "EAST"
input: ""
output: "str"
cost: ""
iotas_in: []
iotas_out: ["[[String]]"]
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Comma Reflection

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="133" viewBox="0 0 170.0 132.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="41.0" r="2"/><circle cx="70.3" cy="41.0" r="2"/><circle cx="129.0" cy="41.0" r="2"/><circle cx="41.0" cy="91.8" r="2"/><circle cx="99.7" cy="91.8" r="2"/><circle cx="158.3" cy="91.8" r="2"/></g><polyline class="hexpat-base" points="41.0,91.8 99.7,91.8 129.0,41.0 70.3,41.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:176;--dur:1.6s" points="41.0,91.8 99.7,91.8 129.0,41.0 70.3,41.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.0" y1="91.8" x2="59.8" y2="91.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.3,91.8 59.8,98.2 59.8,85.4" fill="#fc77be"/><circle cx="41.0" cy="91.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `str` ([[String]])
- **ID:** `moreiotas:string/comma`

```text
before   …        ◂ top
after    … │ str  ◂ top
```

## Description

Add a comma string to the [[The Stack|stack]], useful for combining with other strings.

## Related patterns (same book page)

- [[Separation Distillation]] `str, str → [str]`
- [[Input Purification]] `str → num | null`
- [[Blank Reflection]] `— → str`
- [[Spacing Reflection]] `— → str`
- [[Breaking Reflection]] `— → str`
- [[Whisper Reflection]] `— → str`
- [[Listener's Reflection]] `— → str`
- [[Sifter's Gambit]] `str | null → —`
- [[Sifter's Reflection]] `— → str | null`
- [[Reader's Purification]] `vec → str | [str]`
- [[Write]] `vec, str | [str] → —`
- [[Scrivener's Purification]] `any → str`
- [[Patternmaster's Purification]] `pattern → str | null`
- [[Moniker Purification]] `entity → str`
- [[Name]] `str, entity → —`
- [[Case Distillation]] `str, bool | null → str`

## Sources

- 📖 **Guidebook:** [[Strings]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Comma Reflection](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/comma)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `moreiotas:string/comma`
