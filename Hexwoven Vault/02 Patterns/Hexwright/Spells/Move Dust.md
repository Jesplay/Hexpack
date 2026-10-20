---
aliases: ["Move Dust"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:dust_move"
signature: "waaqaeq"
start_direction: "EAST"
input: "vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Controlled Dust]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Move Dust

✦ **Spell** · [[Hexwright]] · 📖 [[Controlled Dust]] · Hex Wright › Abilities

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,54.3 59.0,54.3 93.7,54.3 76.3,24.3 59.0,54.3 76.3,84.4 93.7,54.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:2.2s" points="24.3,54.3 59.0,54.3 93.7,54.3 76.3,24.3 59.0,54.3 76.3,84.4 93.7,54.3 128.4,54.3 145.7,24.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="54.3" x2="35.4" y2="54.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,54.3 35.4,58.1 35.4,50.5" fill="#fc77be"/><circle cx="24.3" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waaqaeq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:dust_move`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Carries the formation until its centre is at the position. Acceleration is dependant on size and distance.

Anything standing on a moving formation is carried along with it.

Impel keeps going until you tell it otherwise. Move stops by itself when it gets there, and the position has to be within your ambit. Impelling a formation whilst moving abandons the move.

## Related patterns (same book page)

- [[Manifest Dust]] `num → —`
- [[Direct Dust]] `vec → —`
- [[Form Dust]] `region → —`
- [[Impel Dust]] `vec → —`
- [[Constrain Dust]] `bool → —`
- [[Recall Dust]] `— → —`
- [[Await Dust]] `[pattern] | pattern → —`

## Sources

- 📖 **Guidebook:** [[Controlled Dust]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `DustActions$5` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:dust_move`
