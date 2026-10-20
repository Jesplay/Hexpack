---
aliases: ["Impact Purification"]
tags: [hexcasting/pattern, mod/hexodus, family/purification, topic/movement]
kind: "Pattern"
family: "Purification"
topics: ["Movement"]
mod: "Hexodus"
op_id: "hexodus:get_strength"
signature: "wdwdwedewdw"
start_direction: "NORTH_WEST"
input: "entity"
output: "number"
cost: ""
iotas_in: ["[[Entity]]"]
iotas_out: ["[[Number]]"]
entry: "[[Gravitation]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Impact Purification

◇ **Pattern** · [[Hexodus]] · 📖 [[Gravitation]] · Patterns › Spells

**Topics:** [[Movement (Topic)|Movement]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◐ Purification
> This pattern takes **one** iota and gives back **one**: its name marks it as a **Purification**. See [[Pattern Naming Guide]] · [[Patterns by Family#Purification|all Purification patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="72.3,103.4 51.0,66.6 29.8,29.8 72.3,29.8 114.8,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,66.6 93.5,66.6 136.1,66.6 114.8,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:510;--dur:3.4s" points="72.3,103.4 51.0,66.6 29.8,29.8 72.3,29.8 114.8,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,66.6 93.5,66.6 136.1,66.6 114.8,103.4 93.5,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.3" y1="103.4" x2="65.5" y2="91.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="60.6,83.2 69.5,89.3 61.4,94.0" fill="#fc77be"/><circle cx="72.3" cy="103.4" r="3.4" fill="#fc77be"/></svg>

**Strokes** `wdwdwedewdw` · **Start** `north-west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity` ([[Entity]])
- **Leaves:** `number` ([[Number]])
- **ID:** `hexodus:get_strength`

```text
before   … │ entity  ◂ top
after    … │ number  ◂ top
```

## Description

Pushes the scaling factor of [[Gravitate]]'s gravity compared to baseline, or Null if the entity is not under the effects of [[Gravitate]].

I have discovered some theorywork about **Gravity**. It appears every **Gravity**-generating phenomenon has a numerical *priority*. The direction **Gravity** pulls a given entity along is dictated by whichever force has most priority upon that entity.

The **Gravity** created by [[Gravitate]] is notably rather weak; other sources of **Gravity** often overpower it. I can use this to anchor myself against hostile **Hexcasters**.

Gravitational strength is settled through a much more democratic process. Each source of **Gravity** declares some scaling factor. For [[Gravitate]], that is a number between and including 0 and 5.

When pulling, **Gravity** multiplies all the provided scaling factors together to determine the force with which to pull.

On the next few pages are some patterns used to measure "true" gravity.

## Related patterns (same book page)

- [[Gravitate]] `entity, vector, number → —`
- [[Grasp Purification]] `entity → number`
- [[Falling Purification]] `entity → vector`
- [[Falling Purification II]] `entity → vector`
- [[Impact Purification II]] `entity → number`

## Sources

- 📖 **Guidebook:** [[Gravitation]] (same text as the in-game Hex Notebook)
- 🧩 **Mod page:** [Hexodus on Modrinth](https://modrinth.com/mod/bN6IKmdE)
- ✏️ **Strokes:** read from the mod code
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetGravity` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexodus:get_strength`
