---
aliases: ["Stealth Shroud"]
tags: [hexcasting/great-spell, mod/oneironaut, family/great-spell, topic/effects, hexcasting/needs-enlightenment]
kind: "Great Spell"
family: "Great Spell"
topics: ["Status Effects"]
mod: "Oneironaut"
op_id: "oneironaut:resistdetection"
signature: "wawwwdwdwwaqqqqqe"
start_direction: "EAST"
input: "entity, number"
output: ""
cost: "Costs two amethyst dust per second."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Nondetection]]"
needs_enlightenment: true
per_world: true
kind_source: "tag"
topic_source: "page"
in_book: true
---

# Stealth Shroud

⭐ **Great Spell** · [[Oneironaut]] · 📖 [[Nondetection]] · Patterns › Great Spells

**Topics:** [[Status Effects (Topic)|Status Effects]]

**Unlock:** 🔒 needs **enlightenment** ([[Progression - Enlightenment & Great Spells|how]]) · 🎲 strokes **differ per world**: find the scroll

> [!abstract] ⭐ Great Spell
> This Great Spell is **world-altering**, has **per-world strokes** and usually needs **enlightenment**. See [[Pattern Naming Guide]] · [[Patterns by Family#Great Spell|all Great Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="24.5,145.5 59.4,145.5 94.4,145.5 76.9,115.3 59.4,85.0 41.9,54.7 24.5,24.5 59.4,24.5 94.4,24.5 76.9,54.7 59.4,85.0 41.9,115.3 76.9,115.3 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="24.5,145.5 59.4,145.5 94.4,145.5 76.9,115.3 59.4,85.0 41.9,54.7 24.5,24.5 59.4,24.5 94.4,24.5 76.9,54.7 59.4,85.0 41.9,115.3 76.9,115.3 94.4,85.0 76.9,54.7 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="145.5" x2="35.6" y2="145.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.7,145.5 35.6,149.4 35.6,141.7" fill="#fc77be"/><circle cx="24.5" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawwwdwdwwaqqqqqe` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

> [!warning] Per-world pattern
> This pattern's strokes are **randomised for every world**. The shape above is only the default. Find the real one on scrolls in loot chests (see [[Progression - Enlightenment & Great Spells]]).

## Stack effect

- **Takes:** `entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs two amethyst dust per second.
- **ID:** `oneironaut:resistdetection`

```text
before   … │ entity │ number  ◂ top
after    …                    ◂ top
```

## Description

Accepts a living entity and a number (*n*), and shields the entity from hex-based detection for *n* seconds. Costs two amethyst dust per second.

Also drains [[Media|media]] from the entity at a rate of 1/10 of a dust per second, as the shielding effect collects it to produce the chaff.

I should be careful with what I apply this to, as creatures with no loose media of their own will have their very minds fractured in order to provide this media.

While this is very effective at blocking effects such as [[Scout's Distillation]] and the various forms of [[Zone Distillation Animal|Zone Distillation]], it has no effect on [[Entity Purification]] variants, as those already have a fairly precise idea of where you are. What's more, the chaff itself produces an energy signautre, which, while dim, is detectable by a specialized pattern.

## Related patterns (same book page)

- [[Discern Shroud]] `vector, number → [vector]`

## Sources

- 📖 **Guidebook:** [[Nondetection]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Stealth Shroud](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/spells/great/detection_shielding@oneironaut:resistdetection)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Great Spell):** the mod tags it as a per-world (Great Spell) pattern
- 🔧 **Pattern ID:** `oneironaut:resistdetection`
