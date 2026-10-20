---
aliases: ["Patternmaster's Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/text]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/action"
signature: "wdwewdwdw"
start_direction: "NORTH_WEST"
input: "pattern"
output: "str | null"
cost: ""
iotas_in: ["[[Pattern]]"]
iotas_out: ["[[Null]]", "[[String]]"]
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Patternmaster's Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,94.0 46.4,60.5 27.0,27.0 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 104.3,94.0 65.7,94.0 85.0,60.5 104.3,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:386;--dur:2.8s" points="65.7,94.0 46.4,60.5 27.0,27.0 65.7,27.0 104.3,27.0 123.6,60.5 143.0,94.0 104.3,94.0 65.7,94.0 85.0,60.5 104.3,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="94.0" x2="59.5" y2="83.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,75.6 63.2,81.1 55.8,85.4" fill="#fc77be"/><circle cx="65.7" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwewdwdw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `pattern` ([[Pattern]])
- **Leaves:** `str | null` ([[Null]], [[String]])
- **ID:** `moreiotas:string/action`

## Description

Converts the pattern on top of the [[The Stack|stack]] into the string for that pattern's name. If there is no action associated with that pattern, or if the associated pattern is a Great Spell, it returns null.

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
- [[Moniker Purification]] `entity → str`
- [[Name]] `str, entity → —`
- [[Case Distillation]] `str, bool | null → str`

## Sources

- 📖 **Guidebook:** [[Strings]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Patternmaster's Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/action)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpActionString` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:string/action`
