---
aliases: ["Equestrian Purification"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Ephemera"
op_id: "ephemera:getrider"
signature: "eqqaqqwaaw"
start_direction: "NORTH_EAST"
input: "entity"
output: "[entity]"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Entity]]", "[[List]]"]
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Equestrian Purification

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="24.3,114.4 41.6,84.4 76.3,84.4 93.7,54.3 76.3,24.3 59.0,54.3 76.3,84.4 111.0,84.4 145.7,84.4 128.4,54.3 111.0,84.4 93.7,114.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:382;--dur:3.1s" points="24.3,114.4 41.6,84.4 76.3,84.4 93.7,54.3 76.3,24.3 59.0,54.3 76.3,84.4 111.0,84.4 145.7,84.4 128.4,54.3 111.0,84.4 93.7,114.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="114.4" x2="29.8" y2="104.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="33.8,97.9 33.1,106.7 26.5,102.9" fill="#fc77be"/><circle cx="24.3" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqaqqwaaw` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `[entity]` ([[Entity]], [[List]])
- **ID:** `ephemera:getrider`

```text
before   … │ entity    ◂ top
after    … │ [entity]  ◂ top
```

## Description

Accepts an entity, and returns a list of entities seated atop it.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Entropy Reflection II]] `— → num`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Reflection II]] `— → num`
- [[Flock's Purification]] `— → num`
- [[State Purification]] `vec → [string]`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Ride Wisp]] `wisp → —`
- [[Dismount]] `— → —`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Equestrian Purification](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:getrider)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetRider` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getrider`
