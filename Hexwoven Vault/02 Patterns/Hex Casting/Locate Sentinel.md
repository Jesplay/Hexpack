---
aliases: ["Locate Sentinel"]
tags: [hexcasting/pattern, mod/hex-casting, family/other, topic/sentinels]
kind: "Pattern"
family: "Other"
topics: ["Sentinels"]
mod: "Hex Casting"
op_id: "hexcasting:sentinel/get_pos"
signature: "waeawaede"
start_direction: "EAST"
input: ""
output: "vector"
cost: "Costs a negligible amount of media."
iotas_in: []
iotas_out: ["[[Vector]]"]
entry: "[[Sentinels]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Locate Sentinel

◇ **Pattern** · [[Hex Casting]] · 📖 [[Sentinels]] · Patterns › Spells

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="52.3,106.0 95.9,106.0 139.5,106.0 117.7,68.3 139.5,30.5 95.9,30.5 52.3,30.5 74.1,68.3 52.3,106.0 30.5,68.3 52.3,30.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:436;--dur:2.8s" points="52.3,106.0 95.9,106.0 139.5,106.0 117.7,68.3 139.5,30.5 95.9,30.5 52.3,30.5 74.1,68.3 52.3,106.0 30.5,68.3 52.3,30.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.3" y1="106.0" x2="66.3" y2="106.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="76.3,106.0 66.3,110.8 66.3,101.2" fill="#fc77be"/><circle cx="52.3" cy="106.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waeawaede` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `vector` ([[Vector]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `hexcasting:sentinel/get_pos`

```text
before   …           ◂ top
after    … │ vector  ◂ top
```

## Description

Add the position of my [[Sentinels|sentinel]] to the [[The Stack|stack]], or **Null** if it isn't summoned. Costs a negligible amount of **[[Media|media]]**.

## Related patterns (same book page)

- [[Summon Sentinel]] `vector → —`
- [[Banish Sentinel]] `— → —`
- [[Wayfind Sentinel]] `vector → vector`

## Sources

- 📖 **Guidebook:** [[Sentinels]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Locate Sentinel](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/sentinels@hexcasting:sentinel/get_pos)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetSentinelPos` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:sentinel/get_pos`
