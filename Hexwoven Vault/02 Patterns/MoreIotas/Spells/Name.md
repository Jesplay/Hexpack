---
aliases: ["Name"]
tags: [hexcasting/spell, mod/moreiotas, family/spell, topic/text]
kind: "Spell"
family: "Spell"
topics: ["Text, Strings & Chat"]
mod: "MoreIotas"
op_id: "moreiotas:string/name/set"
signature: "aqeeqaaeweeewe"
start_direction: "SOUTH_WEST"
input: "str, entity"
output: ""
cost: "Costs a hundredth of an Amethyst Dust."
iotas_in: ["[[Entity]]", "[[String]]"]
iotas_out: []
entry: "[[Strings]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Name

✦ **Spell** · [[MoreIotas]] · 📖 [[Strings]] · Patterns

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 116.5,76.6 148.0,76.6 132.2,49.3 116.5,76.6 85.0,76.6 53.5,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:472;--dur:4.2s" points="37.8,49.3 22.0,76.6 53.5,76.6 69.3,49.3 100.7,49.3 116.5,76.6 148.0,76.6 132.2,49.3 116.5,76.6 85.0,76.6 53.5,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="37.8" y1="49.3" x2="32.7" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="29.1,64.3 29.7,56.3 35.7,59.8" fill="#fc77be"/><circle cx="37.8" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqeeqaaeweeewe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `str, entity` ([[Entity]], [[String]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a hundredth of an Amethyst Dust.
- **ID:** `moreiotas:string/name/set`

```text
before   … │ str │ entity  ◂ top
after    …                 ◂ top
```

## Description

Takes a string and an entity, and sets that entities name to the given string. If the entity is an item entity, also set the item's name. Costs a hundredth of an [[Amethyst|Amethyst Dust]].

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
- [[Case Distillation]] `str, bool | null → str`

## Sources

- 📖 **Guidebook:** [[Strings]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Name](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings@moreiotas:string/name/set)
- 🧩 **Mod page:** [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpNameSet` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `moreiotas:string/name/set`
