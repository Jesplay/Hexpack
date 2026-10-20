---
aliases: ["Shard Reflection"]
tags: [hexcasting/pattern, mod/hexthings, family/reflection, topic/sentinels]
kind: "Pattern"
family: "Reflection"
topics: ["Sentinels"]
mod: "HexThings"
op_id: "hexthings:isgreatersent"
signature: "waeawaeqqq"
start_direction: "EAST"
input: ""
output: "bool | null"
cost: ""
iotas_in: []
iotas_out: ["[[Boolean]]", "[[Null]]"]
entry: "[[Utilities]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "pattern"
in_book: true
---

# Shard Reflection

◇ **Pattern** · [[HexThings]] · 📖 [[Utilities]] · Patterns

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,103.4 72.3,103.4 114.8,103.4 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="29.8,103.4 72.3,103.4 114.8,103.4 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="103.4" x2="43.4" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="53.2,103.4 43.4,108.1 43.4,98.7" fill="#fc77be"/><circle cx="29.8" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waeawaeqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `bool | null` ([[Boolean]], [[Null]])
- **ID:** `hexthings:isgreatersent`

## Description

Pushes if my current sentinel is a [[Summon Greater Sentinel (Book Entry)|Greater Sentinel]], or [[Nullary Reflection|Null]] if I have no sentinel at all.

## Related patterns (same book page)

- [[Patience]] `— → —`
- [[Patience Patterns]] `— → —`
- [[Introjection]] `— → —`
- [[Retrojection]] `— → —`

## Sources

- 📖 **Guidebook:** [[Utilities]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexThings on Modrinth](https://modrinth.com/mod/uRBPvIjK)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexthings:isgreatersent`
