---
aliases: ["Free Mind"]
tags: [hexcasting/spell, mod/hierophantics, family/spell, topic/minds]
kind: "Spell"
family: "Spell"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:free_mind"
signature: "ddeawawa"
start_direction: "SOUTH_EAST"
input: "mind"
output: ""
cost: ""
iotas_in: ["[[Mind]]"]
iotas_out: []
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Free Mind

✦ **Spell** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 93.5,140.2 51.0,140.2 72.3,103.4 114.8,103.4 93.5,66.6 72.3,29.8 51.0,66.6 29.8,103.4 72.3,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="72.3,103.4 93.5,140.2 51.0,140.2 72.3,103.4 114.8,103.4 93.5,66.6 72.3,29.8 51.0,66.6 29.8,103.4 72.3,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="79.1" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="84.0,123.7 75.0,117.5 83.1,112.9" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddeawawa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mind` ([[Mind]])
- **Leaves:** `nothing` (—)
- **ID:** `hierophantics:free_mind`

```text
before   … │ mind  ◂ top
after    …         ◂ top
```

## Description

Permanently removes an embedded mind from my own consciousness.

I wonder what happens to them when they go?

## Related patterns (same book page)

- [[Hierophant's Reflection]] `— → [mind]`
- [[Stabilize Minds]] `— → —`
- [[Payload Gambit]] `mind, [pattern] | null → —`
- [[Payload Purification]] `mind → [pattern] | null`
- [[Trigger Gambit]] `mind, trigger | null → —`
- [[Trigger Purification]] `mind → trigger | null`
- [[Mute Mind]] `mind, bool → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpFreeMind` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:free_mind`
