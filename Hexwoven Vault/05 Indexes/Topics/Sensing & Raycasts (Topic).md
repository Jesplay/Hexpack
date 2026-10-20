---
tags: [moc, topic/sensing]
---

# Sensing & Raycasts

Finding out about the world: positions, look direction, raycasts, nearby entities, scrying.

151 patterns. Click the tag `#topic/sensing` to search them, or use the Pattern Browser.

## Patterns

**[[Hex Casting|Hex Casting]]**

- [[Alidade's Purification]] `entity → vector`
- [[An Example]] `— → —`
- [[Archer's Distillation]] `vector, vector → vector | null`
- [[Architect's Distillation]] `vector, vector → vector | null`
- [[Aviator's Purification]] `entity → boolean`
- [[Compass' Purification]] `entity → vector`
- [[Compass' Purification II]] `entity → vector`
- [[Entity Purification]] `vector → entity or null`
- [[Entity Purification Animal]] `vector → entity or null`
- [[Entity Purification Item]] `vector → entity or null`
- [[Entity Purification Living]] `vector → entity or null`
- [[Entity Purification Monster]] `vector → entity or null`
- [[Entity Purification Player]] `vector → entity or null`
- [[Media Reflection (Hex Casting)]] `— → num`
- [[Mind's Reflection]] `— → entity | null`
- [[Pace Purification]] `entity → vector`
- [[Scout's Distillation]] `vector, vector → entity | null`
- [[Stadiometer's Purification]] `entity → num`
- [[Zone Distillation Animal]] `vector, number → list`
- [[Zone Distillation Any]] `vector, number → list`
- [[Zone Distillation Item]] `vector, number → list`
- [[Zone Distillation Living]] `vector, number → list`
- [[Zone Distillation Monster]] `vector, number → list`
- [[Zone Distillation Non-Animal]] `vector, number → list`
- [[Zone Distillation Non-Item]] `vector, number → list`
- [[Zone Distillation Non-Living]] `vector, number → list`
- [[Zone Distillation Non-Monster]] `vector, number → list`
- [[Zone Distillation Non-Player]] `vector, number → list`
- [[Zone Distillation Player]] `vector, number → list`

**[[Ephemera|Ephemera]]**

- [[Clearance Purification]] `vec → bool`
- [[Equestrian Purification]] `entity → [entity]`
- [[Equestrian Purification II]] `entity → entity`
- [[Scout's Exaltation]] `[type], vec, vec → entity | NULL`
- [[State Distillation]] `vec, string → bool|num|vec|string`
- [[State Purification]] `vec → [string]`

**[[Hexal|Hexal]]**

- [[Boxer's Purification]] `entity → num`
- [[Diver's Purification]] `entity → num`
- [[Lamplighter's Purification]] `vec → num`
- [[Nurse's Purification]] `entity → num`
- [[Squire's Purification]] `entity → num`
- [[Timekeeper's Reflection]] `— → number`

**[[Hexical|Hexical]]**

- [[Charge Reflection]] `— → number`
- [[Dodge Reflection]] `— → number`
- [[Evade Reflection]] `— → number`
- [[Leaping Reflection]] `— → number`
- [[Manipulative Reflection]] `— → number`
- [[Offensive Reflection]] `— → number`
- [[Retreat Reflection]] `— → number`
- [[Rolling Reflection]] `— → number`
- [[Send Thought]] `any → —`
- [[Shout Thought]] `any → —`
- [[Stealthy Reflection]] `— → number`
- [[Telepathic Reflection]] `— → number`

**[[Hexpose|Hexpose]]**

- [[Absorption Purification]] `entity → number`
- [[Accessory Purification]] `entity → item`
- [[Adoration Purification]] `entity → entity/null`
- [[Aegis Purification]] `entity → list of items`
- [[Ambit Purification]] `entity/vector → boolean`
- [[Anger Purification]] `entity → number`
- [[Appellation Purification]] `item → text`
- [[Artistic Purification]] `entity → identifier`
- [[Battery Purification]] `vector → number`
- [[Caliper's Purification]] `entity → number`
- [[Calorie Purification]] `identifier → number`
- [[Cart Purification]] `entity → list of items | null`
- [[Cartographer's Purification]] `identifier → vector`
- [[Charm Distillation]] `item, identifier → number`
- [[Chest Purification]] `vector → list of items | null`
- [[Circadian Reflection]] `— → number`
- [[Clearance Distillation]] `entity, identifier → number`
- [[Collector Purification]] `item → number`
- [[Concentration Distillation]] `entity, identifier → number`
- [[Condition Purification]] `identifier → number`
- [[Constructed Reflection]] `— → boolean`
- [[Culture Purification]] `entity → identifier`
- [[Demoman's Purification]] `identifier → number`
- [[Dessert Purification]] `identifier → boolean`
- [[Deterioration Purification]] `item → number`
- [[Device Reflection]] `— → boolean`
- [[Dexterity Reflection]] `— → boolean`
- [[Diagnosis Purification]] `entity → list of identifiers`
- [[Distortion Reflection]] `— → boolean`
- [[Dousing Distillation]] `vector, vector → vector/null`
- [[Edibility Purification]] `identifier → boolean`
- [[Enderman's Purification]] `entity → boolean`
- [[Epiphany Purification]] `player entity → boolean`
- [[Facet Distillation]] `vector, identifier → any`
- [[Facet Purification]] `vector → list of identifiers`
- [[Farmer's Purification]] `vector → number/null`
- [[Feline Purification]] `entity → identifier`
- [[Fitness Purification]] `entity → number`
- [[Flesh Purification]] `identifier → boolean`
- [[Fragility Purification]] `identifier → number`
- [[Geographical Purification]] `— → identifier`
- [[Glamour Purification]] `item → number`
- [[Glamour Purification II]] `item → number`
- [[Hunger Purification]] `player entity → number`
- [[Inferno Purification]] `entity → number`
- [[Item Purification]] `entity → item`
- [[Jockey Purification]] `entity → list of entities`
- [[Laser Exaltation]] `vec, vec, id → vec/null`
- [[Legacy Purification]] `item → list of text`
- [[Lilypad Distillation]] `vector, vector → vector/null`
- [[Luminance Purification]] `vector → number`
- [[Lunar Reflection]] `— → number`
- [[Lung Purification]] `entity → number`
- [[Media Purification]] `entity | item | vector → number`
- [[Media Reflection]] `— → number`
- [[Meterologist's Reflection]] `— → number`
- [[Miner's Purification]] `identifier → number`
- [[Name Purification]] `entity → display`
- [[Nurture Purification]] `identifier → identifier/null`
- [[Offer Purification]] `identifier, number → item`
- [[Orientation Purification]] `vector → vector/null`
- [[Peripheral Purification]] `vector → number/null`
- [[Plane Reflection]] `— → identifier`
- [[Pocket Reflection]] `— → list of items`
- [[Potential Purification]] `item → number/null`
- [[Prescription Purification]] `item → list of identifiers`
- [[Professional Purification]] `entity → identifier`
- [[Racer's Purification]] `entity → boolean`
- [[Railgun Exaltation]] `vec, vec, id → vec/null`
- [[Reproduction Purification]] `entity → boolean/null`
- [[Satiation Purification]] `identifier → number`
- [[Sentience Purification]] `entity → boolean`
- [[Shooter Purification]] `entity → entity/null`
- [[Showcase Purification]] `entity → number`
- [[Sloth's Purification]] `entity → boolean`
- [[Staff Reflection]] `— → boolean`
- [[Stamina Purification]] `entity entity → number`
- [[Storage Purification]] `item → number`
- [[Suffocation Purification]] `entity → number`
- [[Temporal Reflection]] `— → number`
- [[Thaumaturgist's Purification]] `item → list of identifiers`
- [[Theodolite's Purification]] `entity → vector`
- [[Tier Purification]] `entity → number`
- [[Tool Purification]] `entity → item`
- [[Vehicle Purification]] `entity → entity/null`
- [[Vitality Purification]] `entity → number`
- [[Warehouse Purification]] `identifier → number`
- [[Youth Purification]] `entity → boolean`

**[[Hextra Patterns|Hextra Patterns]]**

- [[Archer's Distillation II]] `vec, vec → vec | null`
- [[Architect's Distillation II]] `vec, vec → vec | null`
- [[Sextant Decomposition]] `entity → number, number`
- [[Sextant Purification]] `entity → number`

**[[Lapisworks|Lapisworks]]**

- [[Block Equality Distillation]] `vec, vec → bool`
- [[Block Equivalency Distillation]] `vec, vec → bool`
- [[Cleaner's Purification]] `vec, vec → bool`
- [[Empty Distillation]] `vec, vec → bool`
- [[Empty Purification]] `vec → bool`
- [[Visible Distillation]] `entity, vec → bool`

## Book pages on this topic

[[Basic Patterns]] · [[Blocks]] · [[Enlightened Patterns]] · [[Entities]] · [[Entities (Hexpose)]] · [[Environment]] · [[Flight]] · [[Food]] · [[Hex Casting 101]] · [[Hexal Basic Patterns]] · [[Hextra Basic Patterns]] · [[Items (Hexpose)]] · [[Media (Hexpose)]] · [[Misc. Ephemera Patterns]] · [[Miscellaneous (Hexpose)]] · [[Scrying]] · [[Special Raycasts]] · [[Status Effects]] · [[Telepathy]] · [[Undocumented Hexpose Patterns]] · [[Villagers]] · [[World]]
