---
aliases: ["Gunslinger's Reflection"]
tags: [hexcasting/pattern, mod/hex-machina, family/reflection, topic/combat, topic/devices]
kind: "Pattern"
family: "Reflection"
topics: ["Combat & Damage", "Casting Devices"]
mod: "Hex Machina"
op_id: "hex_machina:current_bullet"
signature: "awqqqwaqweaqaaw"
start_direction: "SOUTH_WEST"
input: ""
output: "num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Guns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gunslinger's Reflection

◇ **Pattern** · [[Hex Machina]] · 📖 [[Guns]] · Patterns

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="90" viewBox="0 0 170.0 90.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.2" cy="20.2" r="2"/><circle cx="49.0" cy="20.2" r="2"/><circle cx="77.8" cy="20.2" r="2"/><circle cx="106.6" cy="20.2" r="2"/><circle cx="135.4" cy="20.2" r="2"/><circle cx="164.2" cy="20.2" r="2"/><circle cx="5.8" cy="45.1" r="2"/><circle cx="34.6" cy="45.1" r="2"/><circle cx="63.4" cy="45.1" r="2"/><circle cx="92.2" cy="45.1" r="2"/><circle cx="121.0" cy="45.1" r="2"/><circle cx="149.8" cy="45.1" r="2"/><circle cx="20.2" cy="70.1" r="2"/><circle cx="49.0" cy="70.1" r="2"/><circle cx="77.8" cy="70.1" r="2"/><circle cx="106.6" cy="70.1" r="2"/><circle cx="135.4" cy="70.1" r="2"/><circle cx="164.2" cy="70.1" r="2"/></g><polyline class="hexpat-base" points="34.6,45.1 20.2,70.1 49.0,70.1 77.8,70.1 92.2,45.1 77.8,20.2 49.0,20.2 20.2,20.2 34.6,45.1 63.4,45.1 92.2,45.1 106.6,70.1 121.0,45.1 106.6,20.2 92.2,45.1 121.0,45.1 149.8,45.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:461;--dur:4.5s" points="34.6,45.1 20.2,70.1 49.0,70.1 77.8,70.1 92.2,45.1 77.8,20.2 49.0,20.2 20.2,20.2 34.6,45.1 63.4,45.1 92.2,45.1 106.6,70.1 121.0,45.1 106.6,20.2 92.2,45.1 121.0,45.1 149.8,45.1" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.6" y1="45.1" x2="30.0" y2="53.1" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="26.7,58.8 27.2,51.5 32.7,54.7" fill="#fc77be"/><circle cx="34.6" cy="45.1" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awqqqwaqweaqaaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num` ([[Number]])
- **ID:** `hex_machina:current_bullet`

```text
before   …        ◂ top
after    … │ num  ◂ top
```

## Description

Gets the current bullet slot in the gun

## Related patterns (same book page)

- [[Gunslinger's Prog]] `— → —`
- [[Gunslinger's Regr]] `— → —`
- [[Gunslinger's Roulette]] `num → —`
- [[Colt's Reflection]] `— → entity`

## Sources

- 📖 **Guidebook:** [[Guns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hex Machina on Modrinth](https://modrinth.com/mod/uQ6K0yH1)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** from the mod code: its action class `CurrentBulletAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hex_machina:current_bullet`
