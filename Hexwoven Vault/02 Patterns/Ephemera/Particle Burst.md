---
aliases: ["Particle Burst"]
tags: [hexcasting/pattern, mod/ephemera, family/other, topic/visuals]
kind: "Pattern"
family: "Other"
topics: ["Visuals, Sound & UI"]
mod: "Ephemera"
op_id: "ephemera:particleburst"
signature: "deeeewaaddwqqqqa"
start_direction: "EAST"
input: "vec, vec, num, num"
output: ""
cost: "Costs a negligible amount of media."
iotas_in: ["[[Number]]", "[[Vector]]"]
iotas_out: []
entry: "[[Particle Burst (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Particle Burst

◇ **Pattern** · [[Ephemera]] · 📖 [[Particle Burst (Book Entry)]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="145" height="170" viewBox="0 0 145.4 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.8" cy="20.8" r="2"/><circle cx="50.4" cy="20.8" r="2"/><circle cx="80.1" cy="20.8" r="2"/><circle cx="109.8" cy="20.8" r="2"/><circle cx="139.4" cy="20.8" r="2"/><circle cx="5.9" cy="46.5" r="2"/><circle cx="35.6" cy="46.5" r="2"/><circle cx="65.3" cy="46.5" r="2"/><circle cx="94.9" cy="46.5" r="2"/><circle cx="124.6" cy="46.5" r="2"/><circle cx="20.8" cy="72.2" r="2"/><circle cx="50.4" cy="72.2" r="2"/><circle cx="80.1" cy="72.2" r="2"/><circle cx="109.8" cy="72.2" r="2"/><circle cx="139.4" cy="72.2" r="2"/><circle cx="5.9" cy="97.8" r="2"/><circle cx="35.6" cy="97.8" r="2"/><circle cx="65.3" cy="97.8" r="2"/><circle cx="94.9" cy="97.8" r="2"/><circle cx="124.6" cy="97.8" r="2"/><circle cx="20.8" cy="123.5" r="2"/><circle cx="50.4" cy="123.5" r="2"/><circle cx="80.1" cy="123.5" r="2"/><circle cx="109.8" cy="123.5" r="2"/><circle cx="139.4" cy="123.5" r="2"/><circle cx="5.9" cy="149.2" r="2"/><circle cx="35.6" cy="149.2" r="2"/><circle cx="65.3" cy="149.2" r="2"/><circle cx="94.9" cy="149.2" r="2"/><circle cx="124.6" cy="149.2" r="2"/></g><polyline class="hexpat-base" points="50.4,123.5 80.1,123.5 65.3,149.2 35.6,149.2 20.8,123.5 35.6,97.8 65.3,97.8 94.9,97.8 80.1,72.2 65.3,97.8 50.4,72.2 80.1,72.2 109.8,72.2 124.6,46.5 109.8,20.8 80.1,20.8 65.3,46.5 94.9,46.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:504;--dur:4.8s" points="50.4,123.5 80.1,123.5 65.3,149.2 35.6,149.2 20.8,123.5 35.6,97.8 65.3,97.8 94.9,97.8 80.1,72.2 65.3,97.8 50.4,72.2 80.1,72.2 109.8,72.2 124.6,46.5 109.8,20.8 80.1,20.8 65.3,46.5 94.9,46.5" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.4" y1="123.5" x2="59.9" y2="123.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="66.8,123.5 59.9,126.8 59.9,120.3" fill="#fc77be"/><circle cx="50.4" cy="123.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `deeeewaaddwqqqqa` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `vec, vec, num, num` ([[Number]], [[Vector]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs a negligible amount of media.
- **ID:** `ephemera:particleburst`

```text
before   … │ vec │ vec │ num │ num  ◂ top
after    …                          ◂ top
```

## Description

Accepts two vectors and two numbers, and uses them to define a burst of particles. Costs a negligible amount of [[Media|media]].

The first vector determines the burst's origin position, and the second vector determines its direction and speed. The first number determines the scale of random numbers added to each position component, and the second does the same for the direction vector.

Mod author's note for Forge users: This spell does not work on Forge. I do not know why. I've set the cost on Forge to zero so at least it won't waste media. Sorry.

I'll try to get it working eventually.

## Sources

- 📖 **Guidebook:** [[Particle Burst (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Particle Burst](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/particle_burst@ephemera:particleburst)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpParticleBurst` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:particleburst`
