---
aliases: ["Apply Pseudosentinels"]
tags: [hexcasting/pattern, mod/slate-works, family/other, topic/circles, topic/sentinels]
kind: "Pattern"
family: "Other"
topics: ["Spell Circles, Loci & Engines", "Sentinels"]
mod: "Slate Works"
op_id: "slate_work:set_sents"
signature: "waeawaewawwa"
start_direction: "EAST"
input: "[vectors]"
output: ""
cost: "Costs 1 dust to command the Pseudosentinel, and due to fatigue, an extra 1/8th of a dust (stacking) for each Pseudosentinel that was already moved in the same instance."
iotas_in: ["[[Vector]]", "[[List]]"]
iotas_out: []
entry: "[[Sentinel Cache Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Apply Pseudosentinels

◇ **Pattern** · [[Slate Works]] · 📖 [[Sentinel Cache Patterns]] · Patterns › Spells

**Topics:** [[Spell Circles, Loci & Engines (Topic)|Spell Circles, Loci & Engines]] · [[Sentinels (Topic)|Sentinels]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="154" viewBox="0 0 170.0 154.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/><circle cx="27.0" cy="127.4" r="2"/><circle cx="65.7" cy="127.4" r="2"/><circle cx="104.3" cy="127.4" r="2"/><circle cx="143.0" cy="127.4" r="2"/></g><polyline class="hexpat-base" points="46.4,94.0 85.0,94.0 123.6,94.0 104.3,60.5 123.6,27.0 85.0,27.0 46.4,27.0 65.7,60.5 46.4,94.0 27.0,127.4 65.7,127.4 104.3,127.4 143.0,127.4 123.6,94.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:502;--dur:3.6s" points="46.4,94.0 85.0,94.0 123.6,94.0 104.3,60.5 123.6,27.0 85.0,27.0 46.4,27.0 65.7,60.5 46.4,94.0 27.0,127.4 65.7,127.4 104.3,127.4 143.0,127.4 123.6,94.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="46.4" y1="94.0" x2="58.7" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.6,94.0 58.7,98.2 58.7,89.7" fill="#fc77be"/><circle cx="46.4" cy="94.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `waeawaewawwa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `[vectors]` ([[Vector]], [[List]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs 1 dust to command the Pseudosentinel, and due to fatigue, an extra 1/8th of a dust (stacking) for each Pseudosentinel that was already moved in the same instance.
- **ID:** `slate_work:set_sents`

```text
before   … │ [vectors]  ◂ top
after    …              ◂ top
```

## Description

This takes a list of vectors, and attempts to move the Pseudosentinels to the positions.

Helpfully, the list of vectors from **[[Locate Pseudosentinels]]** will always match the order of **Apply Pseudosentinels**, meaning I need not worry about the ordering.

This will [[Mishaps|mishap]] if the given list is larger than the current awoken [[Sentinel Cache|Cache]] amount. Costs 1 [[Amethyst|dust]] to command the Pseudosentinel, and due to fatigue, an extra 1/8th of a [[Amethyst|dust]] (stacking) for each Pseudosentinel that was already moved in the same instance.

## Related patterns (same book page)

- [[Locate Pseudosentinels]] `— → [vectors]`

## Sources

- 📖 **Guidebook:** [[Sentinel Cache Patterns]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpSetSents` is not a `SpellAction`
- 🔧 **Pattern ID:** `slate_work:set_sents`
