---
aliases: ["Particulate Reflection"]
tags: [hexcasting/pattern, mod/special-efhexs, family/reflection, topic/visuals]
kind: "Pattern"
family: "Reflection"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:get_particles"
signature: "eqqqqqaq"
start_direction: "NORTH_EAST"
input: ""
output: "list of identifiers"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Identifier]]"]
entry: "[[Particles (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Particulate Reflection

◇ **Pattern** · [[Special Efhexs]] · 📖 [[Particles (Book Entry)]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 144.6 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="29.8" r="2"/><circle cx="51.0" cy="29.8" r="2"/><circle cx="93.5" cy="29.8" r="2"/><circle cx="136.1" cy="29.8" r="2"/><circle cx="29.8" cy="66.6" r="2"/><circle cx="72.3" cy="66.6" r="2"/><circle cx="114.8" cy="66.6" r="2"/><circle cx="8.5" cy="103.4" r="2"/><circle cx="51.0" cy="103.4" r="2"/><circle cx="93.5" cy="103.4" r="2"/><circle cx="136.1" cy="103.4" r="2"/><circle cx="29.8" cy="140.2" r="2"/><circle cx="72.3" cy="140.2" r="2"/><circle cx="114.8" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="29.8,140.2 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 72.3,66.6 51.0,29.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="29.8,140.2 51.0,103.4 93.5,103.4 114.8,66.6 93.5,29.8 51.0,29.8 29.8,66.6 51.0,103.4 72.3,66.6 51.0,29.8" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="29.8" y1="140.2" x2="36.6" y2="128.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="41.5,120.0 40.6,130.8 32.5,126.1" fill="#fc77be"/><circle cx="29.8" cy="140.2" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eqqqqqaq` · **Start** `north-east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `list of identifiers` ([[List]], [[Identifier]])
- **ID:** `efhexs:get_particles`

```text
before   …                        ◂ top
after    … │ list of identifiers  ◂ top
```

## Description

Pushes the last sixteen unique particles picked up by my [[Microphone]].

## Related patterns (same book page)

- [[Conjure Particle]] `identifier, vector, vector → —`
- [[Conjure Dust]] `..., vector, number → —`
- [[Conjure Sparkly Dust]] `..., vector, vector, number → —`
- [[Conjure Debris]] `..., identifier → —`
- [[Conjure Sediment]] `..., identifier → —`
- [[Conjure Fragments]] `..., item → —`
- [[Conjure Media]] `..., vector → —`

## Sources

- 📖 **Guidebook:** [[Particles (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Particulate Reflection](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/particles@efhexs:get_particles)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetParticles` is not a `SpellAction`
- 🔧 **Pattern ID:** `efhexs:get_particles`
