---
aliases: ["Tachygrapher's Distillation"]
tags: [hexcasting/pattern, mod/ioticblocks, family/distillation, topic/storage, topic/blocks]
kind: "Pattern"
family: "Distillation"
topics: ["Reading & Writing", "World: Blocks, Terrain & Weather"]
mod: "IoticBlocks"
op_id: "ioticblocks:read/index"
signature: "aqqqqqedwewewewdw"
start_direction: "EAST"
input: "entity | vec, num"
output: "any"
cost: ""
iotas_in: ["[[Number]]", "[[Vector]]", "[[Entity]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[ReadingWriting Blocks]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Tachygrapher's Distillation

◇ **Pattern** · [[IoticBlocks]] · 📖 [[ReadingWriting Blocks]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]] · [[World - Blocks, Terrain & Weather (Topic)|World: Blocks, Terrain & Weather]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 69.3,49.3 85.0,76.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="85.0,76.6 116.5,76.6 100.7,49.3 69.3,49.3 53.5,76.6 69.3,103.8 100.7,103.8 116.5,76.6 148.0,76.6 132.2,103.8 116.5,131.1 85.0,131.1 53.5,131.1 37.8,103.8 22.0,76.6 37.8,49.3 53.5,22.0 69.3,49.3 85.0,76.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="76.6" x2="95.1" y2="76.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="102.3,76.6 95.1,80.0 95.1,73.1" fill="#fc77be"/><circle cx="85.0" cy="76.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqqqqqedwewewewdw` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity | vec, num` ([[Number]], [[Vector]], [[Entity]])
- **Leaves:** `any` ([[Any Iota]])
- **ID:** `ioticblocks:read/index`

## Description

Read a list [[Iotas|iota]] out of an entity or block, then push the iota at the specified index to the [[The Stack|stack]].

It seems this and the next pattern may be useful for handling large amounts of data without causing a [[Mishaps|Too Many Iotas]] [[Mishaps|mishap]].

## Related patterns (same book page)

- [[Tachygrapher's Gambit]] `entity | vec, num, any → —`

## Sources

- 📖 **Guidebook:** [[ReadingWriting Blocks]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Tachygrapher's Distillation](https://ioticblocks.hexxy.media/v/1.0.2+1.20.1/1.1/en_us/#patterns/readwrite_blocks@ioticblocks:read/index)
- 🧩 **Mod page:** [IoticBlocks on Modrinth](https://modrinth.com/mod/u3PPsXzt)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadIndex` is not a `SpellAction`
- 🔧 **Pattern ID:** `ioticblocks:read/index`
