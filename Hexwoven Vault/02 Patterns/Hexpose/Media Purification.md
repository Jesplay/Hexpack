---
aliases: ["Media Purification"]
tags: [hexcasting/pattern, mod/hexpose, family/purification, topic/sensing]
kind: "Pattern"
family: "Purification"
topics: ["Sensing & Raycasts"]
mod: "Hexpose"
op_id: "hexpose:get_media"
signature: "ddew"
start_direction: "WEST"
input: "entity | item | vector"
output: "number"
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]", "[[Item Stack]]"]
iotas_out: ["[[Number]]"]
entry: "[[Media (Hexpose)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Media Purification

◇ **Pattern** · [[Hexpose]] · 📖 [[Media (Hexpose)]] · Patterns › Scrying

**Topics:** [[Sensing & Raycasts (Topic)|Sensing & Raycasts]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="102" height="170" viewBox="0 0 102.0 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,66.6 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:213;--dur:1.6s" points="72.3,66.6 29.8,66.6 51.0,29.8 72.3,66.6 51.0,103.4 29.8,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="66.6" x2="58.7" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="48.9,66.6 58.7,61.9 58.7,71.3" fill="#fc77be"/><circle cx="72.3" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `ddew` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity | item | vector` ([[Vector]], [[Entity]], [[Item Stack]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexpose:get_media`

## Description

Pushes the amount of **[[Media|media]]** inside an item, that a creature contains or holds, or that a specialized **media**-holding block contains.

## Related patterns (same book page)

- [[Media Reflection]] `— → number`
- [[Potential Purification]] `item → number/null`

## Sources

- 📖 **Guidebook:** [[Media (Hexpose)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Media Purification](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/media@hexpose:get_media)
- 🧩 **Mod page:** [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetMedia` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexpose:get_media`
