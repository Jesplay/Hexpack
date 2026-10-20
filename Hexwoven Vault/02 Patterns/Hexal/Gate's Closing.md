---
aliases: ["Gate's Closing"]
tags: [hexcasting/pattern, mod/hexal, family/other, topic/movement]
kind: "Pattern"
family: "Other"
topics: ["Movement"]
mod: "Hexal"
op_id: "hexal:gate/close"
signature: "qqqwwqqqwqqawdedw"
start_direction: "WEST"
input: "gate | gate, vec"
output: ""
cost: "Costs half an Amethyst Shard, plus an Amethyst Dust per metre transported per entity, if the gate was drifting."
iotas_in: ["[[Vector]]", "[[Gate]]"]
iotas_out: []
entry: "[[Gates]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Gate's Closing

◇ **Pattern** · [[Hexal]] · 📖 [[Gates]] · Patterns › Great Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 69.3,22.0 53.5,49.3 69.3,76.6 85.0,49.3 100.7,22.0 116.5,49.3 100.7,76.6 85.0,49.3 69.3,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:567;--dur:5.0s" points="69.3,22.0 37.8,22.0 22.0,49.3 37.8,76.6 69.3,76.6 100.7,76.6 132.2,76.6 148.0,49.3 132.2,22.0 100.7,22.0 69.3,22.0 53.5,49.3 69.3,76.6 85.0,49.3 100.7,22.0 116.5,49.3 100.7,76.6 85.0,49.3 69.3,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="69.3" y1="22.0" x2="59.2" y2="22.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="51.9,22.0 59.2,18.6 59.2,25.5" fill="#fc77be"/><circle cx="69.3" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqwwqqqwqqawdedw` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `gate | gate, vec` ([[Vector]], [[Gate]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs half an Amethyst Shard, plus an Amethyst Dust per metre transported per entity, if the gate was drifting.
- **ID:** `hexal:gate/close`

## Description

Consumes a non-drifing gate [[Iotas|iota]], or a drifting gate iota and a position. Transports all the entities marked with the passed gate to the stored or given position, as relevant. Costs half an [[Amethyst|Amethyst Shard]], plus an [[Amethyst|Amethyst Dust]] per metre transported per entity, if the gate was drifting.

## Related patterns (same book page)

- [[Gate's Reflection]] `null | vec | vec, entity → gate`
- [[Gate's Opening]] `gate, entity → —`
- [[Gate's Dismissal]] `gate, entity → —`
- [[Marked Distillation]] `gate, entity → bool`

## Sources

- 📖 **Guidebook:** [[Gates]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Gate's Closing](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/great_spells/gates@hexal:gate/close)
- 🧩 **Mod page:** [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCloseGate` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexal:gate/close`
