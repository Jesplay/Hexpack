---
aliases: ["Magic Missile"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/combat]
kind: "Spell"
family: "Spell"
topics: ["Combat & Damage"]
mod: "Hexical"
op_id: "hexical:magic_missile"
signature: "qaqww"
start_direction: "WEST"
input: "vector, vector"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]"]
iotas_out: []
entry: "[[Magic Missile (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Magic Missile

✦ **Spell** · [[Hexical]] · 📖 [[Magic Missile (Book Entry)]] · Patterns › Spells

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,103.4 51.0,103.4 29.8,140.2 72.3,140.2 93.5,103.4 114.8,66.6 136.1,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:1.7s" points="93.5,103.4 51.0,103.4 29.8,140.2 72.3,140.2 93.5,103.4 114.8,66.6 136.1,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="103.4" x2="79.9" y2="103.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,103.4 79.9,98.7 79.9,108.1" fill="#fc77be"/><circle cx="93.5" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqww` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, vector` ([[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexical:magic_missile`

```text
before   … │ vector │ vector  ◂ top
after    …                    ◂ top
```

## Description

Conjures a small silver of amethyst that always deals a full heart of damage and forces the target backwards. Costs about one [[Amethyst|Amethyst Dust]].

The projectile has a few interesting properties. It automatically disappears after roughly ten seconds or shatters upon hitting an entity or block. It pierces all forms of armor and protection. It is also unaffected by gravity and water drag.

*A weak but enchanting sparkling projectile.*

The first vector is conjuration location and is quite esoteric: no wonder most **Hexcasters** conjure it in front of them. The basic gist is a vector, where my head's position is the origin, [[Alidade's Purification|my facing]] is the Z+ axis, [[Theodolite's Purification|the vector pointing up from my head]] is the Y+ axis, and the vector from my head pointing rightwards is the X+ axis. For basic purposes, the zero vector conjures it inside my head and a vector resembling (0, 0, n) conjures it n blocks in front of me.

The second vector is velocity and thankfully much less complicated. It is a simple offset vector, similar to the one that [[Impulse]] takes in. Unlike [[Impulse]] though, this vector can be of arbitrary magnitude without incurring additional costs.

Unlike **Arrows** though, **Magic Missiles** will not increase in damage simply by travelling faster. They also apply an odd knockback effect that ignores the entity's mass or the projectile's speed.

## Sources

- 📖 **Guidebook:** [[Magic Missile (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Magic Missile](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/magic_missile@hexical:magic_missile)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpMagicMissile` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:magic_missile`
