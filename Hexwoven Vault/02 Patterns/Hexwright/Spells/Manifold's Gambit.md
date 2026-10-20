---
aliases: ["Manifold's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright, topic/storage]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic", "Reading & Writing"]
mod: "Hexwright"
op_id: "hexwright:reliquarys_offering"
signature: "weeeewqaa"
start_direction: "WEST"
input: "item entity, num"
output: ""
cost: ""
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[The Vault Seal]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifold's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[The Vault Seal]] · Hex Wright › Storage

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="111.0,84.4 76.3,84.4 41.6,84.4 24.3,54.3 41.6,24.3 76.3,24.3 93.7,54.3 111.0,84.4 145.7,84.4 128.4,54.3 111.0,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="111.0,84.4 76.3,84.4 41.6,84.4 24.3,54.3 41.6,24.3 76.3,24.3 93.7,54.3 111.0,84.4 145.7,84.4 128.4,54.3 111.0,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.0" y1="84.4" x2="99.9" y2="84.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="91.9,84.4 99.9,80.6 99.9,88.2" fill="#fc77be"/><circle cx="111.0" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeewqaa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item entity, num` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:reliquarys_offering`

```text
before   … │ item entity │ num  ◂ top
after    …                      ◂ top
```

## Description

Put the item entity in the Manifold Vault at the slot given. Player cast displaced items go to inventory.
Costs 1/10 Dust

## Related patterns (same book page)

- [[Manifold's Reflection]] `— → [itemstack]`
- [[Manifold's Distillation]] `stack, num → item`

## Sources

- 📖 **Guidebook:** [[The Vault Seal]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `HeldSealActions$2` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:reliquarys_offering`
