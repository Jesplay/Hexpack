---
aliases: ["Assessor's Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/storage]
kind: "Pattern"
family: "Purification"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:writable/entity"
signature: "wdwewewewewewqw"
start_direction: "EAST"
input: "entity"
output: "bool"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Reading and Writing]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Assessor's Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Reading and Writing]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="112" viewBox="0 0 170.0 111.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.1" cy="16.1" r="2"/><circle cx="39.1" cy="16.1" r="2"/><circle cx="62.0" cy="16.1" r="2"/><circle cx="85.0" cy="16.1" r="2"/><circle cx="108.0" cy="16.1" r="2"/><circle cx="130.9" cy="16.1" r="2"/><circle cx="153.9" cy="16.1" r="2"/><circle cx="4.6" cy="36.0" r="2"/><circle cx="27.6" cy="36.0" r="2"/><circle cx="50.5" cy="36.0" r="2"/><circle cx="73.5" cy="36.0" r="2"/><circle cx="96.5" cy="36.0" r="2"/><circle cx="119.5" cy="36.0" r="2"/><circle cx="142.4" cy="36.0" r="2"/><circle cx="165.4" cy="36.0" r="2"/><circle cx="16.1" cy="55.9" r="2"/><circle cx="39.1" cy="55.9" r="2"/><circle cx="62.0" cy="55.9" r="2"/><circle cx="85.0" cy="55.9" r="2"/><circle cx="108.0" cy="55.9" r="2"/><circle cx="130.9" cy="55.9" r="2"/><circle cx="153.9" cy="55.9" r="2"/><circle cx="4.6" cy="75.8" r="2"/><circle cx="27.6" cy="75.8" r="2"/><circle cx="50.5" cy="75.8" r="2"/><circle cx="73.5" cy="75.8" r="2"/><circle cx="96.5" cy="75.8" r="2"/><circle cx="119.5" cy="75.8" r="2"/><circle cx="142.4" cy="75.8" r="2"/><circle cx="165.4" cy="75.8" r="2"/><circle cx="16.1" cy="95.7" r="2"/><circle cx="39.1" cy="95.7" r="2"/><circle cx="62.0" cy="95.7" r="2"/><circle cx="85.0" cy="95.7" r="2"/><circle cx="108.0" cy="95.7" r="2"/><circle cx="130.9" cy="95.7" r="2"/><circle cx="153.9" cy="95.7" r="2"/></g><polyline class="hexpat-base" points="62.0,55.9 85.0,55.9 108.0,55.9 96.5,75.8 85.0,95.7 62.0,95.7 39.1,95.7 27.6,75.8 16.1,55.9 27.6,36.0 39.1,16.1 62.0,16.1 85.0,16.1 96.5,36.0 108.0,55.9 130.9,55.9 153.9,55.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:368;--dur:4.5s" points="62.0,55.9 85.0,55.9 108.0,55.9 96.5,75.8 85.0,95.7 62.0,95.7 39.1,95.7 27.6,75.8 16.1,55.9 27.6,36.0 39.1,16.1 62.0,16.1 85.0,16.1 96.5,36.0 108.0,55.9 130.9,55.9 153.9,55.9" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="62.0" y1="55.9" x2="69.4" y2="55.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.7,55.9 69.4,58.4 69.4,53.3" fill="#fc77be"/><circle cx="62.0" cy="55.9" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwewewewewewqw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:writable/entity`

```text
before   … │ entity  ◂ top
after    … │ bool    ◂ top
```

## Description

Like [[Assessor's Reflection]], but the writability of an entity is checked instead of my other hand.

## Other uses (same pattern, other inputs)

### [[IoticBlocks]] · 📖 [[ReadingWriting Blocks]]

`vec` → `bool`

Like [[Assessor's Reflection]], but the writability of a block is checked instead of my other hand.

## Related patterns (same book page)

- [[Chronicler's Purification]] `entity → any`
- [[Chronicler's Gambit]] `entity, any → —`
- [[Auditor's Purification]] `entity → bool`
- [[Scribe's Reflection]] `— → any`
- [[Scribe's Gambit]] `any → —`
- [[Auditor's Reflection]] `— → bool`
- [[Assessor's Reflection]] `— → bool`
- [[Huginn's Gambit]] `any → —`
- [[Muninn's Reflection]] `— → any`

## Sources

- 📖 **Guidebook:** [[Reading and Writing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Assessor's Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite@hexcasting:writable/entity)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTheCoolerWritable` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:writable/entity`
