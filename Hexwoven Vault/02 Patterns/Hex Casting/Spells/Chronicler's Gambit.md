---
aliases: ["Chronicler's Gambit"]
tags: [hexcasting/spell, mod/hex-casting, family/gambit, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:write/entity"
signature: "wdwewewewewew"
start_direction: "EAST"
input: "entity, any"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Reading and Writing]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Chronicler's Gambit

✦ **Spell** · [[Hex Casting]] · 📖 [[Reading and Writing]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,76.6 116.5,76.6 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:441;--dur:3.9s" points="85.0,76.6 116.5,76.6 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 85.0,22.0 116.5,22.0 132.2,49.3 148.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="76.6" x2="95.1" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.3,76.6 95.1,80.0 95.1,73.1" fill="#fc77be"/><circle cx="85.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwewewewewew` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, any` ([[Entity]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:write/entity`

```text
before   … │ entity │ any  ◂ top
after    …                 ◂ top
```

## Description

Like [[Scribe's Reflection|Scribe's Gambit]], but the [[Iotas|iota]] is written to an entity instead of my other hand.

Interestingly enough, it looks like I cannot write my own Name using this spell. I get a sense that I might be endangered if I could.

## Other uses (same pattern, other inputs)

### [[IoticBlocks]] · 📖 [[ReadingWriting Blocks]]

`vec, any` → `—`

Like [[Scribe's Gambit]], but the iota is written to a block instead of my other hand.

Interestingly enough, it looks like I cannot write my own Name using this spell. I get a sense that I might be endangered if I could.

## Related patterns (same book page)

- [[Chronicler's Purification]] `entity → any`
- [[Auditor's Purification]] `entity → bool`
- [[Assessor's Purification]] `entity → bool`
- [[Scribe's Reflection]] `— → any`
- [[Scribe's Gambit]] `any → —`
- [[Auditor's Reflection]] `— → bool`
- [[Assessor's Reflection]] `— → bool`
- [[Huginn's Gambit]] `any → —`
- [[Muninn's Reflection]] `— → any`

## Sources

- 📖 **Guidebook:** [[Reading and Writing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Chronicler's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite@hexcasting:write/entity)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpTheCoolerWrite` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:write/entity`
