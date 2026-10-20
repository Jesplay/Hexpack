---
aliases: ["Recollection"]
tags: [hexcasting/pattern, mod/hex-casting, family/escaping, topic/control-flow]
kind: "Pattern"
family: "Escaping"
topics: ["Control Flow & Meta-evaluation"]
mod: "Hex Casting"
op_id: "hexcasting:close_all_parens"
signature: "eadedae"
start_direction: "EAST"
input: ""
output: "num, list"
cost: ""
iotas_in: []
iotas_out: ["[[Number]]", "[[List]]"]
entry: "[[Advanced Escaping]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Recollection

◇ **Pattern** · [[Hex Casting]] · 📖 [[Advanced Escaping]] · Patterns

**Topics:** [[Control Flow & Meta-evaluation (Topic)|Control Flow & Meta-evaluation]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ❝ Escaping
> This pattern **controls escaping**: makes the following patterns be pushed as data instead of running: its name marks it as a **Escaping**. See [[Pattern Naming Guide]] · [[Patterns by Family#Escaping|all Escaping patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="170" height="137" viewBox="0 0 170.0 136.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="30.5" cy="30.5" r="2"/><circle cx="74.1" cy="30.5" r="2"/><circle cx="117.7" cy="30.5" r="2"/><circle cx="161.3" cy="30.5" r="2"/><circle cx="8.7" cy="68.3" r="2"/><circle cx="52.3" cy="68.3" r="2"/><circle cx="95.9" cy="68.3" r="2"/><circle cx="139.5" cy="68.3" r="2"/><circle cx="30.5" cy="106.0" r="2"/><circle cx="74.1" cy="106.0" r="2"/><circle cx="117.7" cy="106.0" r="2"/><circle cx="161.3" cy="106.0" r="2"/></g><polyline class="hexpat-base" points="30.5,30.5 74.1,30.5 95.9,68.3 117.7,30.5 139.5,68.3 117.7,106.0 95.9,68.3 74.1,106.0 30.5,106.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:349;--dur:2.2s" points="30.5,30.5 74.1,30.5 95.9,68.3 117.7,30.5 139.5,68.3 117.7,106.0 95.9,68.3 74.1,106.0 30.5,106.0" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.5" y1="30.5" x2="44.5" y2="30.5" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="54.5,30.5 44.5,35.3 44.5,25.7" fill="#fc77be"/><circle cx="30.5" cy="30.5" r="3.4" fill="#fc77be"/></svg>

**Strokes** `eadedae` · **Start** `east` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `nothing` (—)
- **Leaves:** `num, list` ([[Number]], [[List]])
- **ID:** `hexcasting:close_all_parens`

```text
before   …               ◂ top
after    … │ num │ list  ◂ top
```

## Description

Works like [[Retrospection]], but closes all open [[Introspection|Introspections]] at once. The number of layers closed, along with the list of saved patterns, are then added to the [[The Stack|stack]].

## Related patterns (same book page)

- [[Interjection]] `— → —`
- [[Meditation]] `num → —`

## Sources

- 📖 **Guidebook:** [[Advanced Escaping]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Recollection](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_escaping@hexcasting:close_all_parens)
- 🧩 **Mod page:** [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Pattern):** from the mod code: its action class `OpCloseAllParens` is not a `SpellAction`
- 🔧 **Pattern ID:** `hexcasting:close_all_parens`
