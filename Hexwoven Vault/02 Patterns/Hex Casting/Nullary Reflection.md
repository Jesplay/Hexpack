---
aliases: ["Nullary Reflection"]
tags: [hexcasting/pattern, mod/hex-casting, family/reflection, topic/logic]
kind: "Pattern"
family: "Reflection"
topics: ["Logic & Comparison"]
mod: "Hex Casting"
op_id: "hexcasting:const/null"
signature: "d"
start_direction: "EAST"
input: ""
output: "null"
cost: ""
iotas_in: []
iotas_out: ["[[Null]]"]
entry: "[[Constants]]"
needs_enlightenment: false
per_world: false
kind_source: "book"
topic_source: "pattern"
in_book: true
---

# Nullary Reflection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Constants]] · Patterns

**Topics:** [[Logic & Comparison (Topic)|Logic & Comparison]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="148" viewBox="0 0 156.9 148.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="45.8" cy="45.8" r="2"/><circle cx="111.2" cy="45.8" r="2"/><circle cx="13.1" cy="102.4" r="2"/><circle cx="78.5" cy="102.4" r="2"/><circle cx="143.8" cy="102.4" r="2"/></g><polyline class="hexpat-base" points="45.8,45.8 111.2,45.8 78.5,102.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:131;--dur:1.6s" points="45.8,45.8 111.2,45.8 78.5,102.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.8" y1="45.8" x2="66.7" y2="45.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="81.7,45.8 66.7,53.0 66.7,38.6" fill="#fc77be"/><circle cx="45.8" cy="45.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `d` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `null` ([[Null]])
- **ID:** `hexcasting:const/null`

```text
before   …         ◂ top
after    … │ null  ◂ top
```

## Description

Adds the **Null** influence to the top of the [[The Stack|stack]].

## Related patterns (same book page)

- [[True Reflection]] `— → bool`
- [[False Reflection]] `— → bool`
- [[Vector Reflection Zero]] `— → vector`
- [[Vector Rfln. +X-X]] `— → vector`
- [[Vector Rfln. +Y-Y]] `— → vector`
- [[Vector Rfln. +Z-Z]] `— → vector`
- [[Circle's Reflection]] `— → num`
- [[Arc's Reflection]] `— → num`
- [[Euler's Reflection]] `— → num`
- [[Harmony Reflection]] `— → num`

## Sources

- 📖 **Guidebook:** [[Constants]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Nullary Reflection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/consts@hexcasting:const/null)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the chapter the book lists it in
- 🔧 **Pattern ID:** `hexcasting:const/null`
