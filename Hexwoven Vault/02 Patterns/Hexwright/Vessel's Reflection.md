---
aliases: ["Vessel's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright, topic/movement]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic", "Movement"]
mod: "Hexwright"
op_id: "hexwright:conveyance_entitys_reflection"
signature: "aqaewdeqaa"
start_direction: "SOUTH_WEST"
input: ""
output: "entity"
cost: ""
iotas_in: []
iotas_out: ["[[Entity]]"]
entry: "[[Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vessel's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[Patterns]] · Hex Wright › Flying Devices

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 51.0,140.2 93.5,140.2 114.8,103.4 72.3,103.4 51.0,66.6 29.8,29.8 72.3,29.8 93.5,66.6 136.1,66.6 114.8,29.8 93.5,66.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="72.3,103.4 51.0,140.2 93.5,140.2 114.8,103.4 72.3,103.4 51.0,66.6 29.8,29.8 72.3,29.8 93.5,66.6 136.1,66.6 114.8,29.8 93.5,66.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="65.5" y2="115.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,123.7 61.4,112.9 69.5,117.5" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqaewdeqaa` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `entity` ([[Entity]])
- **ID:** `hexwright:conveyance_entitys_reflection`

```text
before   …           ◂ top
after    … │ entity  ◂ top
```

## Description

Returns the Broom or Carpet itself as an entity. This is how you read its position.
Free to cast

## Related patterns (same book page)

- [[Conveyance's Reflection]] `— → [state]`
- [[Locomotion's Reflection]] `— → num, num`
- [[Speedy's Reflection]] `— → [num]`
- [[Vessel's Dismissal]] `bool → —`

## Sources

- 📖 **Guidebook:** [[Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `FlightActions$5` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:conveyance_entitys_reflection`
