---
aliases: ["Tangle Purification"]
tags: [hexcasting/pattern, mod/hexical, family/purification, topic/visuals]
kind: "Pattern"
family: "Purification"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:read_mesh"
signature: "edeeeeeweeeaqqwqwqq"
start_direction: "SOUTH_WEST"
input: "entity"
output: "list of vectors"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Vector]]", "[[List]]"]
entry: "[[Meshes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Tangle Purification

◇ **Pattern** · [[Hexical]] · 📖 [[Meshes]] · Patterns › Great Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="169" viewBox="0 0 170.0 168.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/><circle cx="24.3" cy="144.5" r="2"/><circle cx="59.0" cy="144.5" r="2"/><circle cx="93.7" cy="144.5" r="2"/><circle cx="128.4" cy="144.5" r="2"/><circle cx="163.1" cy="144.5" r="2"/></g><polyline class="hexpat-base" points="128.4,84.4 111.0,114.4 76.3,114.4 93.7,84.4 128.4,84.4 145.7,114.4 128.4,144.5 93.7,144.5 76.3,114.4 59.0,84.4 76.3,54.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3 41.6,54.3 24.3,84.4 41.6,114.4 76.3,114.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:694;--dur:5.6s" points="128.4,84.4 111.0,114.4 76.3,114.4 93.7,84.4 128.4,84.4 145.7,114.4 128.4,144.5 93.7,144.5 76.3,114.4 59.0,84.4 76.3,54.3 111.0,54.3 128.4,84.4 145.7,54.3 128.4,24.3 93.7,24.3 59.0,24.3 41.6,54.3 24.3,84.4 41.6,114.4 76.3,114.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="128.4" y1="84.4" x2="122.8" y2="94.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="118.8,100.9 119.5,92.1 126.1,95.9" fill="#fc77be"/><circle cx="128.4" cy="84.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `edeeeeeweeeaqqwqwqq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `list of vectors` ([[Vector]], [[List]])
- **ID:** `hexical:read_mesh`

```text
before   … │ entity           ◂ top
after    … │ list of vectors  ◂ top
```

## Description

Pushes the shape of a mesh as a list of vectors.

## Related patterns (same book page)

- [[Conjure Mesh]] `vector → entity`
- [[Weave Mesh]] `entity, list of vectors → —`

## Sources

- 📖 **Guidebook:** [[Meshes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Tangle Purification](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/great_spells/meshes@hexical:read_mesh)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpReadMesh` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:read_mesh`
