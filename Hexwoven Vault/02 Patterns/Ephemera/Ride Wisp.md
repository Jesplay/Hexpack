---
aliases: ["Ride Wisp"]
tags: [hexcasting/pattern, mod/ephemera, family/other, topic/movement, topic/wisps]
kind: "Pattern"
family: "Other"
topics: ["Movement", "Wisps & Links"]
mod: "Ephemera"
op_id: "ephemera:ridewisp"
signature: "aqadqqdaqa"
start_direction: "NORTH_WEST"
input: "wisp"
output: ""
cost: "Costs one amethyst shard."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Misc. Ephemera Patterns]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Ride Wisp

◇ **Pattern** · [[Ephemera]] · 📖 [[Misc. Ephemera Patterns]] · Patterns

**Topics:** [[Movement (Topic)|Movement]] · [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◇ Other
> This pattern has an addon-specific name that doesn't follow the usual naming scheme. See [[Pattern Naming Guide]] · [[Patterns by Family#Other|all Other patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="99" viewBox="0 0 170.0 98.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.3" cy="22.0" r="2"/><circle cx="37.8" cy="22.0" r="2"/><circle cx="69.3" cy="22.0" r="2"/><circle cx="100.7" cy="22.0" r="2"/><circle cx="132.2" cy="22.0" r="2"/><circle cx="163.7" cy="22.0" r="2"/><circle cx="22.0" cy="49.3" r="2"/><circle cx="53.5" cy="49.3" r="2"/><circle cx="85.0" cy="49.3" r="2"/><circle cx="116.5" cy="49.3" r="2"/><circle cx="148.0" cy="49.3" r="2"/><circle cx="6.3" cy="76.6" r="2"/><circle cx="37.8" cy="76.6" r="2"/><circle cx="69.3" cy="76.6" r="2"/><circle cx="100.7" cy="76.6" r="2"/><circle cx="132.2" cy="76.6" r="2"/><circle cx="163.7" cy="76.6" r="2"/></g><polyline class="hexpat-base" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 53.5,49.3 69.3,76.6 100.7,76.6 116.5,49.3 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:346;--dur:3.1s" points="53.5,49.3 37.8,22.0 22.0,49.3 37.8,76.6 53.5,49.3 69.3,76.6 100.7,76.6 116.5,49.3 132.2,76.6 148.0,49.3 132.2,22.0 116.5,49.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.5" y1="49.3" x2="48.5" y2="40.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="44.9,34.3 51.5,38.8 45.5,42.3" fill="#fc77be"/><circle cx="53.5" cy="49.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aqadqqdaqa` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `wisp` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs one amethyst shard.
- **ID:** `ephemera:ridewisp`

```text
before   … │ wisp  ◂ top
after    …         ◂ top
```

## Description

Accepts one of my wisps (or a wandering wisp), and places me atop it, similar to sitting in a minecart. I must be within the wisp's ambit, or four blocks for wandering wisps. Costs one amethyst shard.

I should take care not to accelerate too quickly when riding a wisp, as unlike with Impulse, the acceleration is not applied evenly across my body, and can thus cause significant harm. The threshold for how quickly I can accelerate safely seems to be about 98 m/s over one second.
I should take care when mounting fast wisps, as the instantaneous velocity involved can contribute to this damage if I am too far from my final seated position.

## Related patterns (same book page)

- [[Decorator's Purification]] `item frame → number`
- [[Decorator's Gambit]] `item frame, number → —`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[Entropy Reflection II]] `— → num`
- [[Clearance Purification]] `vec → bool`
- [[Flock's Reflection II]] `— → num`
- [[Flock's Purification]] `— → num`
- [[State Purification]] `vec → [string]`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[Dismount]] `— → —`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`

## Sources

- 📖 **Guidebook:** [[Misc. Ephemera Patterns]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Ride Wisp](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/misc_patterns@ephemera:ridewisp)
- 🧩 **Mod page:** [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpRideWisp` is not a `SpellAction`
- 🔧 **Pattern ID:** `ephemera:ridewisp`
