---
aliases: ["Draw Text"]
tags: [hexcasting/pattern, mod/scryglass, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Scryglass"
op_id: "scryglass:draw_text"
signature: "aaqdwdwd"
start_direction: "NORTH_EAST"
input: "index, vector, number, any"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Any Iota]]"]
iotas_out: []
entry: "[[Viewforming]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Draw Text

◇ **Pattern** · [[Scryglass]] · 📖 [[Viewforming]] · Patterns

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,66.6 93.5,29.8 51.0,29.8 72.3,66.6 114.8,66.6 93.5,103.4 72.3,140.2 51.0,103.4 29.8,66.6 72.3,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="72.3,66.6 93.5,29.8 51.0,29.8 72.3,66.6 114.8,66.6 93.5,103.4 72.3,140.2 51.0,103.4 29.8,66.6 72.3,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="66.6" x2="79.1" y2="54.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.0,46.3 83.1,57.1 75.0,52.5" fill="#fc77be"/><circle cx="72.3" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqdwdwd` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `index, vector, number, any` ([[Number]], [[Vector]], [[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `scryglass:draw_text`

```text
before   … │ index │ vector │ number │ any  ◂ top
after    …                                  ◂ top
```

## Description

Takes an index, a position, a number, and any [[Iotas|iota]]. Conjures an **Icon** displaying that iota.

The number determines the mode of text justification. 0 centers the text at the position given, 1 aligns the left side to the position given, and 2 aligns the right side to the position given.

## Related patterns (same book page)

- [[Periphery Reflection]] `— → vector`
- [[Icons Reflection]] `— → list of indices`
- [[Clear Icon]] `index → —`
- [[Rotate Icon]] `index, number → —`
- [[Scale Icon]] `index, number → —`
- [[Draw Rectangle]] `index, vector, vector, vector → —`
- [[Draw Line]] `index, vector, vector, vector → —`

## Sources

- 📖 **Guidebook:** [[Viewforming]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Draw Text](https://scryglass.hexxy.media/v/latest/main/en_us/#scryglass@scryglass:draw_text)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpDrawText` is not a `SpellAction`
- 🔧 **Pattern ID:** `scryglass:draw_text`
