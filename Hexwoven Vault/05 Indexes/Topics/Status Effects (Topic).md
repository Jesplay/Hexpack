---
tags: [moc, topic/effects]
---

# Status Effects

Potion-style effects: helpful (Zeniths, Aegis, Invisibility) and harmful (Nadirs), plus reading and removing effects.

39 patterns. Click the tag `#topic/effects` to search them, or use the Pattern Browser.

## Great Spells

**[[Hex Casting|Hex Casting]]**

- [[Black Sun's Zenith]] `entity, number, number → —`
- [[Blue Sun's Zenith]] `entity, number → —`
- [[Green Sun's Zenith]] `entity, number, number → —`
- [[Red Sun's Zenith]] `entity, number, number → —`
- [[White Sun's Zenith]] `entity, number, number → —`

**[[Ephemera|Ephemera]]**

- [[Hidden Sun's Zenith]] `entity, num → —`
- [[Instant Aegis]] `player, num, num, status, num → —`

**[[Oneironaut|Oneironaut]]**

- [[Stealth Shroud]] `entity, number → —`
- [[Uplifting Resonance]] `entity → —`

## Spells

**[[Hex Casting|Hex Casting]]**

- [[Alter Scale]] `entity, num → —`
- [[Black Sun's Nadir]] `entity, number, number → —`
- [[Blue Sun's Nadir]] `entity, number → —`
- [[Green Sun's Nadir]] `entity, number, number → —`
- [[Red Sun's Nadir]] `entity, number, number → —`
- [[White Sun's Nadir]] `entity, number, number → —`

**[[Ephemera|Ephemera]]**

- [[Cleanse Status]] `entity, status → —`

**[[Hexic|Hexic]]**

- [[Hidden Sun's Nadir]] `— → —`

**[[Lapisworks|Lapisworks]]**

- [[Enchant with FireyFists]] `entity → —`
- [[Enchant with Lightningbending]] `entity → —`
- [[Envelop Feet in Amel]] `entity → —`
- [[Grant Amel Air Sacs]] `entity → —`
- [[Make Fire-born]] `entity → —`
- [[Strip Enchantment]] `entity, num → num`

## Patterns

**[[Hex Casting|Hex Casting]]**

- [[Gulliver's Purification]] `entity → num`

**[[Ephemera|Ephemera]]**

- [[Ablation Purification]] `entity → num`
- [[Apothecary's Purification]] `entity → [status]`
- [[Apothecary's Purification II]] `status → num`
- [[Entity Distillation Status]] `status, vector → entity | NULL`
- [[Pharmacist's Distillation]] `entity, status → num`
- [[Pharmacist's Dstln. II]] `entity, status → num`
- [[Zone Exaltation Not-Status]] `status, vec, num → [entity]`
- [[Zone Exaltation Status]] `status, vec, num → [entity]`

**[[Hexpose|Hexpose]]**

- [[Clearance Distillation]] `entity, identifier → number`
- [[Concentration Distillation]] `entity, identifier → number`
- [[Condition Purification]] `identifier → number`
- [[Diagnosis Purification]] `entity → list of identifiers`
- [[Prescription Purification]] `item → list of identifiers`

**[[Lapisworks|Lapisworks]]**

- [[Enchantment Purification]] `entity, num → num`

**[[Oneironaut|Oneironaut]]**

- [[Discern Shroud]] `vector, number → [vector]`

## Book pages on this topic

[[Enchantments]] · [[Instant Aegis (Book Entry)]] · [[Invisibility]] · [[Nadirs]] · [[Nondetection]] · [[Pehkui]] · [[Status Effects]] · [[Status Iotas]] · [[Undocumented Hexic Patterns]] · [[Uplifting Resonance (Book Entry)]] · [[Zeniths]]
