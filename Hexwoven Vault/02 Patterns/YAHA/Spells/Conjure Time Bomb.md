---
aliases: ["Conjure Time Bomb"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "YAHA"
op_id: "yaha:time_bomb"
signature: "eewaqawee"
start_direction: "NORTH_WEST"
input: "vec, [pattern], num, num"
output: "entity"
cost: "Costs one Charged Amethyst, plus the media given to the bomb."
iotas_in: ["[[Number]]", "[[Vector]]", "[[List]]", "[[Pattern]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Caster's Armaments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Time Bomb

✦ **Spell** · [[YAHA]] · 📖 [[Caster's Armaments]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="51.0,140.2 29.8,103.4 51.0,66.6 93.5,66.6 136.1,66.6 114.8,29.8 72.3,29.8 93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:425;--dur:2.8s" points="51.0,140.2 29.8,103.4 51.0,66.6 93.5,66.6 136.1,66.6 114.8,29.8 72.3,29.8 93.5,66.6 114.8,103.4 93.5,140.2 51.0,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="51.0" y1="140.2" x2="44.2" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="39.3,120.0 48.3,126.1 40.2,130.8" fill="#fc77be"/><circle cx="51.0" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eewaqawee` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, [pattern], num, num` ([[Number]], [[Vector]], [[List]], [[Pattern]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs one Charged Amethyst, plus the media given to the bomb.
- **ID:** `yaha:time_bomb`

```text
before   … │ vec │ [pattern] │ num │ num  ◂ top
after    … │ entity                       ◂ top
```

## Description

Summons a floating projectile that casts after a given duration. Costs one [[Amethyst|Charged Amethyst]], plus the **[[Media|media]]** given to the bomb. Adds itself onto the [[The Stack|stack]].

A hex suspended in a fluid of **media** and encased in clear amethyst.

The required arguments are as follows:
- The position of the bomb
- A list of patterns to be cast by the bomb
- The amount of **media** in units of [[Amethyst|Dust]] given to the bomb's cast
- The duration before casting, in ticks

The bomb casts as the player that produced it, has an ambit of 8 blocks around it, and can be knocked back by attacks and other projectiles. The bomb entity still exists during its cast and disintegrates after, which must be considered when dealing with [[Entities|Zone Distillations]].

A slow, whorling current of violet pearlescence.
It is entrancing - more than any other device I have been given yet

## Related patterns (same book page)

- [[Conjure Paper Plane]] `vector, entity → entity`
- [[Aima Purification]] `entity → entity | null`
- [[Kavruka Reflection]] `— → vector`
- [[Conjure Anvil]] `vec → entity`
- [[Conjure Trident]] `vec → entity`

## Sources

- 📖 **Guidebook:** [[Caster's Armaments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpTimeBomb` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `yaha:time_bomb`
