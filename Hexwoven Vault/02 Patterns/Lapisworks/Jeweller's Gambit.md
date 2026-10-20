---
aliases: ["Jeweller's Gambit"]
tags: [hexcasting/pattern, mod/lapisworks, family/gambit, topic/items]
kind: "Pattern"
family: "Gambit"
topics: ["Items & Crafting"]
mod: "Lapisworks"
op_id: "lapisworks:write_necklace"
signature: "wadeeeeeq"
start_direction: "NORTH_WEST"
input: "any"
output: ""
cost: ""
iotas_in: ["[[Any Iota]]"]
iotas_out: []
entry: "[[Trinkets]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Jeweller's Gambit

◇ **Pattern** · [[Lapisworks]] · 📖 [[Trinkets]] · Lapisworks › Imbuement

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="114.8,140.2 93.5,103.4 72.3,66.6 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="114.8,140.2 93.5,103.4 72.3,66.6 51.0,103.4 29.8,66.6 51.0,29.8 93.5,29.8 114.8,66.6 93.5,103.4 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="114.8" y1="140.2" x2="108.0" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="103.1,120.0 112.1,126.1 104.0,130.8" fill="#fc77be"/><circle cx="114.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wadeeeeeq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any` ([[Any Iota]])
- **Leaves:** `nothing` (—)
- **ID:** `lapisworks:write_necklace`

```text
before   … │ any  ◂ top
after    …        ◂ top
```

## Description

Writes the [[Iotas|iota]] to the necklace I'm wearing. [[Mishaps|Mishaps]] if there is no writable one available. (Drawing aid: a line into almost a triangle, then a clockwise circle much like [[Scribe's Gambit]].)

## Related patterns (same book page)

- [[Jeweller's Reflection]] `— → any`
- [[Gemologist's Reflection]] `— → bool`
- [[Epigrapher's Reflection]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Trinkets]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `WriteNecklace` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:write_necklace`
