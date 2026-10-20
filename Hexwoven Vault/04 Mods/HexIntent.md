---
tags: [hexcasting/mod]
version: "2.2.2"
---

# HexIntent

> HexIntent is a companion mod for Hex Casting focused on interactive menus and delegated casting. Stack up a menu definition, cast CREATE_LIST_MENU or CREATE_GRID_MENU, and an interactive HUD opens so each choice can dispatch stored patterns as a normal Hex.

🧩 [Modrinth page](https://modrinth.com/mod/4G64Rm66)

**11** book entries · **32** patterns · **0** spells · **0** great spells

## Book entries

- [[Charming Intent]] — Spells
- [[Contained Splinter]] — Items
- [[Equation Synthesizer]] — Items
- [[Hex Reliquary]] — Items
- [[Manifest Relay (Book Entry)]] — Spells
- [[Manifest Threshold (Book Entry)]] — Spells
- [[Memory Crystal]] — Items
- [[Menu Manifestation]] — Spells
- [[Mind Vault]] — Items
- [[Splinters]] — Spells
- [[Visual Manifestation]] — Spells

## Patterns

- [[Manifest List Menu]] `list, text → —`
- [[Manifest Grid Menu]] `list, text, num → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Intent Button]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Numeric Input]] `num, text → intent`
- [[Intent Slider]] `num, num, num, text → intent`
- [[Intent Checkbox]] `bool, text → intent`
- [[Intent Select List]] `list, num, text, bool? → intent`
- [[Intent Section]] `text → intent`
- [[Intent Dropdown]] `list, text → intent`
- [[Manifest Threshold]] `vec, presence, num, text?, pigment? → none`
- [[Presence Intent]] `vec, vec → presenceIntent`
- [[Raze Manifestation]] `vec, num → none`
- [[Manifest Relay]] `vec, vec, vec, num? → none`
- [[Equation Hex Cloud]] `vec|entity, equation, num → none`
- [[Hex Trail]] `vec, vec, vec, num, num, num? → none`
- [[Manifest Spell Circle]] `[pattern], vec, vec, num, num?, vec? → none`
- [[Manifest Splinter]] `vec, num, list → none`
- [[Renew Splinter]] `vec, num → none`
- [[Saturn's Gambit]] `pattern → none`
- [[Get Splinter Location]] `— → vec`
- [[Destroy Current Splinter]] `— → none`
- [[Destroy Splinters]] `— → none`
- [[Set Charm Cast Sound]] `string | text | null, item entity? → none`
- [[Exit If Interacting]] `none → none`
- [[Open Casting Screen]] `none → none`
- [[Clear Stack]] `none → none`
- [[Mind Vault Counts]] `vec → [num, num]`
- [[Resonance Reflection]] `none → memory`
- [[Resonance Purification]] `memory → [pattern] | null`
- [[Resonance Gambit]] `memory, any → none`
