---
aliases: ["Gunslinger's Roulette"]
tags: [hexcasting/pattern, mod/hex-machina, family/other, topic/combat, topic/devices]
kind: "Pattern"
family: "Other"
topics: ["Combat & Damage", "Casting Devices"]
mod: "Hex Machina"
op_id: "hex_machina:roulette"
signature: "qqqeaqaa"
start_direction: "WEST"
input: "num"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Guns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gunslinger's Roulette

◇ **Pattern** · [[Hex Machina]] · 📖 [[Guns]] · Patterns

**Topics:** [[Combat & Damage (Topic)|Combat & Damage]] · [[Casting Devices (Topic)|Casting Devices]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="157.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/><circle cx="157.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,140.2 136.1,103.4 114.8,66.6 93.5,103.4 136.1,103.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 93.5,103.4 114.8,140.2 136.1,103.4 114.8,66.6 93.5,103.4 136.1,103.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="93.5" y1="29.8" x2="79.9" y2="29.8" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="70.2,29.8 79.9,25.1 79.9,34.4" fill="#fc77be"/><circle cx="93.5" cy="29.8" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqeaqaa` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hex_machina:roulette`

```text
before   … │ num  ◂ top
after    …        ◂ top
```

## Description

Cycles §on§r slots in the gun

## Related patterns (same book page)

- [[Gunslinger's Reflection]] `— → num`
- [[Gunslinger's Prog]] `— → —`
- [[Gunslinger's Regr]] `— → —`
- [[Colt's Reflection]] `— → entity`

## Sources

- 📖 **Guidebook:** [[Guns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hex Machina on Modrinth](https://modrinth.com/mod/uQ6K0yH1)
- ✏️ **Strokes:** read from the mod code, verified against the in-game book
- 🏷️ **Type (Pattern):** from the mod code: its action class `CycleBulletWithArgsAction` is not a `SpellAction`
- 🔧 **Pattern ID:** `hex_machina:roulette`
