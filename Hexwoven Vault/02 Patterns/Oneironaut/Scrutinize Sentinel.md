---
aliases: ["Scrutinize Sentinel"]
tags: [hexcasting/pattern, mod/oneironaut, family/other, topic/sentinels]
kind: "Pattern"
family: "Other"
topics: ["Sentinels"]
mod: "Oneironaut"
op_id: "oneironaut:readsentinel"
signature: "waeawaeddwwd"
start_direction: "EAST"
input: "vec, player"
output: "num | NULL"
cost: "Costs a negligible amount of media."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: ["[[Number]]", "[[Null]]"]
entry: "[[Sentinel Detection]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Scrutinize Sentinel

◇ **Pattern** · [[Oneironaut]] · 📖 [[Sentinel Detection]] · The Great Work

**Topics:** [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 85.0,94.0 123.6,94.0 104.3,60.5 123.6,27.0 85.0,27.0 46.4,27.0 65.7,60.5 46.4,94.0 27.0,60.5 65.7,60.5 104.3,60.5 143.0,60.5 123.6,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="46.4,94.0 85.0,94.0 123.6,94.0 104.3,60.5 123.6,27.0 85.0,27.0 46.4,27.0 65.7,60.5 46.4,94.0 27.0,60.5 65.7,60.5 104.3,60.5 143.0,60.5 123.6,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="58.7" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.6,94.0 58.7,98.2 58.7,89.7" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waeawaeddwwd` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, player` ([[Vector]], [[Entity]])
- **Leaves:** `num | NULL` ([[Number]], [[Null]])
- **Cost:** Costs a negligible amount of media.
- **ID:** `oneironaut:readsentinel`

## Description

Accepts a vector and a player, and returns either the distance from the vector to the target player's sentinel or NULL. Costs a negligible amount of [[Media|media]].

Conjures a fleeting sensor at the target coordinates, which "listens" to the patterns in the ambient media to seek out the telltale signature of a sentinel.

## Sources

- 📖 **Guidebook:** [[Sentinel Detection]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Scrutinize Sentinel](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#greatwork/sentinel_detection@oneironaut:readsentinel)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadSentinel` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:readsentinel`
