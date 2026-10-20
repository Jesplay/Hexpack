---
aliases: ["Pharmacist's Distillation"]
tags: [hexcasting/pattern, mod/ephemera, family/distillation, topic/effects, topic/data-structures]
kind: "Pattern"
family: "Distillation"
topics: ["Status Effects", "Sets, Maps & Special Iotas"]
mod: "Ephemera"
op_id: "ephemera:getstatusduration"
signature: "qqqqqedwdwd"
start_direction: "SOUTH_WEST"
input: "entity, status"
output: "num"
cost: ""
iotas_in: ["[[Entity]]", "[[Status]]"]
iotas_out: ["[[Number]]"]
entry: "[[Status Iotas]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Pharmacist's Distillation

◇ **Pattern** · [[Ephemera]] · 📖 [[Status Iotas]] · Patterns

**Topics:** [[Status Effects (Topic)|Status Effects]] · [[Sets, Maps & Special Iotas (Topic)|Sets, Maps & Special Iotas]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 51.0,66.6 29.8,29.8 72.3,29.8 114.8,29.8 93.5,66.6 72.3,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 51.0,66.6 29.8,29.8 72.3,29.8 114.8,29.8 93.5,66.6 72.3,103.4 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="66.6" x2="44.2" y2="78.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,86.8 40.2,76.0 48.3,80.7" fill="#fc77be"/><circle cx="51.0" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqedwdwd` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, status` ([[Entity]], [[Status]])
- **Leaves:** `num` ([[Number]])
- **ID:** `ephemera:getstatusduration`

```text
before   … │ entity │ status  ◂ top
after    … │ num              ◂ top
```

## Description

Accepts an entity and a status effect type, and returns the effect on the target's duration in seconds.

## Related patterns (same book page)

- [[Apothecary's Purification]] `entity → [status]`
- [[Apothecary's Purification II]] `status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Entity Distillation Status]] `status, vector → entity | NULL`
- [[Zone Exaltation Status]] `status, vec, num → [entity]`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Ablation Purification]] `entity → num`
- [[Cleanse Status]] `entity, status → —`

## Sources

- 📖 **Guidebook:** [[Status Iotas]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Pharmacist's Distillation](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/status_iotas@ephemera:getstatusduration)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetStatusDetail` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:getstatusduration`
