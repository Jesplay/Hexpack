---
aliases: ["Recall Dust"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:dust_recall"
signature: "wdd"
start_direction: "EAST"
input: ""
output: ""
cost: ""
iotas_in: []
iotas_out: []
entry: "[[Controlled Dust]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Recall Dust

✦ **Spell** · [[Hexwright]] · 📖 [[Controlled Dust]] · Hex Wright › Abilities

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="35.0,35.0 85.0,35.0 135.0,35.0 110.0,78.3 85.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:1.6s" points="35.0,35.0 85.0,35.0 135.0,35.0 110.0,78.3 85.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="35.0" x2="51.0" y2="35.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,35.0 51.0,40.5 51.0,29.5" fill="#fc77be"/><circle cx="35.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:dust_recall`

## Description

Recalls the dust back to you, releasing any current formations.

## Related patterns (same book page)

- [[Manifest Dust]] `num → —`
- [[Direct Dust]] `vec → —`
- [[Form Dust]] `region → —`
- [[Impel Dust]] `vec → —`
- [[Move Dust]] `vector → —`
- [[Constrain Dust]] `bool → —`
- [[Await Dust]] `[pattern] | pattern → —`

## Sources

- 📖 **Guidebook:** [[Controlled Dust]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `DustActions$7` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:dust_recall`
