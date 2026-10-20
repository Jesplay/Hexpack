---
aliases: ["Muninn's Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/storage]
kind: "Pattern"
family: "Reflection"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:read/local"
signature: "qeewdweddw"
start_direction: "NORTH_EAST"
input: ""
output: "any"
cost: ""
iotas_in: []
iotas_out: ["[[Any Iota]]"]
entry: "[[Reading and Writing]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Muninn's Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Reading and Writing]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 136.1,29.8 114.8,66.6 93.5,103.4 51.0,103.4 72.3,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="29.8,140.2 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 136.1,29.8 114.8,66.6 93.5,103.4 51.0,103.4 72.3,66.6 93.5,103.4 114.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeewdweddw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `hexcasting:read/local`

```text
before   …        ◂ top
after    … │ any  ◂ top
```

## Description

Copy the [[Iotas|iota]] out of my [[Reading and Writing|ravenmind]], which I likely just wrote with [[Huginn's Gambit]].

## Example

Pushes whatever is in the ravenmind ([[Null]] if nothing was saved with [[Huginn's Gambit]] yet).

## Related patterns (same book page)

- [[Chronicler's Purification]] `entity → any`
- [[Chronicler's Gambit]] `entity, any → —`
- [[Auditor's Purification]] `entity → bool`
- [[Assessor's Purification]] `entity → bool`
- [[Scribe's Reflection]] `— → any`
- [[Scribe's Gambit]] `any → —`
- [[Auditor's Reflection]] `— → bool`
- [[Assessor's Reflection]] `— → bool`
- [[Huginn's Gambit]] `any → —`

## Sources

- 📖 **Guidebook:** [[Reading and Writing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Muninn's Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite@hexcasting:read/local)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPeekLocal` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:read/local`
