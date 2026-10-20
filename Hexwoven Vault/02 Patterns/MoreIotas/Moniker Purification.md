---
aliases: ["Moniker Purification"]
tags: [hexcasting/pattern, mod/moreiotas, family/purification, topic/text]
kind: "Pattern"
family: "Purification"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/name/get"
signature: "deqqeddqwqqqwq"
start_direction: "SOUTH_EAST"
input: "entity"
output: "str"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[String]]"]
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Moniker Purification

◇ **Pattern** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="132.2,49.3 148.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,76.6 22.0,76.6 37.8,49.3 53.5,76.6 85.0,76.6 116.5,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="132.2,49.3 148.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,76.6 22.0,76.6 37.8,49.3 53.5,76.6 85.0,76.6 116.5,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0 37.8,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="132.2" y1="49.3" x2="137.3" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="140.9,64.3 134.3,59.8 140.3,56.3" fill="#fc77be"/><circle cx="132.2" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deqqeddqwqqqwq` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `str` ([[String]])
- **ID:** `moreiotas:string/name/get`

```text
before   … │ entity  ◂ top
after    … │ str     ◂ top
```

## Description

Takes an entity and returns that entity's name. If it is an item entity, return the item's name.

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
- [[Name]] `str, entity → —`
- [[Case Distillation]] `str, bool | null → str`

## Sources

- 📖 **Guidebook:** [[Strings]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Moniker Purification](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/name/get)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpNameGet` is not a `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:string/name/get`
