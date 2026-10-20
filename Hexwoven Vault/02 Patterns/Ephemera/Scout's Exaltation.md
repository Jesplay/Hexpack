---
aliases: ["Scout's Exaltation"]
tags: [hexcasting/pattern, mod/ephemera, family/exaltation, topic/sensing]
kind: "Pattern"
family: "Exaltation"
topics: ["Sensing & Raycasts"]
mod: "Ephemera"
op_id: "ephemera:filteredentityraycast"
signature: "wqded"
start_direction: "EAST"
input: "[type], vec, vec"
output: "entity | NULL"
cost: "Costs an amount of media slightly less negligible than that of the aforementioned pattern."
iotas_in: ["[[Vector]]", "[[List]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Scout's Exaltation

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="27.0,60.5 65.7,60.5 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:232;--dur:1.7s" points="27.0,60.5 65.7,60.5 104.3,60.5 123.6,27.0 143.0,60.5 123.6,94.0 104.3,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="60.5" x2="39.4" y2="60.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.3,60.5 39.4,64.8 39.4,56.3" fill="#fc77be"/><circle cx="27.0" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqded` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[type], vec, vec` ([[Vector]], [[List]])
- **Leaves:** `entity | NULL` ([[Entity]], [[Null]])
- **Cost:** Costs an amount of media slightly less negligible than that of the aforementioned pattern.
- **ID:** `ephemera:filteredentityraycast`

## Description

As [[Scout's Distillation]], but also accepts a list of entity type [[Iotas|iotas]]. Any entity whose type is not present in the list will be ignored. Costs an amount of [[Media|media]] slightly less negligible than that of the aforementioned pattern.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Entropy Reflection II]] `— → num`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Reflection II]] `— → num`
- [[Flock's Purification]] `— → num`
- [[State Purification]] `vec → [string]`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Ride Wisp]] `wisp → —`
- [[Dismount]] `— → —`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Scout's Exaltation](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:filteredentityraycast)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpFilteredEntityRaycast` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:filteredentityraycast`
