---
aliases: ["Schrödinger's Reflection"]
tags: [hexcasting/spell, mod/hexcellular, family/reflection, topic/data-structures]
kind: "Spell"
family: "Reflection"
topics: ["Sets, Maps & Special Iotas"]
mod: "Hexcellular"
op_id: "hexcellular:create_property"
signature: "aawe"
start_direction: "SOUTH_WEST"
input: ""
output: "property"
cost: "Costs about five Charged Amethyst to etch this law into the universe."
iotas_in: []
iotas_out: ["[[Property]]"]
entry: "[[Property Iota]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Schrödinger's Reflection

✦ **Spell** · [[Hexcellular]] · 📖 [[Property Iota]] · Patterns

**Topics:** [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,103.4 29.8,140.2 72.3,140.2 51.0,103.4 29.8,66.6 51.0,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="51.0,103.4 29.8,140.2 72.3,140.2 51.0,103.4 29.8,66.6 51.0,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="103.4" x2="44.2" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,123.7 40.2,112.9 48.3,117.5" fill="#fc77be"/><circle cx="51.0" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aawe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `property` ([[Property]])
- **Cost:** Costs about five Charged Amethyst to etch this law into the universe.
- **ID:** `hexcellular:create_property`

```text
before   …             ◂ top
after    … │ property  ◂ top
```

## Description

Creates a property [[Iotas|iota]] for a random **Property**. Costs about five [[Amethyst|Charged Amethyst]] to etch this law into the universe.

When using **Properties**, the created property iota is the *only* reference to the **Property**. I should take care to immediately save the iota lest I lose the only interface to it.

While the cost to create one is steep, once created, the property iota allows for simple connection to the **Property**, allowing me to read and write to it cheaply forevermore.

## Related patterns (same book page)

- [[Observation Purification]] `property → any`
- [[Schrödinger's Gambit]] `property, any → —`
- [[Schrödinger's Purification]] `property → property`

## Sources

- 📖 **Guidebook:** [[Property Iota]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Schrödinger's Reflection](https://hexcellular.hexxy.media/v/1.1.1/1.0.0/en_us/#properties@hexcellular:create_property)
- 🧩 **Mod page:** [Hexcellular on Modrinth](https://modrinth.com/mod/mN6t9Bt4)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpCreateProperty` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcellular:create_property`
