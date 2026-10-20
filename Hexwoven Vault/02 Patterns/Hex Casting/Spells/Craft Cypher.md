---
aliases: ["Craft Cypher"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hex Casting"
op_id: "hexcasting:craft/cypher"
signature: "waqqqqq"
start_direction: "EAST"
input: "entity, list"
output: ""
cost: "Costs about one Charged Amethyst."
iotas_in: ["[[Entity]]", "[[List]]"]
iotas_out: []
entry: "[[Crafting Casting Items]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Craft Cypher

✦ **Spell** · [[Hex Casting]] · 📖 [[Crafting Casting Items]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:2.2s" points="35.0,78.3 85.0,78.3 135.0,78.3 110.0,35.0 60.0,35.0 35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="51.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,78.3 51.0,83.8 51.0,72.8" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waqqqqq` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, list` ([[Entity]], [[List]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Charged Amethyst.
- **ID:** `hexcasting:craft/cypher`

```text
before   … │ entity │ list  ◂ top
after    …                  ◂ top
```

## Description

Costs about one [[Amethyst|Charged Amethyst]].

## Example

1. Put the hex you want to store on the [[The Stack|stack]] as a list ([[Introspection]] … [[Retrospection]])
2. Drop amethyst on the ground; it becomes the battery: [[Entity Purification Item]] on its position
3. Hold an empty cypher in your other hand → [[Jester's Gambit]] if needed so the stack is `entity, list` → **Craft Cypher**

## Related patterns (same book page)

- [[Craft Trinket]] `entity, list → —`
- [[Craft Artifact]] `entity, list → —`
- [[Recharge Item]] `entity → —`
- [[Erase Item]] `— → —`

## Sources

- 📖 **Guidebook:** [[Crafting Casting Items]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Craft Cypher](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/hexcasting@hexcasting:craft/cypher)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpMakePackagedSpell` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:craft/cypher`
