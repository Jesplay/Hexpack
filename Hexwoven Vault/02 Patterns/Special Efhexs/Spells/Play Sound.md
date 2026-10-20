---
aliases: ["Play Sound"]
tags: [hexcasting/spell, mod/special-efhexs, family/spell, topic/visuals]
kind: "Spell"
family: "Spell"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:play_sound"
signature: "qaqqd"
start_direction: "WEST"
input: "id, vec, num, num"
output: ""
cost: "Costs about a sixteenth of an Amethyst Dust."
iotas_in: ["[[Number]]", "[[Vector]]", "[[Identifier]]"]
iotas_out: []
entry: "[[Sounds]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Play Sound

✦ **Spell** · [[Special Efhexs]] · 📖 [[Sounds]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="110.0,78.3 60.0,78.3 35.0,121.6 85.0,121.6 110.0,78.3 85.0,35.0 135.0,35.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="110.0,78.3 60.0,78.3 35.0,121.6 85.0,121.6 110.0,78.3 85.0,35.0 135.0,35.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="110.0" y1="78.3" x2="94.0" y2="78.3" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="82.5,78.3 94.0,72.8 94.0,83.8" fill="#fc77be"/><circle cx="110.0" cy="78.3" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qaqqd` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `id, vec, num, num` ([[Number]], [[Vector]], [[Identifier]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about a sixteenth of an Amethyst Dust.
- **ID:** `efhexs:play_sound`

```text
before   … │ id │ vec │ num │ num  ◂ top
after    …                         ◂ top
```

## Description

Plays a sound at a particular location, volume, and pitch. Costs about a sixteenth of an [[Amethyst|Amethyst Dust]].

Volume and pitch are all numbers that range between 0 and 2. Volume controls both loudness and distance that sound can be heard. Meanwhile, pitch controls the "speed" the sound is played.

The sound is entirely disembodied to any body or event. It seems that the spell essentially uses tiny bits of **[[Media|media]]** manipulation to vibrate the air precisely into a given sound.

## Related patterns (same book page)

- [[Audio Reflection]] `— → list of identifiers`

## Sources

- 📖 **Guidebook:** [[Sounds]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Play Sound](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/sounds@efhexs:play_sound)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpPlaySound` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `efhexs:play_sound`
