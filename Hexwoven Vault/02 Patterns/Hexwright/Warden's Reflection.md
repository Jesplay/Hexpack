---
aliases: ["Warden's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic", "Casting Devices"]
mod: "Hexwright"
op_id: "hexwright:wardens_purification"
signature: "wawqwawa"
start_direction: "SOUTH_WEST"
input: "vec"
output: "[vec,vec]"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Warding Box]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Warden's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[Warding Box]] · Hex Wright › Casting Tools

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="27.0" cy="27.0" r="2"/><circle cx="65.7" cy="27.0" r="2"/><circle cx="104.3" cy="27.0" r="2"/><circle cx="143.0" cy="27.0" r="2"/><circle cx="7.7" cy="60.5" r="2"/><circle cx="46.4" cy="60.5" r="2"/><circle cx="85.0" cy="60.5" r="2"/><circle cx="123.6" cy="60.5" r="2"/><circle cx="162.3" cy="60.5" r="2"/><circle cx="27.0" cy="94.0" r="2"/><circle cx="65.7" cy="94.0" r="2"/><circle cx="104.3" cy="94.0" r="2"/><circle cx="143.0" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="65.7,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:2.5s" points="65.7,27.0 46.4,60.5 27.0,94.0 65.7,94.0 104.3,94.0 123.6,60.5 143.0,27.0 104.3,27.0 65.7,27.0 85.0,60.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.7" y1="27.0" x2="59.5" y2="37.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="55.1,45.4 55.8,35.6 63.2,39.9" fill="#fc77be"/><circle cx="65.7" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wawqwawa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `[vec,vec]` ([[Vector]], [[List]])
- **ID:** `hexwright:wardens_purification`

```text
before   … │ vec        ◂ top
after    … │ [vec,vec]  ◂ top
```

## Description

Returns the upper and lower vector of the ward cuboid for the Warding Box at the position vec.

## Related patterns (same book page)

- [[Watcher's Gambit]] `vec, entity/item type → —`
- [[Warden's Gambit]] `[vec,vec] → —`
- [[Redirect]] `entity, vec → —`

## Sources

- 📖 **Guidebook:** [[Warding Box]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `WardingBoxActions$2` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:wardens_purification`
