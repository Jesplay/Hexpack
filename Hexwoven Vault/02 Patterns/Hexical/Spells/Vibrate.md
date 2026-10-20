---
aliases: ["Vibrate"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:vibration"
signature: "wwawawwd"
start_direction: "EAST"
input: "vector, entity/vector, number"
output: ""
cost: "Costs about a sixteenth of one Amethyst Dust."
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Hexical Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vibrate

✦ **Spell** · [[Hexical]] · 📖 [[Hexical Spells]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="27.0,94.0 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 85.0,60.5 65.7,94.0 46.4,127.4 27.0,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="27.0,94.0 65.7,94.0 104.3,94.0 143.0,94.0 123.6,60.5 104.3,27.0 85.0,60.5 65.7,94.0 46.4,127.4 27.0,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="94.0" x2="39.4" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,94.0 39.4,98.2 39.4,89.7" fill="#fc77be"/><circle cx="27.0" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwawawwd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, entity/vector, number` ([[Number]], [[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a sixteenth of one Amethyst Dust.
- **ID:** `hexical:vibration`

```text
before   … │ vector │ entity/vector │ number  ◂ top
after    …                                    ◂ top
```

## Description

Causes a visible pulse of vibration from a vector to an entity or vector over a number of seconds. Costs about a sixteenth of one [[Amethyst|Amethyst Dust]].

The effect, while resembling the pulses produced by **Sculk Sensors**, is entirely distinct. It is a purely visual effect.

The duration can range anywhere from zero seconds to ten seconds.

*If a tree falls in a forest and no one is around to hear it, does it make a sound?*

## Related patterns (same book page)

- [[Confetti]] `vector, number/vector → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Crack Device]] `— → —`
- [[Conjure Flower]] `vector, identifer → —`
- [[Illuminate]] `vector, number → —`
- [[Gasp]] `entity → —`
- [[Squawk]] `vector, identifier → —`

## Sources

- 📖 **Guidebook:** [[Hexical Spells]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpVibrate` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:vibration`
