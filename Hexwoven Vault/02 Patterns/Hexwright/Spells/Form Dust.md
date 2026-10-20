---
aliases: ["Form Dust"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:dust_form"
signature: "waaqae"
start_direction: "EAST"
input: "region"
output: ""
cost: ""
iotas_in: ["[[Region]]"]
iotas_out: []
entry: "[[Controlled Dust]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Form Dust

✦ **Spell** · [[Hexwright]] · 📖 [[Controlled Dust]] · Hex Wright › Abilities

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 104.3,60.5 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:2.0s" points="27.0,60.5 65.7,60.5 104.3,60.5 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5 143.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waaqae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `region` ([[Region]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:dust_form`

```text
before   … │ region  ◂ top
after    …           ◂ top
```

## Description

Gives the dust a [[Regions|region]] to fill. The whole region must be within your ambit.

Formed dust is solid to everyone, you included.

Forming any other region, even the same shape somewhere else, reforms the dust. To move a formation without reforming, use Impel or Move.

Formations more than 48 blocks from the caster returns to dust.

## Related patterns (same book page)

- [[Manifest Dust]] `num → —`
- [[Direct Dust]] `vec → —`
- [[Impel Dust]] `vec → —`
- [[Move Dust]] `vector → —`
- [[Constrain Dust]] `bool → —`
- [[Recall Dust]] `— → —`
- [[Await Dust]] `[pattern] | pattern → —`

## Sources

- 📖 **Guidebook:** [[Controlled Dust]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `DustActions$3` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:dust_form`
