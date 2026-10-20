---
aliases: ["Sifter's Gambit"]
tags: [hexcasting/pattern, mod/moreiotas, family/gambit, topic/text]
kind: "Pattern"
family: "Gambit"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/chat/prefix/set"
signature: "qwaqa"
start_direction: "SOUTH_EAST"
input: "str | null"
output: ""
cost: ""
iotas_in: ["[[Null]]", "[[String]]"]
iotas_out: []
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Sifter's Gambit

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/></g><polyline class="hexpat-base" points="30.5,30.5 52.3,68.3 95.9,68.3 139.5,68.3 117.7,30.5 74.1,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:262;--dur:1.7s" points="30.5,30.5 52.3,68.3 95.9,68.3 139.5,68.3 117.7,30.5 74.1,30.5 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="30.5" x2="37.5" y2="42.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.5,51.3 33.3,45.0 41.6,40.2" fill="#fc77be"/><circle cx="30.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwaqa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `str | null` ([[Null]], [[String]])
- **Leaves:** `nothing` (—)
- **ID:** `moreiotas:string/chat/prefix/set`

## Description

Accepts a string; All future chat messages starting with that string won't be seen by others, and only messages prefixed with that string can be read by [[Whisper Reflection]].

## Related patterns (same book page)

- [[Separation Distillation]] `str, str → [str]`
- [[Input Purification]] `str → num | null`
- [[Blank Reflection]] `— → str`
- [[Spacing Reflection]] `— → str`
- [[Comma Reflection]] `— → str`
- [[Breaking Reflection]] `— → str`
- [[Whisper Reflection]] `— → str`
- [[Listener's Reflection]] `— → str`
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
- 🌐 **Official online docs:** [Sifter's Gambit](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/chat/prefix/set)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetChatPrefix` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:string/chat/prefix/set`
