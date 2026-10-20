---
aliases: ["Zone Exaltation: Status"]
tags: [hexcasting/pattern, mod/ephemera, family/exaltation, topic/effects, topic/data-structures]
kind: "Pattern"
family: "Exaltation"
topics: ["Status Effects", "Sets, Maps & Special Iotas"]
mod: "Ephemera"
op_id: "ephemera:getbystatus"
signature: "ewqqqqqwe"
start_direction: "EAST"
input: "status, vec, num"
output: "[entity]"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Status]]"]
iotas_out: ["[[Entity]]", "[[List]]"]
entry: "[[Status Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Zone Exaltation: Status

◇ **Pattern** · [[Ephemera]] · 📖 [[Status Iotas]] · Patterns

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◆ Exaltation
> This pattern takes **three or more** iotas and gives back **one**: its name marks it as a **Exaltation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Exaltation|all Exaltation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,24.3 59.0,24.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 59.0,84.4 24.3,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="24.3,24.3 59.0,24.3 76.3,54.3 93.7,84.4 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 76.3,54.3 59.0,84.4 24.3,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="24.3" x2="35.4" y2="24.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,24.3 35.4,28.1 35.4,20.5" fill="#fc77be"/><circle cx="24.3" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ewqqqqqwe` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `status, vec, num` ([[Number]], [[Vector]], [[Status]])
- **Leaves:** `[entity]` ([[Entity]], [[List]])
- **ID:** `ephemera:getbystatus`

```text
before   … │ status │ vec │ num  ◂ top
after    … │ [entity]            ◂ top
```

## Description

Accepts a status effect type, a vector, and a number (*n*), and returns a list of entities within radius *n* of the vector which have that effect.

## Related patterns (same book page)

- [[Apothecary's Purification]] `entity → [status]`
- [[Apothecary's Purification II]] `status → num`
- [[Pharmacist's Distillation]] `entity, status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Entity Distillation Status]] `status, vector → entity | NULL`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Ablation Purification]] `entity → num`
- [[Cleanse Status]] `entity, status → —`

## Sources

- 📖 **Guidebook:** [[Status Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Zone Exaltation: Status](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/status_iotas@ephemera:getbystatus)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntitiesByStatus` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getbystatus`
