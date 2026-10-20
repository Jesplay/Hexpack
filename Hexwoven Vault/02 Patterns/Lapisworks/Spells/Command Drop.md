---
aliases: ["Command Drop"]
tags: [hexcasting/spell, mod/lapisworks, family/spell, topic/wisps]
kind: "Spell"
family: "Spell"
topics: ["Wisps & Links"]
mod: "Lapisworks"
op_id: "lapisworks:take_away_poor_baby_wisp_candy_like_evil_monster"
signature: "dedwqwqwdedwqeedeeqaqdq"
start_direction: "SOUTH_WEST"
input: "wisp"
output: ""
cost: "Costs two and a half dust."
iotas_in: ["[[Entity]]"]
iotas_out: []
entry: "[[Wisp Handling]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Command Drop

✦ **Spell** · [[Lapisworks]] · 📖 [[Wisp Handling]] · Lapisworks › Interop › Lapse

**Topics:** [[Wisps & Links (Topic)|Wisps & Links]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="129" viewBox="0 0 170.0 129.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.6" r="2"/><circle cx="31.9" cy="18.6" r="2"/><circle cx="58.4" cy="18.6" r="2"/><circle cx="85.0" cy="18.6" r="2"/><circle cx="111.6" cy="18.6" r="2"/><circle cx="138.1" cy="18.6" r="2"/><circle cx="164.7" cy="18.6" r="2"/><circle cx="18.6" cy="41.6" r="2"/><circle cx="45.2" cy="41.6" r="2"/><circle cx="71.7" cy="41.6" r="2"/><circle cx="98.3" cy="41.6" r="2"/><circle cx="124.8" cy="41.6" r="2"/><circle cx="151.4" cy="41.6" r="2"/><circle cx="5.3" cy="64.6" r="2"/><circle cx="31.9" cy="64.6" r="2"/><circle cx="58.4" cy="64.6" r="2"/><circle cx="85.0" cy="64.6" r="2"/><circle cx="111.6" cy="64.6" r="2"/><circle cx="138.1" cy="64.6" r="2"/><circle cx="164.7" cy="64.6" r="2"/><circle cx="18.6" cy="87.6" r="2"/><circle cx="45.2" cy="87.6" r="2"/><circle cx="71.7" cy="87.6" r="2"/><circle cx="98.3" cy="87.6" r="2"/><circle cx="124.8" cy="87.6" r="2"/><circle cx="151.4" cy="87.6" r="2"/><circle cx="5.3" cy="110.6" r="2"/><circle cx="31.9" cy="110.6" r="2"/><circle cx="58.4" cy="110.6" r="2"/><circle cx="85.0" cy="110.6" r="2"/><circle cx="111.6" cy="110.6" r="2"/><circle cx="138.1" cy="110.6" r="2"/><circle cx="164.7" cy="110.6" r="2"/></g><polyline class="hexpat-base" points="45.2,87.6 31.9,110.6 18.6,87.6 31.9,64.6 45.2,87.6 58.4,110.6 85.0,110.6 111.6,110.6 124.8,87.6 138.1,64.6 151.4,87.6 138.1,110.6 124.8,87.6 111.6,64.6 85.0,64.6 71.7,41.6 85.0,18.6 98.3,41.6 85.0,64.6 58.4,64.6 45.2,87.6 71.7,87.6 85.0,64.6 98.3,87.6 124.8,87.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:638;--dur:6.0s" points="45.2,87.6 31.9,110.6 18.6,87.6 31.9,64.6 45.2,87.6 58.4,110.6 85.0,110.6 111.6,110.6 124.8,87.6 138.1,64.6 151.4,87.6 138.1,110.6 124.8,87.6 111.6,64.6 85.0,64.6 71.7,41.6 85.0,18.6 98.3,41.6 85.0,64.6 58.4,64.6 45.2,87.6 71.7,87.6 85.0,64.6 98.3,87.6 124.8,87.6" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.2" y1="87.6" x2="40.9" y2="95.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="37.9,100.3 38.4,93.5 43.4,96.4" fill="#fc77be"/><circle cx="45.2" cy="87.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `dedwqwqwdedwqeedeeqaqdq` · **Start** `south-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `wisp` ([[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs two and a half dust.
- **ID:** `lapisworks:take_away_poor_baby_wisp_candy_like_evil_monster`

```text
before   … │ wisp  ◂ top
after    …         ◂ top
```

## Description

Takes the item in the provided wisp (that I must own)'s offhand and lets it go into the world at the wisp's position. Costs two and a half dust.

While I was working on extensions to wisp functionality, I found that the [[Media Manipulation]] spells function the same for a wisp. Particularly strangely, [[Deposit Media]] considers wisps a valid target for depositing and [[Withdraw Media]] recharges the wisp instead of any [[Media|media]] holder the wisp is holding, and if targeting a wisp, the caster must own that wisp or suffer a [[Mishaps|mishap]].

## Related patterns (same book page)

- [[Allocate Item]] `wisp, item entity → —`

## Sources

- 📖 **Guidebook:** [[Wisp Handling]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Spell):** from the mod code: its action class `RemoveWispItem` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `lapisworks:take_away_poor_baby_wisp_candy_like_evil_monster`
