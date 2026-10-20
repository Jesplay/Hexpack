---
aliases: ["Colt's Reflection"]
tags: [hexcasting/pattern, mod/hex-machina, family/reflection, topic/combat, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Combat & Damage", "Casting Devices"]
mod: "Hex Machina"
op_id: "hex_machina:get_bullet"
signature: "qaqqqeqaww"
start_direction: "NORTH_WEST"
input: ""
output: "entity"
cost: ""
iotas_in: []
iotas_out: ["[[Entity]]"]
entry: "[[Guns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Colt's Reflection

◇ **Pattern** · [[Hex Machina]] · 📖 [[Guns]] · Patterns

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="119" height="170" viewBox="0 0 118.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.0" cy="24.5" r="2"/><circle cx="41.9" cy="24.5" r="2"/><circle cx="76.9" cy="24.5" r="2"/><circle cx="111.8" cy="24.5" r="2"/><circle cx="24.5" cy="54.7" r="2"/><circle cx="59.4" cy="54.7" r="2"/><circle cx="94.4" cy="54.7" r="2"/><circle cx="7.0" cy="85.0" r="2"/><circle cx="41.9" cy="85.0" r="2"/><circle cx="76.9" cy="85.0" r="2"/><circle cx="111.8" cy="85.0" r="2"/><circle cx="24.5" cy="115.3" r="2"/><circle cx="59.4" cy="115.3" r="2"/><circle cx="94.4" cy="115.3" r="2"/><circle cx="7.0" cy="145.5" r="2"/><circle cx="41.9" cy="145.5" r="2"/><circle cx="76.9" cy="145.5" r="2"/><circle cx="111.8" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="76.9,145.5 59.4,115.3 24.5,115.3 41.9,145.5 76.9,145.5 94.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:384;--dur:3.1s" points="76.9,145.5 59.4,115.3 24.5,115.3 41.9,145.5 76.9,145.5 94.4,115.3 76.9,85.0 94.4,54.7 76.9,24.5 59.4,54.7 41.9,85.0 24.5,115.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.9" y1="145.5" x2="71.3" y2="135.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.3,128.9 74.6,133.9 68.0,137.8" fill="#fc77be"/><circle cx="76.9" cy="145.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqeqaww` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `entity` ([[Entity]])
- **ID:** `hex_machina:get_bullet`

```text
before   …           ◂ top
after    … │ entity  ◂ top
```

## Description

Gets a reference to the bullet casting this hex

## Related patterns (same book page)

- [[Gunslinger's Reflection]] `— → num`
- [[Gunslinger's Prog]] `— → —`
- [[Gunslinger's Regr]] `— → —`
- [[Gunslinger's Roulette]] `num → —`

## Sources

- 📖 **Guidebook:** [[Guns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hex Machina on Modrinth](https://modrinth.com/mod/uQ6K0yH1)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** from the mod code: its action class `BulletReflectionAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hex_machina:get_bullet`
