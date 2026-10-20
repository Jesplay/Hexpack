---
aliases: ["Manifold's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright, topic/storage]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic", "Reading & Writing"]
mod: "Hexwright"
op_id: "hexwright:reliquarys_reflection"
signature: "weeeew"
start_direction: "WEST"
input: ""
output: "[itemstack]"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Item Stack]]"]
entry: "[[The Vault Seal]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Manifold's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[The Vault Seal]] · Hex Wright › Storage

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,106.0 95.9,106.0 52.3,106.0 30.5,68.3 52.3,30.5 95.9,30.5 117.7,68.3 139.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:305;--dur:2.0s" points="139.5,106.0 95.9,106.0 52.3,106.0 30.5,68.3 52.3,30.5 95.9,30.5 117.7,68.3 139.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="106.0" x2="125.5" y2="106.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="115.5,106.0 125.5,101.2 125.5,110.8" fill="#fc77be"/><circle cx="139.5" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `weeeew` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `[itemstack]` ([[List]], [[Item Stack]])
- **ID:** `hexwright:reliquarys_reflection`

```text
before   …                ◂ top
after    … │ [itemstack]  ◂ top
```

## Description

Returns a list of every slot in the Manifold Vault, empty ones included, as Stacks in slot order.

## Related patterns (same book page)

- [[Manifold's Gambit]] `item entity, num → —`
- [[Manifold's Distillation]] `stack, num → item`

## Sources

- 📖 **Guidebook:** [[The Vault Seal]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `HeldSealActions$1` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:reliquarys_reflection`
