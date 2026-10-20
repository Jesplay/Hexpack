---
aliases: ["Media Reflection"]
tags: [hexcasting/pattern, mod/hexpose, family/reflection, topic/sensing]
kind: "Pattern"
family: "Reflection"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:env_media"
signature: "dde"
start_direction: "WEST"
input: ""
output: "number"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]"]
entry: "[[Media (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Media Reflection

◇ **Pattern** · [[Hexpose]] · 📖 [[Media (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="130" height="170" viewBox="0 0 130.3 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="10.9" cy="38.0" r="2"/><circle cx="65.1" cy="38.0" r="2"/><circle cx="119.4" cy="38.0" r="2"/><circle cx="38.0" cy="85.0" r="2"/><circle cx="92.3" cy="85.0" r="2"/><circle cx="10.9" cy="132.0" r="2"/><circle cx="65.1" cy="132.0" r="2"/><circle cx="119.4" cy="132.0" r="2"/></g><polyline class="hexpat-base" points="92.3,85.0 38.0,85.0 65.1,38.0 92.3,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:1.6s" points="92.3,85.0 38.0,85.0 65.1,38.0 92.3,85.0 65.1,132.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="92.3" y1="85.0" x2="74.9" y2="85.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="62.4,85.0 74.9,79.0 74.9,91.0" fill="#fc77be"/><circle cx="92.3" cy="85.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dde` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:env_media`

```text
before   …           ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the **[[Media|media]]** available to the current cast.

## Related patterns (same book page)

- [[Media Purification]] `entity | item | vector → number`
- [[Potential Purification]] `item → number/null`

## Sources

- 📖 **Guidebook:** [[Media (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Media Reflection](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/media@hexpose:env_media)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetEnvData` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:env_media`
