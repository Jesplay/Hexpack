---
aliases: ["Blink"]
tags: [hexcasting/spell, mod/hex-casting, family/spell, topic/movement]
kind: "Spell"
family: "Spell"
topics: ["Movement"]
mod: "Hex Casting"
op_id: "hexcasting:blink"
signature: "awqqqwaq"
start_direction: "SOUTH_WEST"
input: "entity, number"
output: ""
cost: "Costs about one Amethyst Shard per two blocks travelled."
iotas_in: ["[[Number]]", "[[Entity]]"]
iotas_out: []
entry: "[[Basic Spells]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "pattern"
in_book: true
---

# Blink

✦ **Spell** · [[Hex Casting]] · 📖 [[Basic Spells]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 95.9,68.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:392;--dur:2.5s" points="52.3,68.3 30.5,106.0 74.1,106.0 117.7,106.0 139.5,68.3 117.7,30.5 74.1,30.5 30.5,30.5 52.3,68.3 95.9,68.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="52.3" y1="68.3" x2="45.3" y2="80.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="40.3,89.0 41.2,77.9 49.5,82.7" fill="#fc77be"/><circle cx="52.3" cy="68.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `awqqqwaq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity, number` ([[Number]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Shard per two blocks travelled.
- **ID:** `hexcasting:blink`

```text
before   … │ entity │ number  ◂ top
after    …                    ◂ top
```

## Description

Remove an entity and length from the [[The Stack|stack]], then teleport the given entity along its look vector by the given length.
Costs about one [[Amethyst|Amethyst Shard]] per two blocks travelled.

## Example

[[Mind's Reflection]] → [[Numerical Reflection]] `5` → **Blink** teleports you 5 blocks forward, in the direction you're looking.

## Related patterns (same book page)

- [[Explosion]] `vector, number → —`
- [[Fireball]] `vector, number → —`
- [[Impulse]] `entity, vector → —`
- [[Make Note]] `vector, number, number → —`

## Sources

- 📖 **Guidebook:** [[Basic Spells]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Blink](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/basic@hexcasting:blink)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpBlink` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:blink`
