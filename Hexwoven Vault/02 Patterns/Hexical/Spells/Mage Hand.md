---
aliases: ["Mage Hand"]
tags: [hexcasting/spell, mod/hexical, family/spell, topic/items]
kind: "Spell"
family: "Spell"
topics: ["Items & Crafting"]
mod: "Hexical"
op_id: "hexical:mage_hand"
signature: "aaqqaeea"
start_direction: "WEST"
input: "entity/vector"
output: ""
cost: "Costs about one Amethyst Dust."
iotas_in: ["[[Vector]]", "[[Entity]]"]
iotas_out: []
entry: "[[Wristpocket (Book Entry)]]"
needs_enlightenment: false
per_world: false
kind_source: "code"
topic_source: "page"
in_book: true
---

# Mage Hand

✦ **Spell** · [[Hexical]] · 📖 [[Wristpocket (Book Entry)]] · Patterns › Spells

**Topics:** [[Items & Crafting (Topic)|Items & Crafting]]

**Unlock:** ✅ available from the start: just draw it

> [!abstract] ✦ Spell
> This spell **changes the world** and costs media. See [[Pattern Naming Guide]] · [[Patterns by Family#Spell|all Spell patterns]].

## How to draw it

<svg class="hexpat" xmlns="http://www.w3.org/2000/svg" width="166" height="170" viewBox="0 0 165.8 170.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="29.8" cy="29.8" r="2"/><circle cx="72.3" cy="29.8" r="2"/><circle cx="114.8" cy="29.8" r="2"/><circle cx="157.3" cy="29.8" r="2"/><circle cx="8.5" cy="66.6" r="2"/><circle cx="51.0" cy="66.6" r="2"/><circle cx="93.5" cy="66.6" r="2"/><circle cx="136.1" cy="66.6" r="2"/><circle cx="29.8" cy="103.4" r="2"/><circle cx="72.3" cy="103.4" r="2"/><circle cx="114.8" cy="103.4" r="2"/><circle cx="157.3" cy="103.4" r="2"/><circle cx="8.5" cy="140.2" r="2"/><circle cx="51.0" cy="140.2" r="2"/><circle cx="93.5" cy="140.2" r="2"/><circle cx="136.1" cy="140.2" r="2"/></g><polyline class="hexpat-base" points="136.1,66.6 93.5,66.6 114.8,103.4 136.1,66.6 114.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,140.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:383;--dur:2.5s" points="136.1,66.6 93.5,66.6 114.8,103.4 136.1,66.6 114.8,29.8 72.3,29.8 93.5,66.6 72.3,103.4 29.8,103.4 51.0,140.2" fill="none" stroke="#b38ef3" stroke-width="4" stroke-linejoin="round" stroke-linecap="round"/><line x1="136.1" y1="66.6" x2="122.5" y2="66.6" stroke="#fc77be" stroke-width="2.4" stroke-linecap="round"/><polygon points="112.7,66.6 122.5,61.9 122.5,71.3" fill="#fc77be"/><circle cx="136.1" cy="66.6" r="3.4" fill="#fc77be"/></svg>

**Strokes** `aaqqaeea` · **Start** `west` · *start at the pink dot, first stroke along the arrow; any rotation works*

## Stack effect

- **Takes:** `entity/vector` ([[Vector]], [[Entity]])
- **Leaves:** `nothing` (—)
- **Cost:** Costs about one Amethyst Dust.
- **ID:** `hexical:mage_hand`

```text
before   … │ entity/vector  ◂ top
after    …                  ◂ top
```

## Description

Projects my mind's hand forward to use my wristpocketed item and interact with the world. If my wristpocket is empty, acts as though a plain hand had reached out. Costs about one [[Amethyst|Amethyst Dust]].

If given an entity, it interacts with the entity using the wristpocketed item. If given a vector, it interacts with the block or *space* with the item.

Because it only brings forth a mind's hand, it is still tied to me and will not work for casting methods not attached to a player. I can however use it to open **Chests**.

*A spectral, floating hand appears at a point you choose within range.*

## Related patterns (same book page)

- [[Wristpocket]] `— → —`
- [[Wristpocket Reflection]] `— → item`
- [[Sleight]] `item entity/vector → —`
- [[Mage Mouth]] `— → —`

## Sources

- 📖 **Guidebook:** [[Wristpocket (Book Entry)]] (same text as the in-game Hex Notebook)
- 🌐 **Official online docs:** [Mage Hand](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/wristpocket@hexical:mage_hand)
- ✏️ **Strokes:** read from the mod code, matches the official docs ✔
- 🏷️ **Type (Spell):** from the mod code: its action class `OpMageHand` implements Hex Casting's `SpellAction`
- 🔧 **Pattern ID:** `hexical:mage_hand`
