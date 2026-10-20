---
aliases: ["Enqueue"]
tags: [hexcasting/pattern, mod/hexcassettes, family/other, topic/control-flow]
kind: "Pattern"
family: "Other"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hexcassettes"
op_id: "hexcassettes:enqueue"
signature: "qeqwqwqwqwqeqaweqqqqqwweeweweewqdwwewewwewweweww"
start_direction: "EAST"
input: "list, number"
output: "text"
cost: ""
iotas_in: ["[[Number]]", "[[List]]"]
iotas_out: ["[[String]]"]
entry: "[[Delaying Hexes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Enqueue

◇ **Pattern** · [[Hexcassettes]] · 📖 [[Delaying Hexes]] · Patterns › Spells

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="169" height="170" viewBox="0 0 168.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.9" cy="15.9" r="2"/><circle cx="38.7" cy="15.9" r="2"/><circle cx="61.5" cy="15.9" r="2"/><circle cx="84.3" cy="15.9" r="2"/><circle cx="107.1" cy="15.9" r="2"/><circle cx="129.9" cy="15.9" r="2"/><circle cx="152.6" cy="15.9" r="2"/><circle cx="4.6" cy="35.7" r="2"/><circle cx="27.3" cy="35.7" r="2"/><circle cx="50.1" cy="35.7" r="2"/><circle cx="72.9" cy="35.7" r="2"/><circle cx="95.7" cy="35.7" r="2"/><circle cx="118.5" cy="35.7" r="2"/><circle cx="141.2" cy="35.7" r="2"/><circle cx="164.0" cy="35.7" r="2"/><circle cx="15.9" cy="55.4" r="2"/><circle cx="38.7" cy="55.4" r="2"/><circle cx="61.5" cy="55.4" r="2"/><circle cx="84.3" cy="55.4" r="2"/><circle cx="107.1" cy="55.4" r="2"/><circle cx="129.9" cy="55.4" r="2"/><circle cx="152.6" cy="55.4" r="2"/><circle cx="4.6" cy="75.1" r="2"/><circle cx="27.3" cy="75.1" r="2"/><circle cx="50.1" cy="75.1" r="2"/><circle cx="72.9" cy="75.1" r="2"/><circle cx="95.7" cy="75.1" r="2"/><circle cx="118.5" cy="75.1" r="2"/><circle cx="141.2" cy="75.1" r="2"/><circle cx="164.0" cy="75.1" r="2"/><circle cx="15.9" cy="94.9" r="2"/><circle cx="38.7" cy="94.9" r="2"/><circle cx="61.5" cy="94.9" r="2"/><circle cx="84.3" cy="94.9" r="2"/><circle cx="107.1" cy="94.9" r="2"/><circle cx="129.9" cy="94.9" r="2"/><circle cx="152.6" cy="94.9" r="2"/><circle cx="4.6" cy="114.6" r="2"/><circle cx="27.3" cy="114.6" r="2"/><circle cx="50.1" cy="114.6" r="2"/><circle cx="72.9" cy="114.6" r="2"/><circle cx="95.7" cy="114.6" r="2"/><circle cx="118.5" cy="114.6" r="2"/><circle cx="141.2" cy="114.6" r="2"/><circle cx="164.0" cy="114.6" r="2"/><circle cx="15.9" cy="134.3" r="2"/><circle cx="38.7" cy="134.3" r="2"/><circle cx="61.5" cy="134.3" r="2"/><circle cx="84.3" cy="134.3" r="2"/><circle cx="107.1" cy="134.3" r="2"/><circle cx="129.9" cy="134.3" r="2"/><circle cx="152.6" cy="134.3" r="2"/><circle cx="4.6" cy="154.1" r="2"/><circle cx="27.3" cy="154.1" r="2"/><circle cx="50.1" cy="154.1" r="2"/><circle cx="72.9" cy="154.1" r="2"/><circle cx="95.7" cy="154.1" r="2"/><circle cx="118.5" cy="154.1" r="2"/><circle cx="141.2" cy="154.1" r="2"/><circle cx="164.0" cy="154.1" r="2"/></g><polyline class="hexpat-base" points="72.9,114.6 95.7,114.6 107.1,94.9 129.9,94.9 141.2,75.1 152.6,55.4 141.2,35.7 129.9,15.9 107.1,15.9 84.3,15.9 72.9,35.7 61.5,55.4 72.9,75.1 61.5,94.9 72.9,114.6 84.3,94.9 95.7,75.1 118.5,75.1 129.9,55.4 118.5,35.7 95.7,35.7 84.3,55.4 95.7,75.1 107.1,94.9 118.5,114.6 107.1,134.3 84.3,134.3 61.5,134.3 50.1,114.6 38.7,94.9 50.1,75.1 72.9,75.1 95.7,75.1 107.1,55.4 118.5,75.1 129.9,94.9 141.2,114.6 129.9,134.3 118.5,154.1 95.7,154.1 72.9,154.1 50.1,154.1 38.7,134.3 27.3,114.6 15.9,94.9 27.3,75.1 38.7,55.4 61.5,55.4 84.3,55.4 107.1,55.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:1116;--dur:6.0s" points="72.9,114.6 95.7,114.6 107.1,94.9 129.9,94.9 141.2,75.1 152.6,55.4 141.2,35.7 129.9,15.9 107.1,15.9 84.3,15.9 72.9,35.7 61.5,55.4 72.9,75.1 61.5,94.9 72.9,114.6 84.3,94.9 95.7,75.1 118.5,75.1 129.9,55.4 118.5,35.7 95.7,35.7 84.3,55.4 95.7,75.1 107.1,94.9 118.5,114.6 107.1,134.3 84.3,134.3 61.5,134.3 50.1,114.6 38.7,94.9 50.1,75.1 72.9,75.1 95.7,75.1 107.1,55.4 118.5,75.1 129.9,94.9 141.2,114.6 129.9,134.3 118.5,154.1 95.7,154.1 72.9,154.1 50.1,154.1 38.7,134.3 27.3,114.6 15.9,94.9 27.3,75.1 38.7,55.4 61.5,55.4 84.3,55.4 107.1,55.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.9" y1="114.6" x2="80.2" y2="114.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="85.4,114.6 80.2,117.1 80.2,112.1" fill="#fc77be"/><circle cx="72.9" cy="114.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qeqwqwqwqwqeqaweqqqqqwweeweweewqdwwewewwewweweww` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `list, number` ([[Number]], [[List]])
- **Leaves:** `text` ([[String]])
- **ID:** `hexcassettes:enqueue`

```text
before   … │ list │ number  ◂ top
after    … │ text           ◂ top
```

## Description

Queues a **Hex** to cast after that many twentieths of a second. Inherits the label of the current **Cassette** if possible or a randomized pattern. Label is returned.

## Other uses (same pattern, other inputs)

### [[Hexcassettes]] · 📖 [[Delaying Hexes]]

`list, number, text` → `—`

A variant that takes an additional [[Text Iota|text iota]]. Queues the **Hex** under the given label.

## Related patterns (same book page)

- [[Dequeue]] `pattern → —`
- [[Disqueue]] `— → —`
- [[Threading Reflection]] `— → number`
- [[Operating Reflection]] `— → list of text iotas`
- [[Program Distillation]] `player, text → boolean`
- [[Oracle Purification]] `text → null/number`
- [[State Reflection]] `— → null/text`

## Sources

- 📖 **Guidebook:** [[Delaying Hexes]] (same text as the in-game Hex Notebook)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpEnqueue` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcassettes:enqueue`
