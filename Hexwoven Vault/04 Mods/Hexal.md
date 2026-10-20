---
tags: [hexcasting/mod]
version: "0.3.1"
---

# Hexal

> Extension mod for Hex Casting to add new patterns (and whatever else I feel like).

🧩 [Modrinth page](https://modrinth.com/mod/aBVJ6Q36) · 🌐 [Official online docs](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/)

**14** book entries · **52** patterns · **23** spells · **4** great spells

## Book entries

- [[Accelerate (Book Entry)]] — Great Spells
- [[Everbook]] — Patterns
- [[Everbook Entries]] — Patterns
- [[Fabric-Only Hexal Spells]] — Cross-Mod Compatibility
- [[Gates]] — Great Spells
- [[Great Wisp Spells]] — Great Spells
- [[Hexal Basic Patterns]] — Patterns
- [[Hexal Maths]] — Patterns
- [[Hexal Spells]] — Spells
- [[Links]] — Spells
- [[Motes]] — Spells
- [[Relay]] — Items
- [[Slipways]] — Getting Started
- [[Wisps]] — Spells

## Great Spells

- [[Accelerate]] `vec → —`
- [[Consume Wisp]] `entity → —`
- [[Bind Wisp]] `entity → —`
- [[Gate's Reflection]] `null | vec | vec, entity → gate`

## Spells

- [[Smelt]] `vec | entity | mote → —`
- [[Freeze]] `vec → —`
- [[Falling Block]] `vec → —`
- [[Place Block II]] `(itemtype | mote), vec → —`
- [[Particles]] `vec | [vec] → —`
- [[Summon Projectile Wisp]] `[pattern], vec, vec, number → —`
- [[Summon Cyclic Wisp]] `[pattern], vec, number → —`
- [[Haste]] `num → —`
- [[Allow Transfer]] `num → —`
- [[Disallow Transfer]] `num → —`
- [[Allow Transfer Others]] `wisp, num → —`
- [[Disallow Transfer Others]] `wisp, num → —`
- [[Link]] `linkable → —`
- [[Link Others]] `linkable, linkable → —`
- [[Unlink]] `num → —`
- [[Unlink Others]] `linkable, linkable → —`
- [[Send Iota]] `num, any → —`
- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
- [[Gate's Opening]] `gate, entity → —`
- [[Gate's Dismissal]] `gate, entity → —`
- [[Phase Block]] `vec, num → —`
- [[Mnemosyne's Gambit]] `vector, pattern → —`

## Patterns

- [[Identity Reflection]] `— → entity`
- [[Reservoir Reflection]] `— → num`
- [[Manager's Purification]] `entity → [pattern]`
- [[Allegiance Distillation]] `entity, entity → bool`
- [[Pathfinder's Gambit]] `vec → —`
- [[Pathfinder's Reflection]] `— → vec`
- [[Speedometer's Reflection]] `— → num`
- [[Entity Purification Wisp]] `vec → entity | null`
- [[Zone Distillation Wisp]] `vec, num → [entity]`
- [[Zone Distillation Non-Wisp]] `vec, num → [entity]`
- [[Delay Wisp]] `number → —`
- [[Listen]] `— → —`
- [[Wander]] `— → —`
- [[Phonebook Purification]] `num → linkable`
- [[Recognition Purification]] `linkable → num`
- [[Popularity Reflection]] `— → num`
- [[Recitation Reflection]] `— → any`
- [[Postmaster's Reflection]] `— → num`
- [[Unclogging Gambit]] `— → —`
- [[Open Transmit]] `num → —`
- [[Close Transmit]] `— → —`
- [[Stocktake Reflection]] `— → [itemtype]`
- [[Stocktake Purification]] `mote | itemtype → [mote]`
- [[Capacity Reflection]] `— → int`
- [[Containment Distillation]] `vec, (itemtype | mote) → bool`
- [[Mediafy Item]] `item | item, mote → mote`
- [[Return Item]] `mote, vec | mote, vec, num → —`
- [[Stacking Distillation II]] `mote, (mote | item) → bool`
- [[Splitting Gambit]] `mote, int → mote, mote`
- [[Depot Purification]] `mote → vec`
- [[Depot Gambit]] `mote, vec → mote`
- [[Craft]] `mote | [mote] | [[mote]] → [mote]`
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] `villager → int`
- [[Offering Purification]] `villager → [complicated!]`
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`
- [[Familiar's Reflection]] `— → entity | null`
- [[Marked Distillation]] `gate, entity → bool`
- [[Gate's Closing]] `gate | gate, vec → —`
- [[Timekeeper's Reflection]] `— → number`
- [[Diver's Purification]] `entity → num`
- [[Nurse's Purification]] `entity → num`
- [[Squire's Purification]] `entity → num`
- [[Boxer's Purification]] `entity → num`
- [[Lamplighter's Purification]] `vec → num`
- [[Factorial Purification]] `num → num`
- [[Running Sum Purification]] `[num] → [num]`
- [[Running Product Purification]] `[num] → [num]`
- [[Elysium's Gambit]] `vector, pattern → —`
- [[Lethe's Gambit]] `pattern → —`
- [[Brigh's Gambit]] `pattern → —`
