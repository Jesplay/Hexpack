---
aliases: ["Locator's Reflection"]
tags: [hexcasting/pattern, mod/lapisworks, family/reflection, topic/sentinels]
kind: "Pattern"
family: "Reflection"
topics: ["Sentinels"]
mod: "Lapisworks"
op_id: "lapisworks:find_my_enchsent"
signature: "wdwwwdwqwdwwwdweeqqqq"
start_direction: "NORTH_WEST"
input: ""
output: "vec|null"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]", "[[Null]]"]
entry: "[[Enchanted Sentinel]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Locator's Reflection

◇ **Pattern** · [[Lapisworks]] · 📖 [[Enchanted Sentinel]] · Lapisworks › The Mage's Spells

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="53.5,76.6 37.8,49.3 22.0,22.0 53.5,22.0 85.0,22.0 116.5,22.0 148.0,22.0 132.2,49.3 116.5,76.6 132.2,103.8 148.0,131.1 116.5,131.1 85.0,131.1 53.5,131.1 22.0,131.1 37.8,103.8 53.5,76.6 85.0,76.6 100.7,103.8 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:693;--dur:6.0s" points="53.5,76.6 37.8,49.3 22.0,22.0 53.5,22.0 85.0,22.0 116.5,22.0 148.0,22.0 132.2,49.3 116.5,76.6 132.2,103.8 148.0,131.1 116.5,131.1 85.0,131.1 53.5,131.1 22.0,131.1 37.8,103.8 53.5,76.6 85.0,76.6 100.7,103.8 132.2,103.8 148.0,76.6 132.2,49.3 100.7,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="76.6" x2="48.5" y2="67.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,61.6 51.5,66.1 45.5,69.6" fill="#fc77be"/><circle cx="53.5" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwwwdwqwdwwwdweeqqqq` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vec|null` ([[Vector]], [[Null]])
- **ID:** `lapisworks:find_my_enchsent`

## Description

Returns the location of my Enchanted Sentinel or null if I do not have one summoned.

## Related patterns (same book page)

- [[Summon Enchanted Sentinel]] `vec, num → —`
- [[Banish Own Enchanted Sentinel]] `— → —`
- [[Banish Other's Enchanted Sentinel]] `vec → —`

## Sources

- 📖 **Guidebook:** [[Enchanted Sentinel]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `LocateMySent` is not a `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:find_my_enchsent`
