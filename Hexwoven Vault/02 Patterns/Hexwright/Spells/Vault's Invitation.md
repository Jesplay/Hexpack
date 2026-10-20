---
aliases: ["Vault's Invitation"]
tags: [hexcasting/spell, mod/hexwright, family/spell, topic/hexwright, topic/storage]
kind: "Spell"
family: "Spell"
topics: ["Hex Wright Magic", "Reading & Writing"]
mod: "Hexwright"
op_id: "hexwright:vaults_invitation"
signature: "wqeawwaeqwqaeqaqe"
start_direction: "SOUTH_WEST"
input: "player entity"
output: ""
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Vaults]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Vault's Invitation

✦ **Spell** · [[Hexwright]] · 📖 [[Vaults]] · Hex Wright › Storage

**Topics:** [[Hex Wright Magic (Topic)|Hex Wright Magic]] · [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="154" height="170" viewBox="0 0 153.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.5" cy="24.5" r="2"/><circle cx="59.4" cy="24.5" r="2"/><circle cx="94.4" cy="24.5" r="2"/><circle cx="129.3" cy="24.5" r="2"/><circle cx="7.0" cy="54.7" r="2"/><circle cx="41.9" cy="54.7" r="2"/><circle cx="76.9" cy="54.7" r="2"/><circle cx="111.8" cy="54.7" r="2"/><circle cx="146.8" cy="54.7" r="2"/><circle cx="24.5" cy="85.0" r="2"/><circle cx="59.4" cy="85.0" r="2"/><circle cx="94.4" cy="85.0" r="2"/><circle cx="129.3" cy="85.0" r="2"/><circle cx="7.0" cy="115.3" r="2"/><circle cx="41.9" cy="115.3" r="2"/><circle cx="76.9" cy="115.3" r="2"/><circle cx="111.8" cy="115.3" r="2"/><circle cx="146.8" cy="115.3" r="2"/><circle cx="24.5" cy="145.5" r="2"/><circle cx="59.4" cy="145.5" r="2"/><circle cx="94.4" cy="145.5" r="2"/><circle cx="129.3" cy="145.5" r="2"/></g><polyline class="hexpat-base" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5 59.4,145.5 94.4,145.5 129.3,145.5 111.8,115.3 129.3,85.0 111.8,54.7 94.4,24.5 59.4,24.5 76.9,54.7 59.4,85.0 76.9,115.3 94.4,85.0 76.9,54.7 94.4,24.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:629;--dur:5.0s" points="59.4,24.5 41.9,54.7 24.5,85.0 41.9,115.3 24.5,145.5 59.4,145.5 94.4,145.5 129.3,145.5 111.8,115.3 129.3,85.0 111.8,54.7 94.4,24.5 59.4,24.5 76.9,54.7 59.4,85.0 76.9,115.3 94.4,85.0 76.9,54.7 94.4,24.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="59.4" y1="24.5" x2="53.8" y2="34.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="49.8,41.1 50.5,32.2 57.2,36.1" fill="#fc77be"/><circle cx="59.4" cy="24.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wqeawwaeqwqaeqaqe` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `player entity` ([[Entity]])
- **Leaves:** `nothing` (—)
- **ID:** `hexwright:vaults_invitation`

```text
before   … │ player entity  ◂ top
after    …                  ◂ top
```

## Description

Toggles that player into the held key's Vault allow list.
Costs 1/10 Dust

## Related patterns (same book page)

- [[Vault's Gambit]] `— → —`
- [[Vault's Relocation Gambit]] `vec → —`

## Sources

- 📖 **Guidebook:** [[Vaults]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `VaultActions$2` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexwright:vaults_invitation`
