---
aliases: ["Audio Reflection"]
tags: [hexcasting/pattern, mod/special-efhexs, family/reflection, topic/visuals]
kind: "Pattern"
family: "Reflection"
topics: ["Visuals, Sound & UI"]
mod: "Special Efhexs"
op_id: "efhexs:get_sounds"
signature: "aeede"
start_direction: "WEST"
input: ""
output: "list of identifiers"
cost: ""
iotas_in: []
iotas_out: ["[[List]]", "[[Identifier]]"]
entry: "[[Sounds]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Audio Reflection

◇ **Pattern** · [[Special Efhexs]] · 📖 [[Sounds]] · Patterns › Spells

**Topics:** [[Visuals, Sound & UI (Topic)|Visuals, Sound & UI]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ◎ Reflection
> This pattern takes **nothing** and pushes **one** iota: its name marks it as a **Reflection**. See [[Pattern Naming Guide]] · [[Patterns by Family#Reflection|all Reflection patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="157" viewBox="0 0 170.0 156.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="35.0" cy="35.0" r="2"/><circle cx="85.0" cy="35.0" r="2"/><circle cx="135.0" cy="35.0" r="2"/><circle cx="10.0" cy="78.3" r="2"/><circle cx="60.0" cy="78.3" r="2"/><circle cx="110.0" cy="78.3" r="2"/><circle cx="160.0" cy="78.3" r="2"/><circle cx="35.0" cy="121.6" r="2"/><circle cx="85.0" cy="121.6" r="2"/><circle cx="135.0" cy="121.6" r="2"/></g><polyline class="hexpat-base" points="135.0,35.0 85.0,35.0 110.0,78.3 85.0,121.6 35.0,121.6 60.0,78.3 110.0,78.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:1.7s" points="135.0,35.0 85.0,35.0 110.0,78.3 85.0,121.6 35.0,121.6 60.0,78.3 110.0,78.3" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="135.0" y1="35.0" x2="119.0" y2="35.0" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="107.5,35.0 119.0,29.5 119.0,40.5" fill="#fc77be"/><circle cx="135.0" cy="35.0" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aeede` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `list of identifiers` ([[List]], [[Identifier]])
- **ID:** `efhexs:get_sounds`

```text
before   …                        ◂ top
after    … │ list of identifiers  ◂ top
```

## Description

Pushes the last sixteen unique sounds picked up by my [[Microphone]].

## Related patterns (same book page)

- [[Play Sound]] `id, vec, num, num → —`

## Sources

- 📖 **Guidebook:** [[Sounds]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Audio Reflection](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/sounds@efhexs:get_sounds)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpGetSounds` is not a `SpellAction`
- 🔧 **Pattern ID:** `efhexs:get_sounds`
