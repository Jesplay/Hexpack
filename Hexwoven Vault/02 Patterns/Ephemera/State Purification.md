---
aliases: ["State Purification"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/sensing, topic/blocks]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts", "World: Blocks, Terrain & Weather"]
mod: "Ephemera"
op_id: "ephemera:getproperties"
signature: "qaqqqqqwwdwewdw"
start_direction: "EAST"
input: "vec"
output: "[string]"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[List]]", "[[String]]"]
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# State Purification

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="169" viewBox="0 0 170.0 168.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/><circle cx="6.9" cy="144.5" r="2"/><circle cx="41.6" cy="144.5" r="2"/><circle cx="76.3" cy="144.5" r="2"/><circle cx="111.0" cy="144.5" r="2"/><circle cx="145.7" cy="144.5" r="2"/></g><polyline class="hexpat-base" points="24.3,114.4 59.0,114.4 76.3,84.4 41.6,84.4 24.3,114.4 41.6,144.5 76.3,144.5 93.7,114.4 76.3,84.4 59.0,54.3 41.6,24.3 76.3,24.3 111.0,24.3 128.4,54.3 145.7,84.4 111.0,84.4 76.3,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:555;--dur:4.5s" points="24.3,114.4 59.0,114.4 76.3,84.4 41.6,84.4 24.3,114.4 41.6,144.5 76.3,144.5 93.7,114.4 76.3,84.4 59.0,54.3 41.6,24.3 76.3,24.3 111.0,24.3 128.4,54.3 145.7,84.4 111.0,84.4 76.3,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="114.4" x2="35.4" y2="114.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,114.4 35.4,118.2 35.4,110.6" fill="#fc77be"/><circle cx="24.3" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqwwdwewdw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `[string]` ([[List]], [[String]])
- **ID:** `ephemera:getproperties`

```text
before   … │ vec       ◂ top
after    … │ [string]  ◂ top
```

## Description

Accepts a vector, and returns a list of strings corresponding to values contained in the state of the block at the corresponding position.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Entropy Reflection II]] `— → num`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Reflection II]] `— → num`
- [[Flock's Purification]] `— → num`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Ride Wisp]] `wisp → —`
- [[Dismount]] `— → —`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [State Purification](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:getproperties)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetProperties` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getproperties`
