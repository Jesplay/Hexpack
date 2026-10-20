---
aliases: ["Gravitate"]
tags: [hexcasting/spell, mod/hexodus, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hexodus"
op_id: "hexodus:change_gravity"
signature: "wawawqaqwa"
start_direction: "NORTH_EAST"
input: "entity, vector, number"
output: ""
cost: "costs about one-fourth of an Amethyst Dust."
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Gravitation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gravitate

✦ **Spell** · [[Hexodus]] · 📖 [[Gravitation]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="95.9,106.0 117.7,68.3 139.5,30.5 95.9,30.5 52.3,30.5 74.1,68.3 95.9,106.0 139.5,106.0 117.7,68.3 74.1,68.3 30.5,68.3 52.3,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:479;--dur:3.1s" points="95.9,106.0 117.7,68.3 139.5,30.5 95.9,30.5 52.3,30.5 74.1,68.3 95.9,106.0 139.5,106.0 117.7,68.3 74.1,68.3 30.5,68.3 52.3,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="95.9" y1="106.0" x2="102.9" y2="93.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.9,85.3 107.0,96.3 98.7,91.5" fill="#fc77be"/><circle cx="95.9" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawawqaqwa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vector, number` ([[Number]], [[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** costs about one-fourth of an Amethyst Dust.
- **ID:** `hexodus:change_gravity`

```text
before   … │ entity │ vector │ number  ◂ top
after    …                             ◂ top
```

## Description

Takes an entity, an axial vector for new gravity, and a strength scalar relative to standard pull. Specifics are detailed on the next page.

The scalar can be any number 0 to 5 inclusive.

If the target is myself, this spell costs about one-fourth of an [[Amethyst|Amethyst Dust]]. If the target is another player, costs about one [[Amethyst|Amethyst Shard]]. Otherwises, costs about one [[Amethyst|Amethyst Dust]].

Each cast of the spell only lasts for about two and a half seconds; for sustained gravitation, I will need some kind of automated **Hexcasting** to periodically reapply the effects.

## Related patterns (same book page)

- [[Grasp Purification]] `entity → number`
- [[Falling Purification]] `entity → vector`
- [[Impact Purification]] `entity → number`
- [[Falling Purification II]] `entity → vector`
- [[Impact Purification II]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Gravitation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexodus on Modrinth](https://modrinth.com/mod/bN6IKmdE)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpChangeGravity` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexodus:change_gravity`
