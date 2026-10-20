---
aliases: ["Entity Distillation: Status"]
tags: [hexcasting/pattern, mod/ephemera, family/distillation, topic/effects, topic/data-structures]
kind: "Pattern"
family: "Distillation"
topics: ["Status Effects", "Sets, Maps & Special Iotas"]
mod: "Ephemera"
op_id: "ephemera:getbystatussingle"
signature: "eaeeeeeae"
start_direction: "EAST"
input: "status, vector"
output: "entity | NULL"
cost: ""
iotas_in: ["[[Vector]]", "[[Status]]"]
iotas_out: ["[[Entity]]", "[[Null]]"]
entry: "[[Status Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Entity Distillation: Status

◇ **Pattern** · [[Ephemera]] · 📖 [[Status Iotas]] · Patterns

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="24.3,24.3 59.0,24.3 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 59.0,84.4 24.3,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="24.3,24.3 59.0,24.3 76.3,54.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4 93.7,84.4 76.3,54.3 59.0,84.4 24.3,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.3" y1="24.3" x2="35.4" y2="24.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="43.4,24.3 35.4,28.1 35.4,20.5" fill="#fc77be"/><circle cx="24.3" cy="24.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eaeeeeeae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `status, vector` ([[Vector]], [[Status]])
- **Leaves:** `entity | NULL` ([[Entity]], [[Null]])
- **ID:** `ephemera:getbystatussingle`

## Description

Accepts a status effect type and a vector, and returns an entity at that position which has that effect.

## Related patterns (same book page)

- [[Apothecary's Purification]] `entity → [status]`
- [[Apothecary's Purification II]] `status → num`
- [[Pharmacist's Distillation]] `entity, status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Zone Exaltation Status]] `status, vec, num → [entity]`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Ablation Purification]] `entity → num`
- [[Cleanse Status]] `entity, status → —`

## Sources

- 📖 **Guidebook:** [[Status Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Entity Distillation: Status](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/status_iotas@ephemera:getbystatussingle)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEntityByStatus` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getbystatussingle`
