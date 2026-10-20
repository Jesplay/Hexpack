---
tags: [moc, topic/movement]
---

# Movement

Moving entities around: pushes, teleports, flight, gates.

46 patterns. Click the tag `#topic/movement` to search them, or use the Pattern Browser.

## Great Spells

**[[Hex Casting|Hex Casting]]**

- [[Altiora]] `player → —`
- [[Greater Teleport]] `entity, vector → —`

**[[Hexal|Hexal]]**

- [[Gate's Reflection]] `null | vec | vec, entity → gate`

**[[Hexical|Hexical]]**

- [[Greater Blink]] `vector → —`

**[[Oneironaut|Oneironaut]]**

- [[Noetic Gateway]] `Entity, Imprint → —`

## Spells

**[[Hex Casting|Hex Casting]]**

- [[Anchorite's Flight]] `entity, number → —`
- [[Blink]] `entity, number → —`
- [[Impulse]] `entity, vector → —`
- [[Wayfarer's Flight]] `entity, number → —`

**[[Hexal|Hexal]]**

- [[Gate's Dismissal]] `gate, entity → —`
- [[Gate's Opening]] `gate, entity → —`

**[[Hexical|Hexical]]**

- [[Rotate Entity]] `entity, number, number → —`

**[[Hexodus|Hexodus]]**

- [[Alter Ballast]] `vector, number → —`
- [[Gravitate]] `entity, vector, number → —`

**[[Moves the Mind|Moves the Mind]]**

- [[Banish Creature]] `player → —`
- [[Bid Attack]] `player, num → —`
- [[Bid Drop Item]] `player → —`
- [[Bid Jumping]] `player → —`
- [[Bid Looking]] `player, vec → —`
- [[Bid Select Toolbar]] `player, num → —`
- [[Bid Sneaking]] `player, bool → —`
- [[Bid Sprinting]] `player, bool → —`
- [[Bid Strafing]] `player, num → —`
- [[Bid Swap Hands]] `player → —`
- [[Bid Use Item]] `player, num → —`
- [[Bid Walking]] `player, num → —`
- [[Halt Creature]] `player → —`
- [[Summon Creature]] `vec → —`

## Patterns

**[[Hex Casting|Hex Casting]]**

- [[Aviator's Purification]] `entity → boolean`

**[[Ephemera|Ephemera]]**

- [[Aviator's Purification II]] `player → bool | null`
- [[Aviator's Purification III]] `player → num | null`
- [[Dismount]] `— → —`
- [[Ride Wisp]] `wisp → —`
- [[Terminate Flight]] `— → —`

**[[Hexal|Hexal]]**

- [[Gate's Closing]] `gate | gate, vec → —`
- [[Marked Distillation]] `gate, entity → bool`

**[[Hexodus|Hexodus]]**

- [[Falling Purification]] `entity → vector`
- [[Falling Purification II]] `entity → vector`
- [[Grasp Purification]] `entity → number`
- [[Impact Purification]] `entity → number`
- [[Impact Purification II]] `entity → number`

**[[Hexwright|Hexwright]]**

- [[Conveyance's Reflection]] `— → [state]`
- [[Locomotion's Reflection]] `— → num, num`
- [[Speedy's Reflection]] `— → [num]`
- [[Vessel's Dismissal]] `bool → —`
- [[Vessel's Reflection]] `— → entity`

## Book pages on this topic

[[Additional Flight Patterns]] · [[Altiora (Book Entry)]] · [[Ballasts]] · [[Basic Spells]] · [[Carpet Player Manipulation]] · [[Flight]] · [[Gates]] · [[Gravitation]] · [[Greater Blink (Book Entry)]] · [[Greater Teleport (Book Entry)]] · [[Misc. Ephemera Patterns]] · [[Noetic Gateway (Book Entry)]] · [[Patterns]] · [[Rotation]]
