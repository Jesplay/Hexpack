---
aliases: ["Mind Vault Counts"]
tags: [hexcasting/pattern, mod/hexintent, family/other, topic/storage]
kind: "Pattern"
family: "Other"
topics: ["Reading & Writing"]
mod: "HexIntent"
op_id: "manifestation:mind_vault_counts"
signature: "qqawddad"
start_direction: "SOUTH_EAST"
input: "vec"
output: "[num, num]"
cost: ""
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Mind Vault]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mind Vault Counts

◇ **Pattern** · [[HexIntent]] · 📖 [[Mind Vault]] · Items

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.0" cy="35.0" r="2"/><circle cx="60.0" cy="35.0" r="2"/><circle cx="110.0" cy="35.0" r="2"/><circle cx="160.0" cy="35.0" r="2"/><circle cx="35.0" cy="78.3" r="2"/><circle cx="85.0" cy="78.3" r="2"/><circle cx="135.0" cy="78.3" r="2"/><circle cx="10.0" cy="121.6" r="2"/><circle cx="60.0" cy="121.6" r="2"/><circle cx="110.0" cy="121.6" r="2"/><circle cx="160.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 85.0,78.3 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:450;--dur:2.5s" points="35.0,78.3 60.0,121.6 110.0,121.6 135.0,78.3 85.0,78.3 35.0,78.3 60.0,35.0 85.0,78.3 110.0,35.0 135.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="35.0" y1="78.3" x2="43.0" y2="92.2" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.8,102.1 38.2,94.9 47.8,89.4" fill="#fc77be"/><circle cx="35.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqawddad` · **Start** `south-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec` ([[Vector]])
- **Leaves:** `[num, num]` ([[Number]], [[List]])
- **ID:** `manifestation:mind_vault_counts`

```text
before   … │ vec         ◂ top
after    … │ [num, num]  ◂ top
```

## Description

Takes a vector of a mind vault location and returns \[ready, total\], where ready is the number of stored minds not on cooldown and total is the number of stored minds in the vault.

## Sources

- 📖 **Guidebook:** [[Mind Vault]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpMindVaultCounts` is not a `SpellAction`
- 🔧 **Pattern ID:** `manifestation:mind_vault_counts`
