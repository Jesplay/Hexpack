---
aliases: ["Exchange Mind"]
tags: [hexcasting/spell, mod/slate-works, family/spell, topic/circles, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Spell Circles, Loci & Engines", "Items & Crafting"]
mod: "Slate Works"
op_id: "slate_work:exchange_mind"
signature: "eaqwqaeqawawaddwwdqeeqdwwd"
start_direction: "SOUTH_WEST"
input: "vector, entity"
output: ""
cost: "Costs 1 Charged Amethyst to reapply the knowledge of Working."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Merchant Directrix Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Exchange Mind

✦ **Spell** · [[Slate Works]] · 📖 [[Merchant Directrix Patterns]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/><circle cx="22.0" cy="103.8" r="2"/><circle cx="53.5" cy="103.8" r="2"/><circle cx="85.0" cy="103.8" r="2"/><circle cx="116.5" cy="103.8" r="2"/><circle cx="148.0" cy="103.8" r="2"/><circle cx="6.3" cy="131.1" r="2"/><circle cx="37.8" cy="131.1" r="2"/><circle cx="69.3" cy="131.1" r="2"/><circle cx="100.7" cy="131.1" r="2"/><circle cx="132.2" cy="131.1" r="2"/><circle cx="163.7" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,49.3 69.3,76.6 37.8,76.6 53.5,103.8 85.0,103.8 116.5,103.8 132.2,76.6 100.7,76.6 85.0,49.3 53.5,49.3 69.3,76.6 85.0,103.8 100.7,76.6 116.5,49.3 85.0,49.3 100.7,22.0 116.5,49.3 132.2,76.6 148.0,103.8 116.5,103.8 100.7,131.1 69.3,131.1 53.5,103.8 22.0,103.8 37.8,76.6 53.5,49.3 69.3,22.0 85.0,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:850;--dur:6.0s" points="85.0,49.3 69.3,76.6 37.8,76.6 53.5,103.8 85.0,103.8 116.5,103.8 132.2,76.6 100.7,76.6 85.0,49.3 53.5,49.3 69.3,76.6 85.0,103.8 100.7,76.6 116.5,49.3 85.0,49.3 100.7,22.0 116.5,49.3 132.2,76.6 148.0,103.8 116.5,103.8 100.7,131.1 69.3,131.1 53.5,103.8 22.0,103.8 37.8,76.6 53.5,49.3 69.3,22.0 85.0,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="49.3" x2="80.0" y2="58.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,64.3 77.0,56.3 83.0,59.8" fill="#fc77be"/><circle cx="85.0" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaqwqaeqawawaddwwdqeeqdwwd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, entity` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 1 Charged Amethyst to reapply the knowledge of Working.
- **ID:** `slate_work:exchange_mind`

```text
before   … │ vector │ entity  ◂ top
after    …                    ◂ top
```

## Description

This feels... [[Flay Mind (Book Entry)|familiar]]. This spell takes a Villager, and a [[Merchant Directrix]] and exchanges their Jobs. Costs 1 [[Amethyst|Charged Amethyst]] to reapply the knowledge of Working.

## Related patterns (same book page)

- [[Merchant's Purification]] `vec → complicated`
- [[Induce Restock]] `vector → —`

## Sources

- 📖 **Guidebook:** [[Merchant Directrix Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpExchangeMind` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `slate_work:exchange_mind`
