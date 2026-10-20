---
aliases: ["Wayfind Sentinel"]
tags: [hexcasting/pattern, mod/hex-casting, family/other, topic/sentinels]
kind: "Pattern"
family: "Other"
topics: ["Sentinels"]
mod: "Hex Casting"
op_id: "hexcasting:sentinel/wayfind"
signature: "waeawaedwa"
start_direction: "EAST"
input: "vector"
output: "vector"
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]"]
iotas_out: ["[[Vector]]"]
entry: "[[Sentinels]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Wayfind Sentinel

◇ **Pattern** · [[Hex Casting]] · 📖 [[Sentinels]] · Patterns › Spells

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="109" viewBox="0 0 170.0 108.7"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.3" r="2"/><circle cx="41.6" cy="24.3" r="2"/><circle cx="76.3" cy="24.3" r="2"/><circle cx="111.0" cy="24.3" r="2"/><circle cx="145.7" cy="24.3" r="2"/><circle cx="24.3" cy="54.3" r="2"/><circle cx="59.0" cy="54.3" r="2"/><circle cx="93.7" cy="54.3" r="2"/><circle cx="128.4" cy="54.3" r="2"/><circle cx="163.1" cy="54.3" r="2"/><circle cx="6.9" cy="84.4" r="2"/><circle cx="41.6" cy="84.4" r="2"/><circle cx="76.3" cy="84.4" r="2"/><circle cx="111.0" cy="84.4" r="2"/><circle cx="145.7" cy="84.4" r="2"/></g><polyline class="hexpat-base" points="76.3,84.4 111.0,84.4 145.7,84.4 128.4,54.3 145.7,24.3 111.0,24.3 76.3,24.3 93.7,54.3 76.3,84.4 59.0,54.3 41.6,24.3 24.3,54.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:382;--dur:3.1s" points="76.3,84.4 111.0,84.4 145.7,84.4 128.4,54.3 145.7,24.3 111.0,24.3 76.3,24.3 93.7,54.3 76.3,84.4 59.0,54.3 41.6,24.3 24.3,54.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="84.4" x2="87.4" y2="84.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.4,84.4 87.4,88.2 87.4,80.6" fill="#fc77be"/><circle cx="76.3" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waeawaedwa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector` ([[Vector]])
- **Leaves:** `vector` ([[Vector]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexcasting:sentinel/wayfind`

```text
before   … │ vector  ◂ top
after    … │ vector  ◂ top
```

## Description

Transform the position vector on the top of the [[The Stack|stack]] into a unit vector pointing from that position to my [[Sentinels|sentinel]], or **Null** if it isn't summoned. Costs a negligible amount of **[[Media|media]]**.

## Related patterns (same book page)

- [[Summon Sentinel]] `vector → —`
- [[Banish Sentinel]] `— → —`
- [[Locate Sentinel]] `— → vector`

## Sources

- 📖 **Guidebook:** [[Sentinels]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Wayfind Sentinel](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/sentinels@hexcasting:sentinel/wayfind)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetSentinelWayfind` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:sentinel/wayfind`
