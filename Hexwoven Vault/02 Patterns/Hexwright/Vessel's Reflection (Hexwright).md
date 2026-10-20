---
aliases: ["Vessel's Reflection"]
tags: [hexcasting/pattern, mod/hexwright, family/reflection, topic/hexwright]
kind: "Pattern"
family: "Reflection"
topics: ["Hex Wright Magic"]
mod: "Hexwright"
op_id: "hexwright:vessel_purification"
signature: "wqwqwqwqweawwawwawaw"
start_direction: "SOUTH_WEST"
input: "item"
output: "str, num"
cost: ""
iotas_in: ["[[Item Stack]]"]
iotas_out: ["[[Number]]", "[[String]]"]
entry: "[[The Hex Engraved Bottle]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vessel's Reflection

◇ **Pattern** · [[Hexwright]] · 📖 [[The Hex Engraved Bottle]] · Hex Wright › Remnants

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.9" cy="20.8" r="2"/><circle cx="35.6" cy="20.8" r="2"/><circle cx="65.3" cy="20.8" r="2"/><circle cx="94.9" cy="20.8" r="2"/><circle cx="124.6" cy="20.8" r="2"/><circle cx="154.3" cy="20.8" r="2"/><circle cx="20.8" cy="46.5" r="2"/><circle cx="50.4" cy="46.5" r="2"/><circle cx="80.1" cy="46.5" r="2"/><circle cx="109.8" cy="46.5" r="2"/><circle cx="139.4" cy="46.5" r="2"/><circle cx="5.9" cy="72.2" r="2"/><circle cx="35.6" cy="72.2" r="2"/><circle cx="65.3" cy="72.2" r="2"/><circle cx="94.9" cy="72.2" r="2"/><circle cx="124.6" cy="72.2" r="2"/><circle cx="154.3" cy="72.2" r="2"/><circle cx="20.8" cy="97.8" r="2"/><circle cx="50.4" cy="97.8" r="2"/><circle cx="80.1" cy="97.8" r="2"/><circle cx="109.8" cy="97.8" r="2"/><circle cx="139.4" cy="97.8" r="2"/><circle cx="5.9" cy="123.5" r="2"/><circle cx="35.6" cy="123.5" r="2"/><circle cx="65.3" cy="123.5" r="2"/><circle cx="94.9" cy="123.5" r="2"/><circle cx="124.6" cy="123.5" r="2"/><circle cx="154.3" cy="123.5" r="2"/><circle cx="20.8" cy="149.2" r="2"/><circle cx="50.4" cy="149.2" r="2"/><circle cx="80.1" cy="149.2" r="2"/><circle cx="109.8" cy="149.2" r="2"/><circle cx="139.4" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="50.4,46.5 35.6,72.2 20.8,97.8 35.6,123.5 50.4,149.2 80.1,149.2 109.8,149.2 124.6,123.5 139.4,97.8 124.6,72.2 109.8,46.5 124.6,20.8 94.9,20.8 65.3,20.8 35.6,20.8 50.4,46.5 65.3,72.2 80.1,97.8 94.9,72.2 109.8,46.5 80.1,46.5 50.4,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:623;--dur:5.9s" points="50.4,46.5 35.6,72.2 20.8,97.8 35.6,123.5 50.4,149.2 80.1,149.2 109.8,149.2 124.6,123.5 139.4,97.8 124.6,72.2 109.8,46.5 124.6,20.8 94.9,20.8 65.3,20.8 35.6,20.8 50.4,46.5 65.3,72.2 80.1,97.8 94.9,72.2 109.8,46.5 80.1,46.5 50.4,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.4" y1="46.5" x2="45.7" y2="54.7" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="42.3,60.6 42.9,53.1 48.5,56.3" fill="#fc77be"/><circle cx="50.4" cy="46.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqwqwqwqweawwawwawaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `item` ([[Item Stack]])
- **Leaves:** `str, num` ([[Number]], [[String]])
- **ID:** `hexwright:vessel_purification`

```text
before   … │ item       ◂ top
after    … │ str │ num  ◂ top
```

## Description

Takes an Item [[The Stack|Stack]] of a bottle and returns the contained remnant's name, and quantity of Rems.

## Related patterns (same book page)

- [[Remnant Decanting]] `num → remnant`

## Sources

- 📖 **Guidebook:** [[The Hex Engraved Bottle]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `RemnantActions$2` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexwright:vessel_purification`
