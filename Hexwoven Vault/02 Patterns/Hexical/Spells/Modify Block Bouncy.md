---
aliases: ["Modify Block: Bouncy"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/blocks]
kind: "Spell"
family: "Spell"
topics: ["World: Blocks, Terrain & Weather"]
mod: "Hexical"
op_id: "hexical:modify_block_bouncy"
signature: "deeqa"
start_direction: "NORTH_WEST"
input: "vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Mage Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Modify Block: Bouncy

✦ **Spell** · [[Hexical]] · 📖 [[Mage Blocks]] · Patterns › Spells

**Topics:** [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,66.6 29.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4 93.5,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="51.0,66.6 29.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4 93.5,140.2 114.8,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="66.6" x2="44.2" y2="54.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,46.3 48.3,52.5 40.2,57.1" fill="#fc77be"/><circle cx="51.0" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:modify_block_bouncy`

```text
before   … │ vector  ◂ top
after    …           ◂ top
```

## Description

Makes the block bouncy, like a **Slime Block**. Bounces me regardless of whether I am sneaking. Delightfully fun!

## Related patterns (same book page)

- [[Conjure Mage Block]] `vector → —`
- [[Reset Mage Block]] `vector → —`
- [[Modify Block Energized]] `vector, number → —`
- [[Modify Block Ephemeral]] `vector, number → —`
- [[Modify Block Volatile]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Mage Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Modify Block: Bouncy](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/mage_blocks@hexical:modify_block_bouncy)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConfigureMageBlock` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:modify_block_bouncy`
