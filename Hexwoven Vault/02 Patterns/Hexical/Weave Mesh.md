---
aliases: ["Weave Mesh"]
tags: [hexcasting/pattern, mod/hexical, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Hexical"
op_id: "hexical:weave_mesh"
signature: "qaqqqqqwqqqdeewewee"
start_direction: "EAST"
input: "entity, list of vectors"
output: ""
cost: ""
iotas_in: ["[[Vector]]", "[[Entity]]", "[[List]]"]
iotas_out: []
entry: "[[Meshes]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Weave Mesh

◇ **Pattern** · [[Hexical]] · 📖 [[Meshes]] · Patterns › Great Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="169" viewBox="0 0 170.0 168.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="24.3" cy="24.3" r="2"/><circle cx="59.0" cy="24.3" r="2"/><circle cx="93.7" cy="24.3" r="2"/><circle cx="128.4" cy="24.3" r="2"/><circle cx="163.1" cy="24.3" r="2"/><circle cx="6.9" cy="54.3" r="2"/><circle cx="41.6" cy="54.3" r="2"/><circle cx="76.3" cy="54.3" r="2"/><circle cx="111.0" cy="54.3" r="2"/><circle cx="145.7" cy="54.3" r="2"/><circle cx="24.3" cy="84.4" r="2"/><circle cx="59.0" cy="84.4" r="2"/><circle cx="93.7" cy="84.4" r="2"/><circle cx="128.4" cy="84.4" r="2"/><circle cx="163.1" cy="84.4" r="2"/><circle cx="6.9" cy="114.4" r="2"/><circle cx="41.6" cy="114.4" r="2"/><circle cx="76.3" cy="114.4" r="2"/><circle cx="111.0" cy="114.4" r="2"/><circle cx="145.7" cy="114.4" r="2"/><circle cx="24.3" cy="144.5" r="2"/><circle cx="59.0" cy="144.5" r="2"/><circle cx="93.7" cy="144.5" r="2"/><circle cx="128.4" cy="144.5" r="2"/><circle cx="163.1" cy="144.5" r="2"/></g><polyline class="hexpat-base" points="76.3,114.4 111.0,114.4 128.4,84.4 93.7,84.4 76.3,114.4 93.7,144.5 128.4,144.5 145.7,114.4 128.4,84.4 111.0,54.3 76.3,54.3 59.0,84.4 76.3,114.4 41.6,114.4 24.3,84.4 41.6,54.3 59.0,24.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:694;--dur:5.6s" points="76.3,114.4 111.0,114.4 128.4,84.4 93.7,84.4 76.3,114.4 93.7,144.5 128.4,144.5 145.7,114.4 128.4,84.4 111.0,54.3 76.3,54.3 59.0,84.4 76.3,114.4 41.6,114.4 24.3,84.4 41.6,54.3 59.0,24.3 93.7,24.3 128.4,24.3 145.7,54.3 128.4,84.4" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="76.3" y1="114.4" x2="87.4" y2="114.4" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="95.4,114.4 87.4,118.2 87.4,110.6" fill="#fc77be"/><circle cx="76.3" cy="114.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqqqqwqqqdeewewee` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, list of vectors` ([[Vector]], [[Entity]], [[List]])
- **Leaves:** `nothing` (—)
- **ID:** `hexical:weave_mesh`

```text
before   … │ entity │ list of vectors  ◂ top
after    …                             ◂ top
```

## Description

Takes a list of vectors, with constraints defined on the prior page, and weaves the mesh into that shape. Free.

## Related patterns (same book page)

- [[Conjure Mesh]] `vector → entity`
- [[Tangle Purification]] `entity → list of vectors`

## Sources

- 📖 **Guidebook:** [[Meshes]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Weave Mesh](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/great_spells/meshes@hexical:weave_mesh)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpWeaveMesh` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexical:weave_mesh`
