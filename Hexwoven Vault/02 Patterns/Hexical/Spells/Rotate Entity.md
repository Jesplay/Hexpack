---
aliases: ["Rotate Entity"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hexical"
op_id: "hexical:rotate_entity"
signature: "qqqdaqqqa"
start_direction: "EAST"
input: "entity, number, number"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Rotation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Rotate Entity

✦ **Spell** · [[Hexical]] · 📖 [[Rotation]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,140.2 114.8,140.2 136.1,103.4 114.8,66.6 72.3,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="72.3,140.2 114.8,140.2 136.1,103.4 114.8,66.6 72.3,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 72.3,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="140.2" x2="85.9" y2="140.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.7,140.2 85.9,144.9 85.9,135.6" fill="#fc77be"/><circle cx="72.3" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqdaqqqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:rotate_entity`

```text
before   … │ entity │ number │ number  ◂ top
after    …                             ◂ top
```

## Description

Rotates an entity by a given change in pitch and yaw, expressed in radians.

Nature seems to be against entity rotation. Rather than a straightforward vector, Nature demands measurements in radians by which to turn horizontally and vertically.

The cost is strange and multilayered too. Free if cast on myself, about half of an [[Amethyst|Amethyst Dust]] for other entities, and the natural aura of **[[Media|media]]** surrounding a player elevates the cost to about a whole [[Amethyst|Charged Amethyst]].

## Related patterns (same book page)

- [[Rotate Block]] `vector, vector → —`

## Sources

- 📖 **Guidebook:** [[Rotation]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpRotateEntity` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:rotate_entity`
