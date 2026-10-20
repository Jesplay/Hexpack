---
aliases: ["Tier Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:villager_level"
signature: "qeqwqwqwqwqeqawdaeaeaeaeaea"
start_direction: "EAST"
input: "entity"
output: "number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Villagers]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Tier Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Villagers]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="160" height="170" viewBox="0 0 160.2 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.8" cy="20.8" r="2"/><circle cx="50.4" cy="20.8" r="2"/><circle cx="80.1" cy="20.8" r="2"/><circle cx="109.8" cy="20.8" r="2"/><circle cx="139.4" cy="20.8" r="2"/><circle cx="5.9" cy="46.5" r="2"/><circle cx="35.6" cy="46.5" r="2"/><circle cx="65.3" cy="46.5" r="2"/><circle cx="94.9" cy="46.5" r="2"/><circle cx="124.6" cy="46.5" r="2"/><circle cx="154.3" cy="46.5" r="2"/><circle cx="20.8" cy="72.2" r="2"/><circle cx="50.4" cy="72.2" r="2"/><circle cx="80.1" cy="72.2" r="2"/><circle cx="109.8" cy="72.2" r="2"/><circle cx="139.4" cy="72.2" r="2"/><circle cx="5.9" cy="97.8" r="2"/><circle cx="35.6" cy="97.8" r="2"/><circle cx="65.3" cy="97.8" r="2"/><circle cx="94.9" cy="97.8" r="2"/><circle cx="124.6" cy="97.8" r="2"/><circle cx="154.3" cy="97.8" r="2"/><circle cx="20.8" cy="123.5" r="2"/><circle cx="50.4" cy="123.5" r="2"/><circle cx="80.1" cy="123.5" r="2"/><circle cx="109.8" cy="123.5" r="2"/><circle cx="139.4" cy="123.5" r="2"/><circle cx="5.9" cy="149.2" r="2"/><circle cx="35.6" cy="149.2" r="2"/><circle cx="65.3" cy="149.2" r="2"/><circle cx="94.9" cy="149.2" r="2"/><circle cx="124.6" cy="149.2" r="2"/><circle cx="154.3" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="35.6,149.2 65.3,149.2 80.1,123.5 109.8,123.5 124.6,97.8 139.4,72.2 124.6,46.5 109.8,20.8 80.1,20.8 50.4,20.8 35.6,46.5 20.8,72.2 35.6,97.8 20.8,123.5 35.6,149.2 50.4,123.5 65.3,97.8 80.1,123.5 94.9,97.8 124.6,97.8 109.8,72.2 124.6,46.5 94.9,46.5 80.1,20.8 65.3,46.5 35.6,46.5 50.4,72.2 35.6,97.8 65.3,97.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:831;--dur:6.0s" points="35.6,149.2 65.3,149.2 80.1,123.5 109.8,123.5 124.6,97.8 139.4,72.2 124.6,46.5 109.8,20.8 80.1,20.8 50.4,20.8 35.6,46.5 20.8,72.2 35.6,97.8 20.8,123.5 35.6,149.2 50.4,123.5 65.3,97.8 80.1,123.5 94.9,97.8 124.6,97.8 109.8,72.2 124.6,46.5 94.9,46.5 80.1,20.8 65.3,46.5 35.6,46.5 50.4,72.2 35.6,97.8 65.3,97.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.6" y1="149.2" x2="45.1" y2="149.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.9,149.2 45.1,152.5 45.1,146.0" fill="#fc77be"/><circle cx="35.6" cy="149.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeqwqwqwqwqeqawdaeaeaeaeaea` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:villager_level`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes how advanced a villager is as a number, encoding mastery of their craft.

## Related patterns (same book page)

- [[Professional Purification]] `entity → identifier`
- [[Culture Purification]] `entity → identifier`
- [[Nurture Purification]] `identifier → identifier/null`

## Sources

- 📖 **Guidebook:** [[Villagers]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Tier Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/villager@hexpose:villager_level)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetVillagerData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:villager_level`
