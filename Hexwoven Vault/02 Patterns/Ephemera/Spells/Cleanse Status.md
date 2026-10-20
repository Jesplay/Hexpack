---
aliases: ["Cleanse Status"]
tags: [hexcasting/spell, mod/ephemera, family/spell, topic/effects, topic/data-structures]
kind: "Spell"
family: "Spell"
topics: ["Status Effects", "Sets, Maps & Special Iotas"]
mod: "Ephemera"
op_id: "ephemera:removestatus"
signature: "eeeeedaqdewed"
start_direction: "SOUTH_WEST"
input: "entity, status"
output: ""
cost: ""
iotas_in: ["[[Entity]]", "[[Status]]"]
iotas_out: []
entry: "[[Status Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Cleanse Status

✦ **Spell** · [[Ephemera]] · 📖 [[Status Iotas]] · Patterns

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/><circle cx="7.7" cy="127.4" r="2"/><circle cx="46.4" cy="127.4" r="2"/><circle cx="85.0" cy="127.4" r="2"/><circle cx="123.6" cy="127.4" r="2"/><circle cx="162.3" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="123.6,60.5 104.3,94.0 65.7,94.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5 85.0,60.5 104.3,94.0 143.0,94.0 123.6,127.4 85.0,127.4 46.4,127.4 27.0,94.0 65.7,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:541;--dur:3.9s" points="123.6,60.5 104.3,94.0 65.7,94.0 46.4,60.5 65.7,27.0 104.3,27.0 123.6,60.5 85.0,60.5 104.3,94.0 143.0,94.0 123.6,127.4 85.0,127.4 46.4,127.4 27.0,94.0 65.7,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="123.6" y1="60.5" x2="117.5" y2="71.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="113.0,78.9 113.8,69.1 121.1,73.3" fill="#fc77be"/><circle cx="123.6" cy="60.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeedaqdewed` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, status` ([[Entity]], [[Status]])
- **Leaves:** `nothing` (—)
- **ID:** `ephemera:removestatus`

```text
before   … │ entity │ status  ◂ top
after    …                    ◂ top
```

## Description

Accepts an entity and a status effect type, and removes that status effect from the entity. Cost is based on the effect's innate harmfulness, its potency, and its remaining duration.

Costs more if attempting to remove a positive effect from an entity other than myself.

All of the patterns which deal with a specific status effect on an entity have the side effect that if the entity does not currently have the specified effect, the pattern will fail and inflict nausea upon me.

## Related patterns (same book page)

- [[Apothecary's Purification]] `entity → [status]`
- [[Apothecary's Purification II]] `status → num`
- [[Pharmacist's Distillation]] `entity, status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Entity Distillation Status]] `status, vector → entity | NULL`
- [[Zone Exaltation Status]] `status, vec, num → [entity]`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Ablation Purification]] `entity → num`

## Sources

- 📖 **Guidebook:** [[Status Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Cleanse Status](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/status_iotas@ephemera:removestatus)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpRemoveStatus` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `ephemera:removestatus`
