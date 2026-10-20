---
aliases: ["Akasha's Distillation"]
tags: [hexcasting/pattern, mod/hex-casting, family/distillation, topic/storage]
kind: "Pattern"
family: "Distillation"
topics: ["Reading & Writing"]
mod: "Hex Casting"
op_id: "hexcasting:akashic/read"
signature: "qqqwqqqqqaq"
start_direction: "WEST"
input: "vector, pattern"
output: "any"
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Pattern]]"]
iotas_out: ["[[Any Iota]]"]
entry: "[[Akashic Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Akasha's Distillation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Akashic Patterns]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◑ Distillation
> This pattern takes **two** iotas and gives back **one**: its name marks it as a **Distillation**. See [[Pattern Naming Guide]] · [[Patterns by Family#Distillation|all Distillation patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="121" viewBox="0 0 170.0 121.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="7.7" cy="27.0" r="2"/><circle cx="46.4" cy="27.0" r="2"/><circle cx="85.0" cy="27.0" r="2"/><circle cx="123.6" cy="27.0" r="2"/><circle cx="162.3" cy="27.0" r="2"/><circle cx="27.0" cy="60.5" r="2"/><circle cx="65.7" cy="60.5" r="2"/><circle cx="104.3" cy="60.5" r="2"/><circle cx="143.0" cy="60.5" r="2"/><circle cx="7.7" cy="94.0" r="2"/><circle cx="46.4" cy="94.0" r="2"/><circle cx="85.0" cy="94.0" r="2"/><circle cx="123.6" cy="94.0" r="2"/><circle cx="162.3" cy="94.0" r="2"/></g><polyline class="hexpat-base" points="85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:3.4s" points="85.0,27.0 46.4,27.0 27.0,60.5 46.4,94.0 85.0,94.0 123.6,94.0 143.0,60.5 123.6,27.0 85.0,27.0 65.7,60.5 85.0,94.0 104.3,60.5 85.0,27.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="27.0" x2="72.6" y2="27.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="63.8,27.0 72.6,22.8 72.6,31.3" fill="#fc77be"/><circle cx="85.0" cy="27.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qqqwqqqqqaq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vector, pattern` ([[Vector]], [[Pattern]])
- **Leaves:** `any` ([[Any Iota]])
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexcasting:akashic/read`

```text
before   … │ vector │ pattern  ◂ top
after    … │ any               ◂ top
```

## Description

Read the [[Iotas|iota]] associated with the given pattern out of the [[Akashic Libraries|Akashic Library]] with its [[Akashic Libraries|Record]] at the given position. This has no range limit. Costs about one [[Amethyst|Amethyst Dust]].

## Related patterns (same book page)

- [[Akasha's Gambit]] `vector, pattern, any → —`

## Sources

- 📖 **Guidebook:** [[Akashic Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Akasha's Distillation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/akashic_patterns@hexcasting:akashic/read)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpAkashicRead` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:akashic/read`
