---
aliases: ["Case Distillation"]
tags: [hexcasting/pattern, mod/moreiotas, family/distillation, topic/text]
kind: "Pattern"
family: "Distillation"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/case"
signature: "dwwdwwdwdd"
start_direction: "WEST"
input: "str, bool | null"
output: "str"
cost: ""
iotas_in: ["[[Boolean]]", "[[Null]]", "[[String]]"]
iotas_out: ["[[String]]"]
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Case Distillation

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="65.7,127.4 27.0,127.4 46.4,94.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,94.0 143.0,127.4 104.3,127.4 65.7,127.4 85.0,94.0 104.3,127.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:3.1s" points="65.7,127.4 27.0,127.4 46.4,94.0 65.7,60.5 85.0,27.0 104.3,60.5 123.6,94.0 143.0,127.4 104.3,127.4 65.7,127.4 85.0,94.0 104.3,127.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="127.4" x2="53.3" y2="127.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.4,127.4 53.3,123.2 53.3,131.7" fill="#fc77be"/><circle cx="65.7" cy="127.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwwdwwdwdd` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `str, bool | null` ([[Boolean]], [[Null]], [[String]])
- **Leaves:** `str` ([[String]])
- **ID:** `moreiotas:string/case`

## Description

Removes a string and a bool or null. If it was true, return the string in upper case. If false, lowercase. If null, toggle each character's case.

## Related patterns (same book page)

- [[Separation Distillation]] `str, str → [str]`
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

## Sources

- 📖 **Guidebook:** [[Strings]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Case Distillation](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/case)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCaseString` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:string/case`
