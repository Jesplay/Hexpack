---
aliases: ["Conjure Speck"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:conjure_speck"
signature: "ade"
start_direction: "SOUTH_WEST"
input: "any, vector, vector"
output: "entity"
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]", "[[Any Iota]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Specks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Speck

✦ **Spell** · [[Hexical]] · 📖 [[Specks]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="157" height="170" viewBox="0 0 157.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="38.0" cy="38.0" r="2"/><circle cx="92.3" cy="38.0" r="2"/><circle cx="146.5" cy="38.0" r="2"/><circle cx="10.9" cy="85.0" r="2"/><circle cx="65.1" cy="85.0" r="2"/><circle cx="119.4" cy="85.0" r="2"/><circle cx="38.0" cy="132.0" r="2"/><circle cx="92.3" cy="132.0" r="2"/><circle cx="146.5" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,38.0 65.1,85.0 119.4,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="92.3,38.0 65.1,85.0 119.4,85.0 92.3,132.0 38.0,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="38.0" x2="83.6" y2="53.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="77.3,63.8 78.4,50.1 88.8,56.0" fill="#fc77be"/><circle cx="92.3" cy="38.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ade` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `any, vector, vector` ([[Vector]], [[Any Iota]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexical:conjure_speck`

```text
before   … │ any │ vector │ vector  ◂ top
after    … │ entity                 ◂ top
```

## Description

Conjures a speck with the [[Iotas|iota]], position, and rotation. Costs a negligible amount of **[[Media|media]]**. Pushes the speck iota to the [[The Stack|stack]].

## Related patterns (same book page)

- [[Move Speck]] `speck entity, vector → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Alter Speck]] `speck entity, any → —`
- [[Time Speck]] `speck entity, number → —`
- [[Resize Speck]] `speck entity, number → —`
- [[Thicken Speck]] `speck entity, number → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

## Sources

- 📖 **Guidebook:** [[Specks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Conjure Speck](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/specks@hexical:conjure_speck)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpConjureSpeck` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:conjure_speck`
