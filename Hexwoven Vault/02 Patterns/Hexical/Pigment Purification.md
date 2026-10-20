---
aliases: ["Pigment Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/visuals]
kind: "Pattern"
family: "Purification"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:to_pigment"
signature: "aqwedeweeeewweeew"
start_direction: "NORTH_WEST"
input: "dye/entity"
output: "pigment"
cost: ""
iotas_in: ["[[Entity]]", "[[Pigment]]"]
iotas_out: ["[[Pigment]]"]
entry: "[[Pigment Manipulation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Pigment Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Pigment Manipulation]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="100.7,49.3 85.0,22.0 69.3,49.3 85.0,76.6 100.7,103.8 85.0,131.1 69.3,103.8 85.0,76.6 100.7,49.3 132.2,49.3 148.0,76.6 132.2,103.8 100.7,103.8 69.3,103.8 37.8,103.8 22.0,76.6 37.8,49.3 69.3,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="100.7,49.3 85.0,22.0 69.3,49.3 85.0,76.6 100.7,103.8 85.0,131.1 69.3,103.8 85.0,76.6 100.7,49.3 132.2,49.3 148.0,76.6 132.2,103.8 100.7,103.8 69.3,103.8 37.8,103.8 22.0,76.6 37.8,49.3 69.3,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="100.7" y1="49.3" x2="95.7" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="92.1,34.3 98.7,38.8 92.7,42.3" fill="#fc77be"/><circle cx="100.7" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqwedeweeeewweeew` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `dye/entity` ([[Entity]], [[Pigment]])
- **Leaves:** `pigment` ([[Pigment]])
- **ID:** `hexical:to_pigment`

```text
before   … │ dye/entity  ◂ top
after    … │ pigment     ◂ top
```

## Description

Takes a colored dye [[Iotas|iota]] and transforms it into a [[Pigments|pigment]], takes a player and returns their pigment, or takes an item containing a pigment and returns its pigment.

## Related patterns (same book page)

- [[Pigment Exaltation]] `pigment, vec, num → vec`
- [[Internalize Pigment II]] `pigment → —`

## Sources

- 📖 **Guidebook:** [[Pigment Manipulation]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Pigment Purification](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/pigments@hexical:to_pigment)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpToPigment` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:to_pigment`
