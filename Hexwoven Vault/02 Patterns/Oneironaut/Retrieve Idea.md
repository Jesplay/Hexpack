---
aliases: ["Retrieve Idea"]
tags: [hexcasting/pattern, mod/oneironaut, family/other, topic/storage]
kind: "Pattern"
family: "Other"
topics: ["Reading & Writing"]
mod: "Oneironaut"
op_id: "oneironaut:readidea"
signature: "qwqwqwqwqwqqqwedewq"
start_direction: "WEST"
input: "key"
output: "any"
cost: "Costs one-eigth of an amethyst dust."
iotas_in: []
iotas_out: ["[[Any Iota]]"]
entry: "[[Idea Inscription]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Retrieve Idea

◇ **Pattern** · [[Oneironaut]] · 📖 [[Idea Inscription]] · Patterns

**Topics:** [[Reading & Writing (Topic)|Reading & Writing]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="153" viewBox="0 0 170.0 153.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="22.0" cy="22.0" r="2"/><circle cx="53.5" cy="22.0" r="2"/><circle cx="85.0" cy="22.0" r="2"/><circle cx="116.5" cy="22.0" r="2"/><circle cx="148.0" cy="22.0" r="2"/><circle cx="6.3" cy="49.3" r="2"/><circle cx="37.8" cy="49.3" r="2"/><circle cx="69.3" cy="49.3" r="2"/><circle cx="100.7" cy="49.3" r="2"/><circle cx="132.2" cy="49.3" r="2"/><circle cx="163.7" cy="49.3" r="2"/><circle cx="22.0" cy="76.6" r="2"/><circle cx="53.5" cy="76.6" r="2"/><circle cx="85.0" cy="76.6" r="2"/><circle cx="116.5" cy="76.6" r="2"/><circle cx="148.0" cy="76.6" r="2"/><circle cx="6.3" cy="103.8" r="2"/><circle cx="37.8" cy="103.8" r="2"/><circle cx="69.3" cy="103.8" r="2"/><circle cx="100.7" cy="103.8" r="2"/><circle cx="132.2" cy="103.8" r="2"/><circle cx="163.7" cy="103.8" r="2"/><circle cx="22.0" cy="131.1" r="2"/><circle cx="53.5" cy="131.1" r="2"/><circle cx="85.0" cy="131.1" r="2"/><circle cx="116.5" cy="131.1" r="2"/><circle cx="148.0" cy="131.1" r="2"/></g><polyline class="hexpat-base" points="85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 69.3,49.3 85.0,76.6 100.7,103.8 85.0,131.1 69.3,103.8 85.0,76.6 100.7,49.3 85.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:630;--dur:5.6s" points="85.0,22.0 53.5,22.0 37.8,49.3 22.0,76.6 37.8,103.8 53.5,131.1 85.0,131.1 116.5,131.1 132.2,103.8 148.0,76.6 132.2,49.3 116.5,22.0 85.0,22.0 69.3,49.3 85.0,76.6 100.7,103.8 85.0,131.1 69.3,103.8 85.0,76.6 100.7,49.3 85.0,22.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="85.0" y1="22.0" x2="74.9" y2="22.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="67.7,22.0 74.9,18.6 74.9,25.5" fill="#fc77be"/><circle cx="85.0" cy="22.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qwqwqwqwqwqqqwedewq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `key` (—)
- **Leaves:** `any` ([[Any Iota]])
- **Cost:** Costs one-eigth of an amethyst dust.
- **ID:** `oneironaut:readidea`

```text
before   … │ key  ◂ top
after    … │ any  ◂ top
```

## Description

Accepts a valid key, and returns the corresponding [[Iotas|iota]]. Costs one-eigth of an amethyst dust.

## Related patterns (same book page)

- [[Inscribe Idea]] `key, any → —`
- [[Metadata Purification]] `key → number`
- [[Metadata Distillation]] `key, player → bool`

## Sources

- 📖 **Guidebook:** [[Idea Inscription]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Retrieve Idea](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#patterns/idea_inscription@oneironaut:readidea)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadIdea` is not a `SpellAction`
- 🔧 **Pattern ID:** `oneironaut:readidea`
