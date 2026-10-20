---
aliases: ["Enchant Arms"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/enchanting]
kind: "Spell"
family: "Spell"
topics: ["Enchanting & Imbuement"]
mod: "Lapisworks"
op_id: "lapisworks:gib_reach"
signature: "edewwqewdwwewdaw"
start_direction: "SOUTH_WEST"
input: "entity, bool"
output: ""
cost: "Costs 16 Amel (or 0 if disenchanting) and four shards in media to change that player."
iotas_in: ["[[Boolean]]", "[[Entity]]"]
iotas_out: []
entry: "[[Lapisworks Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enchant Arms

✦ **Spell** · [[Lapisworks]] · 📖 [[Lapisworks Patterns]] · Lapisworks

**Topics:** [[Enchanting & Imbuement (Topic)|Enchanting & Imbuement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="92" viewBox="0 0 170.0 91.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.1" cy="16.1" r="2"/><circle cx="39.1" cy="16.1" r="2"/><circle cx="62.0" cy="16.1" r="2"/><circle cx="85.0" cy="16.1" r="2"/><circle cx="108.0" cy="16.1" r="2"/><circle cx="130.9" cy="16.1" r="2"/><circle cx="153.9" cy="16.1" r="2"/><circle cx="4.6" cy="36.0" r="2"/><circle cx="27.6" cy="36.0" r="2"/><circle cx="50.5" cy="36.0" r="2"/><circle cx="73.5" cy="36.0" r="2"/><circle cx="96.5" cy="36.0" r="2"/><circle cx="119.5" cy="36.0" r="2"/><circle cx="142.4" cy="36.0" r="2"/><circle cx="165.4" cy="36.0" r="2"/><circle cx="16.1" cy="55.9" r="2"/><circle cx="39.1" cy="55.9" r="2"/><circle cx="62.0" cy="55.9" r="2"/><circle cx="85.0" cy="55.9" r="2"/><circle cx="108.0" cy="55.9" r="2"/><circle cx="130.9" cy="55.9" r="2"/><circle cx="153.9" cy="55.9" r="2"/><circle cx="4.6" cy="75.8" r="2"/><circle cx="27.6" cy="75.8" r="2"/><circle cx="50.5" cy="75.8" r="2"/><circle cx="73.5" cy="75.8" r="2"/><circle cx="96.5" cy="75.8" r="2"/><circle cx="119.5" cy="75.8" r="2"/><circle cx="142.4" cy="75.8" r="2"/><circle cx="165.4" cy="75.8" r="2"/></g><polyline class="hexpat-base" points="50.5,36.0 39.1,55.9 16.1,55.9 27.6,36.0 50.5,36.0 73.5,36.0 96.5,36.0 108.0,16.1 130.9,16.1 153.9,16.1 142.4,36.0 130.9,55.9 119.5,75.8 96.5,75.8 73.5,75.8 85.0,55.9 62.0,55.9 39.1,55.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:391;--dur:4.8s" points="50.5,36.0 39.1,55.9 16.1,55.9 27.6,36.0 50.5,36.0 73.5,36.0 96.5,36.0 108.0,16.1 130.9,16.1 153.9,16.1 142.4,36.0 130.9,55.9 119.5,75.8 96.5,75.8 73.5,75.8 85.0,55.9 62.0,55.9 39.1,55.9" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.5" y1="36.0" x2="46.9" y2="42.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.2,46.9 44.7,41.1 49.1,43.6" fill="#fc77be"/><circle cx="50.5" cy="36.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edewwqewdwwewdaw` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, bool` ([[Boolean]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 16 Amel (or 0 if disenchanting) and four shards in media to change that player.
- **ID:** `lapisworks:gib_reach`

```text
before   … │ entity │ bool  ◂ top
after    …                  ◂ top
```

## Description

Enchants the given *player's* arms (or disenchants if given **false**), increasing their reach by 3 blocks and their attack range by 1. Costs 16 **Amel** (or 0 if disenchanting) and four shards in [[Media|media]] to change that player. There is no customizability here.

A sidenote: it seems this spell does not extend one's arms, but rather grants the arms each a ghost-like extension which only comes out when required. Fascinating. Somewhat strangely, these two attribute enhancements given (reach and attack range respectively) count as separate for the purposes of **Enhancement Prfn.**

## Related patterns (same book page)

- [[Imbue Lapis]] `— → —`
- [[Reclaim Amethyst]] `— → —`
- [[Mold Amel]] `— → —`
- [[Enchant Skin]] `entity, num → —`
- [[Enchant Fists]] `entity, num → —`
- [[Enchant Feet]] `entity, num → —`
- [[Enhancement Purification]] `entity, num → —`

## Sources

- 📖 **Guidebook:** [[Lapisworks Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `MoarReachYouBitch` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:gib_reach`
