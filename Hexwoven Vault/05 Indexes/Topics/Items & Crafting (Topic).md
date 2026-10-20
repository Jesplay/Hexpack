---
tags: [moc, topic/items]
---

# Items & Crafting

Item manipulation, motes, crafting casting items, recharging.

116 patterns. Click the tag `#topic/items` to search them, or use the Pattern Browser.

## Great Spells

**[[Hex Casting|Hex Casting]]**

- [[Craft Phial]] `entity → —`

**[[Ephemera|Ephemera]]**

- [[Induce Mending]] `entity → —`

**[[Lapisworks|Lapisworks]]**

- [[Indigan Lapidary]] `— → —`

**[[Oneironaut|Oneironaut]]**

- [[Media Infusion]] `Vector → —`

**[[Slate Works|Slate Works]]**

- [[Induce Restock]] `vector → —`

## Spells

**[[Hex Casting|Hex Casting]]**

- [[Caster's Glamour]] `— → —`
- [[Craft Artifact]] `entity, list → —`
- [[Craft Cypher]] `entity, list → —`
- [[Craft Trinket]] `entity, list → —`
- [[Erase Item]] `— → —`
- [[Recharge Item]] `entity → —`

**[[HexDebug|HexDebug]]**

- [[Craft Debugger]] `entity, [pattern] → —`
- [[Craft Quenched Debugger]] `entity, [pattern] → —`

**[[Hexal|Hexal]]**

- [[Bind Storage]] `vec → —`
- [[Bind Storage - Temporary]] `vec → —`
- [[Smelt]] `vec | entity | mote → —`

**[[Hexchanting|Hexchanting]]**

- [[Imbue Equipment]] `[pattern] → —`

**[[Hexic|Hexic]]**

- [[Etch Pattern]] `pattern, str?, entity, iota → —`
- [[Tie Mediaweave]] `— → —`
- [[Untie Mediaweave]] `— → —`

**[[Hexical|Hexical]]**

- [[Age Scroll]] `entity → —`
- [[Autograph]] `— → —`
- [[Conjure Hexburst]] `vector, any → —`
- [[Conjure Hextito]] `vector, list of patterns → —`
- [[Describe Item]] `entity, list of text/null → —`
- [[Dye Ink]] `entity, vector → —`
- [[Hex Gummy]] `vector → —`
- [[Illuminate Ink]] `entity → —`
- [[Mage Hand]] `entity/vector → —`
- [[Mage Mouth]] `— → —`
- [[Name Item]] `entity, text/null → —`
- [[Restore Scroll]] `entity → —`
- [[Sleight]] `item entity/vector → —`
- [[Unautograph]] `— → —`
- [[Vanish Scroll]] `entity → —`
- [[Wristpocket]] `— → —`

**[[Lapisworks|Lapisworks]]**

- [[Disenchant]] `— → —`
- [[Imbue with Amel]] `num → —`

**[[Oneironaut|Oneironaut]]**

- [[Craft Bottomless Trinket]] `list of patterns → —`

**[[Slate Works|Slate Works]]**

- [[Exchange Mind]] `vector, entity → —`
- [[Lay Item]] `entity → —`

**[[YAHA|YAHA]]**

- [[Bury Item]] `vector, entity → —`
- [[Pluck]] `entity → —`

## Patterns

**[[Ephemera|Ephemera]]**

- [[Decorator's Gambit]] `item frame, number → —`
- [[Decorator's Purification]] `item frame → number`
- [[Menderbug's Reflection]] `— → num`
- [[Menderbug's Reflection II]] `— → num`

**[[Hexal|Hexal]]**

- [[Capacity Reflection]] `— → int`
- [[Containment Distillation]] `vec, (itemtype | mote) → bool`
- [[Craft]] `mote | [mote] | [[mote]] → [mote]`
- [[Depot Gambit]] `mote, vec → mote`
- [[Depot Purification]] `mote → vec`
- [[Mediafy Item]] `item | item, mote → mote`
- [[Offering Purification]] `villager → [complicated!]`
- [[Preview Craft]] `item | [item] | [[item]] → [item]`
- [[Return Item]] `mote, vec | mote, vec, num → —`
- [[Seniority Purification]] `villager → int`
- [[Splitting Gambit]] `mote, int → mote, mote`
- [[Stacking Distillation II]] `mote, (mote | item) → bool`
- [[Stocktake Purification]] `mote | itemtype → [mote]`
- [[Stocktake Reflection]] `— → [itemtype]`
- [[Trade]] `villager, [mote], int? → item`
- [[Use Item On]] `mote, (entity | vec, vec) → —`

**[[Hexical|Hexical]]**

- [[Authenticator's Distillation]] `item stack, entity → boolean`
- [[Handy Reflection]] `— → number`
- [[Hopper]] `source, num?, dest., num? → —`
- [[Hopper Purification]] `source → list`
- [[Switch]] `number → —`
- [[Wristpocket Reflection]] `— → item`

**[[Hexpose|Hexpose]]**

- [[Accessory Purification]] `entity → item`
- [[Aegis Purification]] `entity → list of items`
- [[Appellation Purification]] `item → text`
- [[Bibliography Purification]] `item → text, number`
- [[Cart Purification]] `entity → list of items | null`
- [[Charm Distillation]] `item, identifier → number`
- [[Chest Purification]] `vector → list of items | null`
- [[Collector Purification]] `item → number`
- [[Deterioration Purification]] `item → number`
- [[Fragility Purification]] `identifier → number`
- [[Glamour Purification]] `item → number`
- [[Glamour Purification II]] `item → number`
- [[Item Purification]] `entity → item`
- [[Legacy Purification]] `item → list of text`
- [[Offer Purification]] `identifier, number → item`
- [[Pocket Reflection]] `— → list of items`
- [[Reading Purification]] `item → list of text`
- [[Storage Purification]] `item → number`
- [[Thaumaturgist's Purification]] `item → list of identifiers`
- [[Tool Purification]] `entity → item`
- [[Warehouse Purification]] `identifier → number`

**[[Hexwright|Hexwright]]**

- [[Aspect Distillation]] `vec, num → num`
- [[Cantor's Gambit]] `slot num, any → —`
- [[Cantor's Reflection]] `slot num → any`
- [[Essence Purification]] `vec → [string | null]`

**[[Lapisworks|Lapisworks]]**

- [[Epigrapher's Reflection]] `— → bool`
- [[Gemologist's Reflection]] `— → bool`
- [[Jeweller's Gambit]] `any → —`
- [[Jeweller's Reflection]] `— → any`

**[[MoreIotas|MoreIotas]]**

- [[Collector's Distillation]] `vector, vector → list`
- [[Duelist's Purification]] `entity → item stack`
- [[Hoarder's Distillation]] `vector, vector → list`
- [[Shieldbearer's Purification]] `entity → item stack`
- [[Treasurer's Distillation]] `itemtype|item stack, int → item stack`

**[[Oneironaut|Oneironaut]]**

- [[Sign Item]] `— → —`
- [[Signature Purification]] `soulprint → bool`
- [[Soulprint Reflection]] `— → soulprint`

**[[Slate Works|Slate Works]]**

- [[Bind Macro]] `vec, any iota, pattern → —`
- [[Check Item]] `[patterns] → bool`
- [[Get Craft Count]] `vec → number`
- [[Get Vessels]] `— → [vecs]`
- [[Merchant's Purification]] `vec → complicated`
- [[Reawaken Item]] `[patterns] → —`
- [[Reorient Items]] `— → —`
- [[Review Broadcast]] `vec → any`
- [[Set Craft Count]] `vec, number → —`
- [[Set Recipe]] `vec, [item stack|item type|null] → —`

## Book pages on this topic

[[Animated Scrolls]] · [[Armour & Robes]] · [[Autographs]] · [[Better Media Farming]] · [[Caster's Glamour (Book Entry)]] · [[Conjurable Delights]] · [[Craft Phial (Book Entry)]] · [[Crafting Casting Items]] · [[Debugging Patterns]] · [[Endless Pouch]] · [[Hexal Spells]] · [[Hopper (Book Entry)]] · [[Hotbar]] · [[Imbue Amel]] · [[Imbuing Equipment]] · [[Indigan Lapidary (Book Entry)]] · [[Induce Mending (Book Entry)]] · [[Item Renaming]] · [[Items]] · [[Items (Hexpose)]] · [[Locus Spells]] · [[Media Infusion (Book Entry)]] · [[Mediaweave Tying]] · [[Merchant Directrix Patterns]] · [[Misc. Ephemera Patterns]] · [[Motes]] · [[Pattern Etching]] · [[Patterned Assembler Patterns]] · [[Trinkets]] · [[Undocumented Oneironaut Patterns]] · [[Wristpocket (Book Entry)]] · [[Yaha Spells]]
