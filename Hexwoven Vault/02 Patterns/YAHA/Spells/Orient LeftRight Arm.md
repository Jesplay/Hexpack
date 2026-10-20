---
aliases: ["Orient Left/Right Arm"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/creatures]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities"]
mod: "YAHA"
op_id: "yaha:stand_rotate_middle"
signature: "addweaqada"
start_direction: "EAST"
input: "entity, vector"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Armor Stand Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "guess"
topic_source: "page"
in_book: true
---

# Orient Left/Right Arm

✦ **Spell** · [[YAHA]] · 📖 [[Armor Stand Manipulation]] · Patterns › Spells

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,66.6 93.5,66.6 72.3,29.8 114.8,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,140.2 93.5,140.2 72.3,103.4 114.8,103.4 93.5,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="51.0,66.6 93.5,66.6 72.3,29.8 114.8,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,140.2 93.5,140.2 72.3,103.4 114.8,103.4 93.5,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="66.6" x2="64.6" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="74.4,66.6 64.6,71.3 64.6,61.9" fill="#fc77be"/><circle cx="51.0" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `addweaqada` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `yaha:stand_rotate_middle`

```text
before   … │ entity │ vector  ◂ top
after    …                    ◂ top
```

## Description

The pattern with the left protrusion orients the armor stand's left arm; the pattern with the right protrusion orients the armor stand's right arm.

## Related patterns (same book page)

- [[ShowHide Arms]] `entity → —`
- [[ShowHide Baseplate]] `entity → —`
- [[ShrinkGrow Armor Stand]] `entity → —`
- [[Rotate Armor Stand]] `entity, number → —`
- [[Orient HeadBody]] `entity, vector → —`
- [[Orient LeftRight Leg]] `entity, vector → —`

## Sources

- 📖 **Guidebook:** [[Armor Stand Manipulation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** from the book page
- 🏷️ **Type (Spell):** inferred from the book text (the mod's code couldn't be matched to this pattern)
- 🔧 **Pattern ID:** `yaha:stand_rotate_middle`
