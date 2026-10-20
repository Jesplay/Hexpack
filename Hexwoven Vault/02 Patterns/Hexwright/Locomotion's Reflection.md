---
aliases: ["Locomotion's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright, topic/movement]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic", "Movement"]
mod: "Hexwright"
op_id: "hexwright:locomotions_reflection"
signature: "aqawa"
start_direction: "SOUTH_WEST"
input: ""
output: "num, num"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Locomotion's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[Patterns]] · Hex Wright › Flying Devices

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="113" viewBox="0 0 170.0 113.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/></g><polyline class="hexpat-base" points="85.0,35.0 60.0,78.3 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="85.0,35.0 60.0,78.3 110.0,78.3 135.0,35.0 85.0,35.0 35.0,35.0 60.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="35.0" x2="77.0" y2="48.9" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="71.2,58.8 72.2,46.1 81.8,51.6" fill="#fc77be"/><circle cx="85.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqawa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num, num` ([[Number]])
- **ID:** `hexwright:locomotions_reflection`

```text
before   …              ◂ top
after    … │ num │ num  ◂ top
```

## Description

Your climb speed, then your ground speed.
Free to cast

## Related patterns (same book page)

- [[Conveyance's Reflection]] `— → [state]`
- [[Speedy's Reflection]] `— → [num]`
- [[Vessel's Dismissal]] `bool → —`
- [[Vessel's Reflection]] `— → entity`

## Sources

- 📖 **Guidebook:** [[Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `FlightActions$3` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:locomotions_reflection`
