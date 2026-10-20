---
aliases: ["Meditation"]
tags: [hexcasting/pattern, mod/hex-casting, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:open_n_parens"
signature: "qdaqadq"
start_direction: "WEST"
input: "num"
output: ""
cost: ""
iotas_in: ["[[Number]]"]
iotas_out: []
entry: "[[Advanced Escaping]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Meditation

◇ **Pattern** · [[Hex Casting]] · 📖 [[Advanced Escaping]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.7" cy="30.5" r="2"/><circle cx="52.3" cy="30.5" r="2"/><circle cx="95.9" cy="30.5" r="2"/><circle cx="139.5" cy="30.5" r="2"/><circle cx="30.5" cy="68.3" r="2"/><circle cx="74.1" cy="68.3" r="2"/><circle cx="117.7" cy="68.3" r="2"/><circle cx="161.3" cy="68.3" r="2"/><circle cx="8.7" cy="106.0" r="2"/><circle cx="52.3" cy="106.0" r="2"/><circle cx="95.9" cy="106.0" r="2"/><circle cx="139.5" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="139.5,30.5 95.9,30.5 74.1,68.3 52.3,30.5 30.5,68.3 52.3,106.0 74.1,68.3 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="139.5,30.5 95.9,30.5 74.1,68.3 52.3,30.5 30.5,68.3 52.3,106.0 74.1,68.3 95.9,106.0 139.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="139.5" y1="30.5" x2="125.5" y2="30.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="115.5,30.5 125.5,25.7 125.5,35.3" fill="#fc77be"/><circle cx="139.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `qdaqadq` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `num` ([[Number]])
- **Leaves:** `nothing` (—)
- **ID:** `hexcasting:open_n_parens`

```text
before   … │ num  ◂ top
after    …        ◂ top
```

## Description

Removes a number from the [[The Stack|stack]], then acts as though I'd drawn that many [[Introspection|Introspections]]. Treated as a normal pattern while [[Introspection|introspecting]], so nesting this may require [[Consideration|tricks]] to draw extra [[Retrospection|Retrospections]].

## Related patterns (same book page)

- [[Interjection]] `— → —`
- [[Recollection]] `— → num, list`

## Sources

- 📖 **Guidebook:** [[Advanced Escaping]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Meditation](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_escaping@hexcasting:open_n_parens)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpOpenNParens` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:open_n_parens`
