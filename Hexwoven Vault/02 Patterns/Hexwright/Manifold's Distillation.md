---
aliases: ["Manifold's Distillation"]
tags: [hexcasting/pattern, mod/hexwright, family/distillation, topic/hexwright, topic/storage]
kind: "Pattern"
family: "Distillation"
topics: ["Hex Wright Magic", "Reading & Writing"]
mod: "Hexwright"
op_id: "hexwright:reliquarys_reclamation"
signature: "weeeewqdd"
start_direction: "WEST"
input: "stack, num"
output: "item"
cost: "Costs 1 Dust When this pattern is used in a Sealed Satchel or Hexbound Chest withdraw hex, the item is placed at your feet and then under your cursor in one tick, fast enough that the item never appears on the floor for someone else to pick up."
iotas_in: ["[[Number]]"]
iotas_out: ["[[Item Stack]]"]
entry: "[[The Vault Seal]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifold's Distillation

◇ **Pattern** · [[Hexwright]] · 📖 [[The Vault Seal]] · Hex Wright › Storage

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/><circle cx="24.3" cy="114.4" r="2"/><circle cx="59.0" cy="114.4" r="2"/><circle cx="93.7" cy="114.4" r="2"/><circle cx="128.4" cy="114.4" r="2"/><circle cx="163.1" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="111.0,84.4 76.3,84.4 41.6,84.4 24.3,54.3 41.6,24.3 76.3,24.3 93.7,54.3 111.0,84.4 145.7,84.4 128.4,114.4 111.0,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="111.0,84.4 76.3,84.4 41.6,84.4 24.3,54.3 41.6,24.3 76.3,24.3 93.7,54.3 111.0,84.4 145.7,84.4 128.4,114.4 111.0,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.0" y1="84.4" x2="99.9" y2="84.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="91.9,84.4 99.9,80.6 99.9,88.2" fill="#fc77be"/><circle cx="111.0" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeewqdd` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `stack, num` ([[Number]])
- **Leaves:** `item` ([[Item Stack]])
- **Cost:** Costs 1 Dust When this pattern is used in a Sealed Satchel or Hexbound Chest withdraw hex, the item is placed at your feet and then under your cursor in one tick, fast enough that the item never appears on the floor for someone else to pick up.
- **ID:** `hexwright:reliquarys_reclamation`

```text
before   … │ stack │ num  ◂ top
after    … │ item         ◂ top
```

## Description

Withdraws item at your feet, returning that item entity. Player cast goes to inventory or feet if full. Costs 1 Dust

When this pattern is used in a [[Sealed Satchel]] or [[The Hexbound Chest|Hexbound Chest]] withdraw hex, the item is placed at your feet and then under your cursor in one tick, fast enough that the item never appears on the floor for someone else to pick up. It is placed on the floor so the entity can be captured.

## Related patterns (same book page)

- [[Manifold's Reflection]] `— → [itemstack]`
- [[Manifold's Gambit]] `item entity, num → —`

## Sources

- 📖 **Guidebook:** [[The Vault Seal]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `HeldSealActions$3` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:reliquarys_reclamation`
