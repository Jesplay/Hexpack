---
aliases: ["Assessor's Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/storage]
kind: "Pattern"
family: "Reflection"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:writable"
signature: "deeeeeq"
start_direction: "EAST"
input: ""
output: "bool"
cost: ""
iotas_in: []
iotas_out: ["[[Boolean]]"]
entry: "[[Reading and Writing]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Assessor's Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Reading and Writing]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,60.5 104.3,60.5 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:309;--dur:2.2s" points="65.7,60.5 104.3,60.5 85.0,94.0 46.4,94.0 27.0,60.5 46.4,27.0 85.0,27.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="60.5" x2="78.0" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="86.9,60.5 78.0,64.8 78.0,56.3" fill="#fc77be"/><circle cx="65.7" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeeeq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexcasting:writable`

```text
before   …         ◂ top
after    … │ bool  ◂ top
```

## Description

If I could save an [[Iotas|iota]] into the item in my other hand, returns True. Otherwise, returns False.

## Related patterns (same book page)

- [[Chronicler's Purification]] `entity → any`
- [[Chronicler's Gambit]] `entity, any → —`
- [[Auditor's Purification]] `entity → bool`
- [[Assessor's Purification]] `entity → bool`
- [[Scribe's Reflection]] `— → any`
- [[Scribe's Gambit]] `any → —`
- [[Auditor's Reflection]] `— → bool`
- [[Huginn's Gambit]] `any → —`
- [[Muninn's Reflection]] `— → any`

## Sources

- 📖 **Guidebook:** [[Reading and Writing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Assessor's Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite@hexcasting:writable)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpWritable` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:writable`
