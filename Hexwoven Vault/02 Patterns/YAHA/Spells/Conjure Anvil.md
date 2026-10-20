---
aliases: ["Conjure Anvil"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "YAHA"
op_id: "yaha:anvil"
signature: "dqdwdqdqaa"
start_direction: "WEST"
input: "vec"
output: "entity"
cost: "Costs one Charged Amethyst."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Caster's Armaments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Anvil

✦ **Spell** · [[YAHA]] · 📖 [[Caster's Armaments]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 29.8,103.4 51.0,66.6 29.8,29.8 72.3,29.8 114.8,29.8 93.5,66.6 114.8,103.4 72.3,103.4 51.0,140.2 93.5,140.2 72.3,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:468;--dur:3.1s" points="72.3,103.4 29.8,103.4 51.0,66.6 29.8,29.8 72.3,29.8 114.8,29.8 93.5,66.6 114.8,103.4 72.3,103.4 51.0,140.2 93.5,140.2 72.3,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="58.7" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.9,103.4 58.7,98.7 58.7,108.1" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dqdwdqdqaa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs one Charged Amethyst.
- **ID:** `yaha:anvil`

```text
before   … │ vec     ◂ top
after    … │ entity  ◂ top
```

## Description

Summons an anvil that deals damage based on its velocity. Damage dealt never goes above 10 hearts. Costs one [[Amethyst|Charged Amethyst]]. Adds itself onto the [[The Stack|stack]].

Dense yet brittle, shattering upon collision with the ground or when it has collided with an entity thrice. The anvil does not collide with other anvils, or with paper planes that belong to the same caster.

i think i just heard "1000 falling anvil danmaku" two squads over.

## Related patterns (same book page)

- [[Conjure Paper Plane]] `vector, entity → entity`
- [[Aima Purification]] `entity → entity | null`
- [[Conjure Time Bomb]] `vec, [pattern], num, num → entity`
- [[Kavruka Reflection]] `— → vector`
- [[Conjure Trident]] `vec → entity`

## Sources

- 📖 **Guidebook:** [[Caster's Armaments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpAnvil` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `yaha:anvil`
