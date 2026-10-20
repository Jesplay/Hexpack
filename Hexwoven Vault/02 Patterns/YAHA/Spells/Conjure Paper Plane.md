---
aliases: ["Conjure Paper Plane"]
tags: [hexcasting/spell, mod/yaha, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "YAHA"
op_id: "yaha:paper_plane"
signature: "wwqaqwwdw"
start_direction: "NORTH_WEST"
input: "vector, entity"
output: "entity"
cost: "Costs one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: ["[[Entity]]"]
entry: "[[Caster's Armaments]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Conjure Paper Plane

✦ **Spell** · [[YAHA]] · 📖 [[Caster's Armaments]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="139" viewBox="0 0 170.0 138.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/></g><polyline class="hexpat-base" points="111.0,114.4 93.7,84.4 76.3,54.3 59.0,24.3 24.3,24.3 41.6,54.3 76.3,54.3 111.0,54.3 145.7,54.3 128.4,84.4 111.0,114.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:347;--dur:2.8s" points="111.0,114.4 93.7,84.4 76.3,54.3 59.0,24.3 24.3,24.3 41.6,54.3 76.3,54.3 111.0,54.3 145.7,54.3 128.4,84.4 111.0,114.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="111.0" y1="114.4" x2="105.5" y2="104.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="101.5,97.9 108.8,102.9 102.2,106.7" fill="#fc77be"/><circle cx="111.0" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wwqaqwwdw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, entity` ([[Vector]], [[Entity]])
- **Leaves:** `entity` ([[Entity]])
- **Cost:** Costs one Amethyst Dust.
- **ID:** `yaha:paper_plane`

```text
before   … │ vector │ entity  ◂ top
after    … │ entity           ◂ top
```

## Description

Summons a projectile that slowly flies towards the given entity, dealing 3 hearts of damage. Costs one [[Amethyst|Amethyst Dust]]. Adds itself onto the [[The Stack|stack]].

A sheet of **[[Media|media]]** bearing the [[Iotas|iota]] of an entity, folded like a paper plane. The embedded iota coerces the plane towards its target. With such fragile construction, it loses form after about 10 seconds. Planes can also be redirected, much like a fireball, and lose their target upon doing so.

Normally, paper planes pass through one another. However, conjuring a plane with another as a target allows the first to collide with the second. Planes without a target fly in a straight path.

## Related patterns (same book page)

- [[Aima Purification]] `entity → entity | null`
- [[Conjure Time Bomb]] `vec, [pattern], num, num → entity`
- [[Kavruka Reflection]] `— → vector`
- [[Conjure Anvil]] `vec → entity`
- [[Conjure Trident]] `vec → entity`

## Sources

- 📖 **Guidebook:** [[Caster's Armaments]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [YAHA on Modrinth](https://modrinth.com/mod/Bqci4HwY)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPaperPlane` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `yaha:paper_plane`
