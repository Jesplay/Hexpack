---
aliases: ["Payload Gambit"]
tags: [hexcasting/spell, mod/hierophantics, family/gambit, topic/minds]
kind: "Spell"
family: "Gambit"
topics: ["Minds & Hierophantics"]
mod: "Hierophantics"
op_id: "hierophantics:set_mind_hex"
signature: "ddeedd"
start_direction: "SOUTH_EAST"
input: "mind, [pattern] | null"
output: ""
cost: "Costs one Charged Amethyst."
iotas_in: ["[[List]]", "[[Pattern]]", "[[Null]]", "[[Mind]]"]
iotas_out: []
entry: "[[Hierophantic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Payload Gambit

✦ **Spell** · [[Hierophantics]] · 📖 [[Hierophantic Patterns]] · Patterns

**Topics:** [[Minds & Hierophantics (Topic)|Minds & Hierophantics]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="60.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 85.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:2.0s" points="60.0,35.0 85.0,78.3 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 85.0,78.3 110.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.0" y1="35.0" x2="68.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="73.7,58.8 63.2,51.6 72.8,46.1" fill="#fc77be"/><circle cx="60.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddeedd` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `mind, [pattern] | null` ([[List]], [[Pattern]], [[Null]], [[Mind]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one Charged Amethyst.
- **ID:** `hierophantics:set_mind_hex`

## Description

Stores the provided **Hex** into the mind. If there is already a stored **Hex**, it will be overwritten. Costs one [[Amethyst|Charged Amethyst]].

## Related patterns (same book page)

- [[Hierophant's Reflection]] `— → [mind]`
- [[Stabilize Minds]] `— → —`
- [[Payload Purification]] `mind → [pattern] | null`
- [[Trigger Gambit]] `mind, trigger | null → —`
- [[Trigger Purification]] `mind → trigger | null`
- [[Mute Mind]] `mind, bool → —`
- [[Free Mind]] `mind → —`

## Sources

- 📖 **Guidebook:** [[Hierophantic Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hierophantics on Modrinth](https://modrinth.com/mod/ybwf7iLN)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpSetMindHex` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hierophantics:set_mind_hex`
