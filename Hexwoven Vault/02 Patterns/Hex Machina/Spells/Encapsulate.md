---
aliases: ["Encapsulate"]
tags: [hexcasting/spell, mod/hex-machina, family/spell, topic/creatures, topic/devices]
kind: "Spell"
family: "Spell"
topics: ["Creatures & Entities", "Casting Devices"]
mod: "Hex Machina"
op_id: "hex_machina:contain"
signature: "qqqqqeawa"
start_direction: "WEST"
input: "entity"
output: ""
cost: "costs 1 Amethyst Dust."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Entity Capsule (Hex Machina)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Encapsulate

✦ **Spell** · [[Hex Machina]] · 📖 [[Entity Capsule (Hex Machina)]] · Patterns

**Topics:** [[Creatures & Entities (Topic)|Creatures & Entities]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,66.6 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="93.5,66.6 51.0,66.6 29.8,103.4 51.0,140.2 93.5,140.2 114.8,103.4 93.5,66.6 114.8,29.8 72.3,29.8 29.8,29.8 51.0,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="66.6" x2="79.9" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,66.6 79.9,61.9 79.9,71.3" fill="#fc77be"/><circle cx="93.5" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqqqeawa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** costs 1 Amethyst Dust.
- **ID:** `hex_machina:contain`

```text
before   … │ entity  ◂ top
after    …           ◂ top
```

## Description

Reads a entity from the [[The Stack|stack]] and stores that entity in a entity capsule, costs 1 [[Amethyst|Amethyst Dust.]] per health point of the target entity

## Related patterns (same book page)

- [[Deploy]] `vec → —`
- [[Ketchum's Reflection]] `— → bool`

## Sources

- 📖 **Guidebook:** [[Entity Capsule (Hex Machina)]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hex Machina on Modrinth](https://modrinth.com/mod/uQ6K0yH1)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Spell):** from the mod code: its action class `ContainAction` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hex_machina:contain`
