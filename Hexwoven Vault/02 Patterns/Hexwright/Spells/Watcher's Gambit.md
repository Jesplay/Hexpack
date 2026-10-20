---
aliases: ["Watcher's Gambit"]
tags: [hexcasting/spell, mod/hexwright, family/gambit, topic/hexwright, topic/devices]
kind: "Spell"
family: "Gambit"
topics: ["Hex Wright Magic", "Casting Devices"]
mod: "Hexwright"
op_id: "hexwright:wardens_attunement"
signature: "dqaqd"
start_direction: "EAST"
input: "vec, entity/item type"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Item Type]]"]
iotas_out: []
entry: "[[Warding Box]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Watcher's Gambit

✦ **Spell** · [[Hexwright]] · 📖 [[Warding Box]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ♞ Gambit
> This pattern **rearranges the stack** or **controls how the hex runs**: its name marks it as a **Gambit**. See [[Pattern Naming Guide]] · [[Patterns by Family#Gambit|all Gambit patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,35.0 85.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 85.0,35.0 135.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="35.0,35.0 85.0,35.0 60.0,78.3 85.0,121.6 110.0,78.3 85.0,35.0 135.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="35.0" x2="51.0" y2="35.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.5,35.0 51.0,40.5 51.0,29.5" fill="#fc77be"/><circle cx="35.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqaqd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, entity/item type` ([[Vector]], [[Entity]], [[Item Type]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:wardens_attunement`

```text
before   … │ vec │ entity/item type  ◂ top
after    …                           ◂ top
```

## Description

Sets the Warding Box trigger to a specific entitytype or itemtype. Manual trigger setting resets this.
Costs 1/10 Dust

## Related patterns (same book page)

- [[Warden's Reflection]] `vec → [vec,vec]`
- [[Warden's Gambit]] `[vec,vec] → —`
- [[Redirect]] `entity, vec → —`

## Sources

- 📖 **Guidebook:** [[Warding Box]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `WardingBoxActions$1` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:wardens_attunement`
