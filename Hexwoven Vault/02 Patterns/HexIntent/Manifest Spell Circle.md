---
aliases: ["Manifest Spell Circle"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "HexIntent"
op_id: "manifestation:spell_circle"
signature: "qqqqqeawqwqwqwqwqw"
start_direction: "SOUTH_WEST"
input: "[pattern], vec, vec, num, num?, vec?"
output: "none"
cost: "Cost: 1/500 dust."
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]", "[[Pattern]]"]
iotas_out: []
entry: "[[Visual Manifestation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifest Spell Circle

◇ **Pattern** · [[HexIntent]] · 📖 [[Visual Manifestation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 69.3,49.3 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:598;--dur:5.3s" points="69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 100.7,49.3 69.3,49.3 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 53.5,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="49.3" x2="64.2" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,64.3 61.2,56.3 67.2,59.8" fill="#fc77be"/><circle cx="69.3" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqeawqwqwqwqwqw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[pattern], vec, vec, num, num?, vec?` ([[Number]], [[Vector]], [[List]], [[Pattern]])
- **Leaves:** `none` (—)
- **Cost:** Cost: 1/500 dust.
- **ID:** `manifestation:spell_circle`

```text
before   … │ [pattern] │ vec │ vec │ num │ num? │ vec?  ◂ top
after    … │ none                                       ◂ top
```

## Description

Projects a solid [[Spell Circles|spell circle]] into the world displaying a pattern for a duration before disintegrating.
Cost: 1/500 dust.

Inputs:
patterns: non-empty list of patterns to embed around the ring (max 48).
origin: vector for circle center.
facing: vector used as the circle plane.

duration: lifetime in ticks (1..1200).

size?: optional number 1..6 (default 3).

## Related patterns (same book page)

- [[Equation Hex Cloud]] `vec|entity, equation, num → none`
- [[Hex Trail]] `vec, vec, vec, num, num, num? → none`

## Sources

- 📖 **Guidebook:** [[Visual Manifestation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSpellCircle` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:spell_circle`
