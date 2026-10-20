---
aliases: ["Kavruka Reflection"]
tags: [hexcasting/pattern, mod/yaha, family/reflection, topic/combat]
kind: "Pattern"
family: "Reflection"
topics: ["Combat & Damage"]
mod: "YAHA"
op_id: "yaha:time_bomb_pos"
signature: "eewaqaweedd"
start_direction: "NORTH_WEST"
input: ""
output: "vector"
cost: ""
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Caster's Armaments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Kavruka Reflection

◇ **Pattern** · [[YAHA]] · 📖 [[Caster's Armaments]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,140.2 29.8,103.4 51.0,66.6 93.5,66.6 136.1,66.6 114.8,29.8 72.3,29.8 93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 72.3,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="51.0,140.2 29.8,103.4 51.0,66.6 93.5,66.6 136.1,66.6 114.8,29.8 72.3,29.8 93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2 72.3,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="140.2" x2="44.2" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,120.0 48.3,126.1 40.2,130.8" fill="#fc77be"/><circle cx="51.0" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eewaqaweedd` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **ID:** `yaha:time_bomb_pos`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Returns the position of the time bomb casting this pattern. [[Mishaps|Mishaps]] if not cast by a bomb.

## Related patterns (same book page)

- [[Conjure Paper Plane]] `vector, entity → entity`
- [[Aima Purification]] `entity → entity | null`
- [[Conjure Time Bomb]] `vec, [pattern], num, num → entity`
- [[Conjure Anvil]] `vec → entity`
- [[Conjure Trident]] `vec → entity`

## Sources

- 📖 **Guidebook:** [[Caster's Armaments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpTimeBombPos` is not a `SpellAction`
- 🔧 **Pattern ID:** `yaha:time_bomb_pos`
