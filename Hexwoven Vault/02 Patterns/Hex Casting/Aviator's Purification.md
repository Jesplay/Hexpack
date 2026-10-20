---
aliases: ["Aviator's Purification"]
tags: [hexcasting/pattern, mod/hex-casting, family/purification, topic/movement, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Movement", "Sensing & Raycasts"]
mod: "Hex Casting"
op_id: "hexcasting:flight/can_fly"
signature: "dwdwdeweaqa"
start_direction: "NORTH_EAST"
input: "entity"
output: "boolean"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Boolean]]"]
entry: "[[Flight]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Aviator's Purification

◇ **Pattern** · [[Hex Casting]] · 📖 [[Flight]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]] · [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="41.6,54.3 59.0,24.3 76.3,54.3 93.7,84.4 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3 111.0,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:416;--dur:3.4s" points="41.6,54.3 59.0,24.3 76.3,54.3 93.7,84.4 59.0,84.4 24.3,84.4 41.6,54.3 76.3,54.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3 111.0,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="41.6" y1="54.3" x2="47.2" y2="44.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.2,37.8 50.5,46.6 43.9,42.8" fill="#fc77be"/><circle cx="41.6" cy="54.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dwdwdeweaqa` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `boolean` ([[Boolean]])
- **ID:** `hexcasting:flight/can_fly`

```text
before   … │ entity   ◂ top
after    … │ boolean  ◂ top
```

## Description

Returns whether the given player is under the effects of [[Anchorite's Flight|Anchorite's]] or [[Wayfarer's Flight]].

Does not detect whether the player can fly by other means.

It is unclear to me what use this has. But I suppose its utility is in the eye of the beholder.

## Related patterns (same book page)

- [[Anchorite's Flight]] `entity, number → —`
- [[Wayfarer's Flight]] `entity, number → —`

## Sources

- 📖 **Guidebook:** [[Flight]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Aviator's Purification](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/flight@hexcasting:flight/can_fly)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCanEntityHexFly` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:flight/can_fly`
