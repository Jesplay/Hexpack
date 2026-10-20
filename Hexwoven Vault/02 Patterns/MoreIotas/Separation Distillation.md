---
aliases: ["Separation Distillation"]
tags: [hexcasting/pattern, mod/moreiotas, family/distillation, topic/text]
kind: "Pattern"
family: "Distillation"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/split"
signature: "aqwaqa"
start_direction: "EAST"
input: "str, str"
output: "[str]"
cost: ""
iotas_in: ["[[String]]"]
iotas_out: ["[[List]]", "[[String]]"]
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Separation Distillation

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/></g><polyline class="hexpat-base" points="95.9,68.3 139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 95.9,68.3 74.1,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="95.9,68.3 139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 95.9,68.3 74.1,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="68.3" x2="109.8" y2="68.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="119.9,68.3 109.8,73.1 109.8,63.5" fill="#fc77be"/><circle cx="95.9" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqwaqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `str, str` ([[String]])
- **Leaves:** `[str]` ([[List]], [[String]])
- **ID:** `moreiotas:string/split`

```text
before   … │ str │ str  ◂ top
after    … │ [str]      ◂ top
```

## Description

Removes the top two strings from the [[The Stack|stack]] and splits the first string, using the second string as the delimiter and returning as a list.

## Related patterns (same book page)

- [[Input Purification]] `str → num | null`
- [[Blank Reflection]] `— → str`
- [[Spacing Reflection]] `— → str`
- [[Comma Reflection]] `— → str`
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
- 🌐 **Official online docs:** [Separation Distillation](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/split)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSplitString` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:string/split`
