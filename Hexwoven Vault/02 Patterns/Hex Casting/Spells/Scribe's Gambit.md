---
aliases: ["Scribe's Gambit"]
tags: [hexcasting/spell, mod/hex-casting, family/gambit, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:write"
signature: "deeeee"
start_direction: "EAST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Reading and Writing]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Scribe's Gambit

✦ **Spell** · [[Hex Casting]] · 📖 [[Reading and Writing]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="85.0,78.3 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="85.0,78.3 135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="78.3" x2="101.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.5,78.3 101.0,83.8 101.0,72.8" fill="#fc77be"/><circle cx="85.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:write`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Remove the top [[Iotas|iota]] from the [[The Stack|stack]], and save it into the item in my other hand.

## Example

Hold an empty [[Focus]] in your other hand, put the iota you want to save on top (often a list of patterns from [[Introspection]]/[[Retrospection]]), then **Scribe's Gambit**. Read it back with [[Scribe's Reflection]].

## Related patterns (same book page)

- [[Chronicler's Purification]] `entity → any`
- [[Chronicler's Gambit]] `entity, any → —`
- [[Auditor's Purification]] `entity → bool`
- [[Assessor's Purification]] `entity → bool`
- [[Scribe's Reflection]] `— → any`
- [[Auditor's Reflection]] `— → bool`
- [[Assessor's Reflection]] `— → bool`
- [[Huginn's Gambit]] `any → —`
- [[Muninn's Reflection]] `— → any`

## Sources

- 📖 **Guidebook:** [[Reading and Writing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Scribe's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite@hexcasting:write)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpWrite` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:write`
