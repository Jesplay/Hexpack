---
aliases: ["Gate's Reflection"]
tags: [hexcasting/great-spell, mod/hexal, family/reflection, topic/movement, hexcasting/needs-enlightenment]
kind: "Great Spell"
family: "Reflection"
topics: ["Movement"]
mod: "Hexal"
op_id: "hexal:gate/make"
signature: "qwqwqwqwqwqqeaeaeaeaeae"
start_direction: "WEST"
input: "null | vec | vec, entity"
output: "gate"
cost: "Costs 32 Charged Amethyst."
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Null]]"]
iotas_out: ["[[Gate]]"]
entry: "[[Gates]]"
needs_enlightenment: true
per_world: true
kind_source: "tag"
topic_source: "page"
in_book: true
---

# Gate's Reflection

⭐ **Great Spell** · [[Hexal]] · 📖 [[Gates]] · Patterns › Great Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** 🔒 needs **enlightenment** ([[Progression - Enlightenment & Great Spells|how]]) · 🎲 strokes **differ per world**: find the scroll

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 69.3,49.3 37.8,49.3 53.5,76.6 37.8,103.8 69.3,103.8 85.0,131.1 100.7,103.8 132.2,103.8 116.5,76.6 132.2,49.3 100.7,49.3 85.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:756;--dur:6.0s" points="85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 69.3,49.3 37.8,49.3 53.5,76.6 37.8,103.8 69.3,103.8 85.0,131.1 100.7,103.8 132.2,103.8 116.5,76.6 132.2,49.3 100.7,49.3 85.0,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="22.0" x2="74.9" y2="22.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.7,22.0 74.9,18.6 74.9,25.5" fill="#fc77be"/><circle cx="85.0" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwqwqwqwqwqqeaeaeaeaeae` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

> [!warning] Per-world pattern
> This pattern's strokes are **randomised for every world**. The shape above is only the default. Find the real one on scrolls in loot chests (see [[Progression - Enlightenment & Great Spells]]).

## Stack effect

- **Takes:** `null | vec | vec, entity` ([[Vector]], [[Entity]], [[Null]])
- **Leaves:** `gate` ([[Gate]])
- **Cost:** Costs 32 Charged Amethyst.
- **ID:** `hexal:gate/make`

## Description

Adds a new gate [[Iotas|iota]] to the [[The Stack|stack]]. Any copy of this iota will point to the same numbered gate. Costs 32 [[Amethyst|Charged Amethyst]]. If passed null, makes a Drifting Gate. If passed a vector, makes a Location Anchored Gate, If passed a vector and an entity, makes an Entity Anchored Gate.

## Related patterns (same book page)

- [[Gate's Opening]] `gate, entity → —`
- [[Gate's Dismissal]] `gate, entity → —`
- [[Marked Distillation]] `gate, entity → bool`
- [[Gate's Closing]] `gate | gate, vec → —`

## Sources

- 📖 **Guidebook:** [[Gates]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Gate's Reflection](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/great_spells/gates@hexal:gate/make)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Great Spell):** the mod tags it as a per-world (Great Spell) pattern
- 🔧 **Pattern ID:** `hexal:gate/make`
