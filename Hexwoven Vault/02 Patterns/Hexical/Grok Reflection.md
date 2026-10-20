---
aliases: ["Grok Reflection"]
tags: [hexcasting/pattern, mod/hexical, family/reflection, topic/storage]
kind: "Pattern"
family: "Reflection"
topics: ["Reading & Writing"]
mod: "Hexical"
op_id: "hexical:grok_pull"
signature: "aqawwqaw"
start_direction: "EAST"
input: ""
output: "list, list"
cost: ""
iotas_in: []
iotas_out: ["[[List]]"]
entry: "[[Grok]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Grok Reflection

◇ **Pattern** · [[Hexical]] · 📖 [[Grok]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,66.6 93.5,66.6 72.3,29.8 29.8,29.8 51.0,66.6 72.3,103.4 93.5,140.2 136.1,140.2 114.8,103.4 93.5,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="51.0,66.6 93.5,66.6 72.3,29.8 29.8,29.8 51.0,66.6 72.3,103.4 93.5,140.2 136.1,140.2 114.8,103.4 93.5,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="66.6" x2="64.6" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,66.6 64.6,71.3 64.6,61.9" fill="#fc77be"/><circle cx="51.0" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawwqaw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `list, list` ([[List]])
- **ID:** `hexical:grok_pull`

```text
before   …                ◂ top
after    … │ list │ list  ◂ top
```

## Description

Pushes two lists; one of the [[The Stack|stack]] and one of the unfinished list building after an unclosed [[Introspection]]. Entity references to players that are not me become Null.

## Related patterns (same book page)

- [[Grok Gambit]] `list, list → —`

## Sources

- 📖 **Guidebook:** [[Grok]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGrokPull` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:grok_pull`
