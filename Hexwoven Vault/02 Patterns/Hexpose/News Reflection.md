---
aliases: ["News Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/text]
kind: "Pattern"
family: "Reflection"
topics: ["Text, Strings & Chat"]
mod: "Hexpose"
op_id: "hexpose:get_message"
signature: "aeeedw"
start_direction: "SOUTH_WEST"
input: ""
output: "text, text, number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]", "[[String]]"]
entry: "[[Chat]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# News Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[Chat]] · Patterns › Scrying

**Topics:** [[Text, Strings & Chat (Topic)|Text, Strings & Chat]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="123" height="170" viewBox="0 0 123.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,29.8 29.8,66.6 72.3,66.6 93.5,103.4 72.3,140.2 29.8,140.2 51.0,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:2.0s" points="51.0,29.8 29.8,66.6 72.3,66.6 93.5,103.4 72.3,140.2 29.8,140.2 51.0,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="29.8" x2="44.2" y2="41.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,50.0 40.2,39.2 48.3,43.9" fill="#fc77be"/><circle cx="51.0" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeeedw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `text, text, number` ([[Number]], [[String]])
- **ID:** `hexpose:get_message`

```text
before   …                         ◂ top
after    … │ text │ text │ number  ◂ top
```

## Description

Pushes a [[Text Iota|text iota]] of the last person to use chat, a [[Text Iota|text iota]] of the message, and a number of twenieths of a second since that message sent.

## Related patterns (same book page)

- [[Newspaper Reflection]] `— → list of lists`

## Sources

- 📖 **Guidebook:** [[Chat]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [News Reflection](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/chat@hexpose:get_message)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMessage` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_message`
