---
aliases: ["Apothecary's Purification II"]
tags: [hexcasting/pattern, mod/ephemera, family/purification, topic/effects, topic/data-structures]
kind: "Pattern"
family: "Purification"
topics: ["Status Effects", "Sets, Maps & Special Iotas"]
mod: "Ephemera"
op_id: "ephemera:getstatuscategory"
signature: "eeeeeqawa"
start_direction: "SOUTH_EAST"
input: "status"
output: "num"
cost: ""
iotas_in: ["[[Status]]"]
iotas_out: ["[[Number]]"]
entry: "[[Status Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Apothecary's Purification II

◇ **Pattern** · [[Ephemera]] · 📖 [[Status Iotas]] · Patterns

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 29.8,103.4 51.0,66.6 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="66.6" x2="100.3" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="105.2,86.8 96.3,80.7 104.4,76.0" fill="#fc77be"/><circle cx="93.5" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eeeeeqawa` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `status` ([[Status]])
- **Leaves:** `num` ([[Number]])
- **ID:** `ephemera:getstatuscategory`

```text
before   … │ status  ◂ top
after    … │ num     ◂ top
```

## Description

Accepts a status effect type, and returns a number based on whether the effect is likely to be helpful, harmful, or neutral to a creature.

## Related patterns (same book page)

- [[Apothecary's Purification]] `entity → [status]`
- [[Pharmacist's Distillation]] `entity, status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Entity Distillation Status]] `status, vector → entity | NULL`
- [[Zone Exaltation Status]] `status, vec, num → [entity]`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Ablation Purification]] `entity → num`
- [[Cleanse Status]] `entity, status → —`

## Sources

- 📖 **Guidebook:** [[Status Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Apothecary's Purification II](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/status_iotas@ephemera:getstatuscategory)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEffectCategory` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getstatuscategory`
