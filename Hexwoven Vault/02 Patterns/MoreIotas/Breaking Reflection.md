---
aliases: ["Breaking Reflection"]
tags: [hexcasting/pattern, mod/moreiotas, family/reflection, topic/text]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/newline"
signature: "waawaw"
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

# Breaking Reflection

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,68.3 74.1,68.3 117.7,68.3 95.9,30.5 74.1,68.3 52.3,106.0 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="30.5,68.3 74.1,68.3 117.7,68.3 95.9,30.5 74.1,68.3 52.3,106.0 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="68.3" x2="44.5" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,68.3 44.5,73.1 44.5,63.5" fill="#fc77be"/><circle cx="30.5" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waawaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `str` ([[String]])
- **ID:** `moreiotas:string/newline`

```text
before   …        ◂ top
after    … │ str  ◂ top
```

## Description

Add a string to the [[The Stack|stack]] representing a break between lines, useful for combining with other strings.

## Related patterns (same book page)

- [[Separation Distillation]] `str, str → [str]`
- [[Input Purification]] `str → num | null`
- [[Blank Reflection]] `— → str`
- [[Spacing Reflection]] `— → str`
- [[Comma Reflection]] `— → str`
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
- 🌐 **Official online docs:** [Breaking Reflection](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/newline)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `moreiotas:string/newline`
