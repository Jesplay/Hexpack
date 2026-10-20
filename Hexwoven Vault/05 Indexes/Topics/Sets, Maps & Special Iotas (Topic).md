---
tags: [moc, topic/data-structures]
---

# Sets, Maps & Special Iotas

Sets, dictionaries, maps, properties and other addon data types.

58 patterns. Click the tag `#topic/data-structures` to search them, or use the Pattern Browser.

## Spells

**[[Ephemera|Ephemera]]**

- [[Cleanse Status]] `entity, status → —`

**[[Hexcellular|Hexcellular]]**

- [[Schrödinger's Reflection]] `— → property`

## Patterns

**[[Ephemera|Ephemera]]**

- [[Ablation Purification]] `entity → num`
- [[Apothecary's Purification]] `entity → [status]`
- [[Apothecary's Purification II]] `status → num`
- [[Entity Distillation Status]] `status, vector → entity | NULL`
- [[Hashing Purification]] `any → hash`
- [[Pharmacist's Distillation]] `entity, status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Zone Exaltation Status]] `status, vec, num → [entity]`
- [[undocumented hash-to-numbers pattern]] `— → —`

**[[HexStruction|HexStruction]]**

- [[Bounding Purification]] `structure → [number]`
- [[Mirror Purification X]] `structure → structure`
- [[Mirror Purification Y]] `structure → structure`
- [[Mirror Purification Z]] `structure → structure`
- [[Rotation Purification Clockwise X]] `structure → structure`
- [[Rotation Purification Clockwise Y]] `structure → structure`
- [[Rotation Purification Clockwise Z]] `structure → structure`
- [[Rotation Purification Counter-Clockwise X]] `structure → structure`
- [[Rotation Purification Counter-Clockwise Y]] `structure → structure`
- [[Rotation Purification Counter-Clockwise Z]] `structure → structure`
- [[Structure Transformation Purification]] `structure → [number]`

**[[HexThings|HexThings]]**

- [[Accessed's Purification]] `dict → list`
- [[Accessor's Purification]] `dict → list`
- [[Entry Purification]] `dict → list`
- [[Impasse Reflection]] `— → dict`

**[[Hexcellular|Hexcellular]]**

- [[Observation Purification]] `property → any`
- [[Schrödinger's Gambit]] `property, any → —`
- [[Schrödinger's Purification]] `property → property`

**[[Hexpose|Hexpose]]**

- [[Detective's Purification]] `identifiable → identifier`
- [[Modicum Purification]] `any → identifier`

**[[Hextra Patterns|Hextra Patterns]]**

- [[Bucketing Gambit]] `many, num → set`
- [[Disordering Purification]] `[num]|[vec]|[entity] → set`
- [[Insertion Gambit]] `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] `set → list`
- [[Removal Gambit]] `set, (num|vec|entity) → set, bool`
- [[Spilling Disintegration]] `set → many`
- [[Vacant Reflection Entity Set]] `— → entity set`
- [[Vacant Reflection Number Set]] `— → num set`
- [[Vacant Reflection Vector Set]] `— → vec set`

**[[Iotaworks|Iotaworks]]**

- [[Patchwork Exaltation]] `iota, iota, vec, property → patchwork`

**[[MoreIotas|MoreIotas]]**

- [[Additive Distillation II]] `— → —`
- [[Classifier's Purification]] `any → iotatype`
- [[Collector's Distillation]] `vector, vector → list`
- [[Duelist's Purification]] `entity → item stack`
- [[Entity Distillation Type]] `entitytype, pos → entity | null`
- [[Hoarder's Distillation]] `vector, vector → list`
- [[Physician's Purification]] `entity → entitytype`
- [[Shieldbearer's Purification]] `entity → item stack`
- [[Sorter's Purification]] `itemtypable → itemtype`
- [[Sorter's Reflection]] `— → itemtype`
- [[Treasurer's Distillation]] `itemtype|item stack, int → item stack`
- [[Zone Exaltation Not-Type]] `entitytype, pos, num → [entity]`
- [[Zone Exaltation Type]] `entitytype, pos, num → [entity]`

**[[Overevaluate|Overevaluate]]**

- [[Jumbling Decomposition]] `jumble → number, list`
- [[Jumbling Gambit]] `number, list → jumble`

**[[PhLib|PhLib]]**

- [[Uncharted Reflection]] `— → map<⊥, ⊥>`

## Book pages on this topic

[[Dictionary Manipulation]] · [[Hash Set Manipulation]] · [[Identifiers]] · [[Iota Hashing]] · [[Iota Patching]] · [[Items]] · [[Jumble Iota]] · [[Maps]] · [[Property Iota]] · [[Status Iotas]] · [[Structure Iota]] · [[Types (MoreIotas)]] · [[Undocumented Ephemera Patterns]] · [[Undocumented MoreIotas Patterns]]
