---
aliases: ["Place Block"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hex Casting"
op_id: "hexcasting:place_block"
signature: "eeeeede"
start_direction: "SOUTH_WEST"
input: "vector"
output: ""
cost: "Costs about an eighth of one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Block Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Place Block

✦ **Spell** · [[Hex Casting]] · 📖 [[Block Manipulation]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 85.0,78.3 60.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="135.0,78.3 110.0,121.6 60.0,121.6 35.0,78.3 60.0,35.0 110.0,35.0 135.0,78.3 85.0,78.3 60.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="78.3" x2="127.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="121.2,102.1 122.2,89.4 131.8,94.9" fill="#fc77be"/><circle cx="135.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeede` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about an eighth of one Amethyst Dust.
- **ID:** `hexcasting:place_block`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Remove a location from the [[The Stack|stack]], then pick a block item and place it at the given location.
Costs about an eighth of one [[Amethyst|Amethyst Dust]].

## Example

Place a block against the face you're looking at:
1. [[Archer's Distillation]] setup → block position
2. same setup again → [[Architect's Distillation]] → face normal
3. [[Additive Distillation]] → the empty spot in front of that face
4. **Place Block** (uses a block from your inventory)

## Related patterns (same book page)

- [[Break Block]] `vector → —`
- [[Create Water]] `vector → —`
- [[Destroy Liquid]] `vector → —`
- [[Conjure Block]] `vector → —`
- [[Conjure Light]] `vector → —`
- [[Overgrow]] `vector → —`
- [[Edify Sapling]] `vector → —`
- [[Ignite]] `entity | vector → —`
- [[Extinguish Area]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Block Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Place Block](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/blockworks@hexcasting:place_block)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPlaceBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:place_block`
