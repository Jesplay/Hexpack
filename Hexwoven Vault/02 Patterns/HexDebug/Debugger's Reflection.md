---
aliases: ["Debugger's Reflection"]
tags: [hexcasting/pattern, mod/hexdebug, family/reflection, topic/debugging]
kind: "Pattern"
family: "Reflection"
topics: ["Debugging"]
mod: "HexDebug"
op_id: "hexdebug:const/debugging"
signature: "qqqqqewaa"
start_direction: "EAST"
input: ""
output: "bool"
cost: ""
iotas_in: []
iotas_out: ["[[Boolean]]"]
entry: "[[Debugging Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Debugger's Reflection

◇ **Pattern** · [[HexDebug]] · 📖 [[Debugging Patterns]] · Patterns

**Topics:** [[Debugging (Topic)|Debugging]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="136" height="170" viewBox="0 0 136.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 41.9,54.7 59.4,85.0 41.9,115.3 24.5,145.5 59.4,145.5 41.9,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.8s" points="59.4,85.0 94.4,85.0 111.8,54.7 94.4,24.5 59.4,24.5 41.9,54.7 59.4,85.0 41.9,115.3 24.5,145.5 59.4,145.5 41.9,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="85.0" x2="70.6" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="78.6,85.0 70.6,88.8 70.6,81.2" fill="#fc77be"/><circle cx="59.4" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqewaa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `bool` ([[Boolean]])
- **ID:** `hexdebug:const/debugging`

```text
before   …         ◂ top
after    … │ bool  ◂ top
```

## Description

Add **True** to the top of the [[The Stack|stack]] if executed by a [[Debugging Items|Debugger]] or [[Debugging Items|Evaluator]], or **False** otherwise.

## Related patterns (same book page)

- [[Cognitohazard Reflection]] `— → cognitohazard`
- [[Set Breakpoint Before]] `— → —`
- [[Set Breakpoint After]] `— → —`
- [[Craft Debugger]] `entity, [pattern] → —`
- [[Craft Quenched Debugger]] `entity, [pattern] → —`

## Sources

- 📖 **Guidebook:** [[Debugging Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Debugger's Reflection](https://hexdebug.hexxy.media/v/0.9.0+1.20.1/1.0/en_us/#patterns/debugging@hexdebug:const/debugging)
- 🧩 **Mod page:** [HexDebug on Modrinth](https://modrinth.com/mod/A2pscGC5)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `hexdebug:const/debugging`
