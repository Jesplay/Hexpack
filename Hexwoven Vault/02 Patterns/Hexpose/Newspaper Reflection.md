---
aliases: ["Newspaper Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/text]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat"]
mod: "Hexpose"
op_id: "hexpose:get_chat"
signature: "dqqqaw"
start_direction: "SOUTH_EAST"
input: ""
output: "list of lists"
cost: ""
iotas_in: []
iotas_out: ["[[List]]"]
entry: "[[Chat]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Newspaper Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[Chat]] · Patterns › Scrying

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,29.8 93.5,66.6 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 72.3,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="72.3,29.8 93.5,66.6 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 72.3,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="29.8" x2="79.1" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.0,50.0 75.0,43.9 83.1,39.2" fill="#fc77be"/><circle cx="72.3" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqqqaw` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `list of lists` ([[List]])
- **ID:** `hexpose:get_chat`

```text
before   …                  ◂ top
after    … │ list of lists  ◂ top
```

## Description

Applies [[News Reflection]] to the last up to 32 messages in chat, wraps each in a list, and pushes a list of those lists.

## Related patterns (same book page)

- [[News Reflection]] `— → text, text, number`

## Sources

- 📖 **Guidebook:** [[Chat]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetChat` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_chat`
