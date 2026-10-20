---
aliases: ["Huginn's Gambit"]
tags: [hexcasting/pattern, mod/hex-casting, family/gambit, topic/storage]
kind: "Pattern"
family: "Gambit"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:write/local"
signature: "eqqwawqaaw"
start_direction: "NORTH_WEST"
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

# Huginn's Gambit

◇ **Pattern** · [[Hex Casting]] · 📖 [[Reading and Writing]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="136.1,140.2 114.8,103.4 136.1,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6 72.3,103.4 114.8,103.4 93.5,66.6 72.3,103.4 51.0,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="136.1,140.2 114.8,103.4 136.1,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6 72.3,103.4 114.8,103.4 93.5,66.6 72.3,103.4 51.0,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="136.1" y1="140.2" x2="129.3" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="124.4,120.0 133.3,126.1 125.2,130.8" fill="#fc77be"/><circle cx="136.1" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqwawqaaw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:write/local`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Removes the top [[Iotas|iota]] from the [[The Stack|stack]], and saves it to my [[Reading and Writing|ravenmind]], storing it there until I stop casting the **Hex**.

## Example

Saves the top iota in the **ravenmind**, the hex's single private variable, until the hex ends. It survives across [[Thoth's Gambit]] iterations, which makes it the way to keep a running total or remember "the best target so far" during a loop. Read it with [[Muninn's Reflection]].

## Related patterns (same book page)

- [[Chronicler's Purification]] `entity → any`
- [[Chronicler's Gambit]] `entity, any → —`
- [[Auditor's Purification]] `entity → bool`
- [[Assessor's Purification]] `entity → bool`
- [[Scribe's Reflection]] `— → any`
- [[Scribe's Gambit]] `any → —`
- [[Auditor's Reflection]] `— → bool`
- [[Assessor's Reflection]] `— → bool`
- [[Muninn's Reflection]] `— → any`

## Sources

- 📖 **Guidebook:** [[Reading and Writing]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Huginn's Gambit](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite@hexcasting:write/local)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpPushLocal` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:write/local`
