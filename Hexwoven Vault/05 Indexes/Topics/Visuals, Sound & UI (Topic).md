---
tags: [moc, topic/visuals]
---

# Visuals, Sound & UI

Particles, sounds, shaders, specks, displays, menus, pigments and dyes: things you see and hear.

92 patterns. Click the tag `#topic/visuals` to search them, or use the Pattern Browser.

## Great Spells

**[[Complex Hex|Complex Hex]]**

- [[Summon Block Display]] `vector, string → —`
- [[Summon Item Display]] `vector, string → —`
- [[Summon Text Display]] `vector, string → —`

**[[Hexical|Hexical]]**

- [[Conjure Mesh]] `vector → entity`

## Spells

**[[Hex Casting|Hex Casting]]**

- [[Internalize Pigment]] `— → —`
- [[Make Note]] `vector, number, number → —`

**[[Complex Hex|Complex Hex]]**

- [[Kill BIT]] `BIT → —`
- [[Rotate BIT]] `BIT, quat → —`
- [[Scale BIT]] `BIT, vec → —`
- [[Set BIT 4x4]] `BIT, matrix → —`
- [[Translate BIT]] `BIT, vec → —`

**[[Ephemera|Ephemera]]**

- [[Externalize Pigment]] `Vector, itemtype → —`

**[[Hexal|Hexal]]**

- [[Particles]] `vec | [vec] → —`

**[[Hexic|Hexic]]**

- [[Apply Pigment]] `— → —`

**[[Hexical|Hexical]]**

- [[Age Scroll]] `entity → —`
- [[Confetti]] `vector, number/vector → —`
- [[Conjure Firework]] `vec, vec, num, num, list of vec, list of vec, bool, bool → —`
- [[Conjure Speck]] `any, vector, vector → entity`
- [[Dye]] `vector/entity, dye → —`
- [[Dye Ink]] `entity, vector → —`
- [[Gasp]] `entity → —`
- [[Illuminate]] `vector, number → —`
- [[Illuminate Ink]] `entity → —`
- [[Restore Scroll]] `entity → —`
- [[Simulate Firework]] `vector, vector, number → —`
- [[Sparkle]] `vector, vector, number → —`
- [[Squawk]] `vector, identifier → —`
- [[Vanish Scroll]] `entity → —`
- [[Vibrate]] `vector, entity/vector, number → —`

**[[Special Efhexs|Special Efhexs]]**

- [[Conjure Debris]] `..., identifier → —`
- [[Conjure Dust]] `..., vector, number → —`
- [[Conjure Fragments]] `..., item → —`
- [[Conjure Media]] `..., vector → —`
- [[Conjure Particle]] `identifier, vector, vector → —`
- [[Conjure Sediment]] `..., identifier → —`
- [[Conjure Sparkly Dust]] `..., vector, vector, number → —`
- [[Haunt]] `null | list of entities → —`
- [[Play Sound]] `id, vec, num, num → —`

## Patterns

**[[Complex Hex|Complex Hex]]**

- [[Entity Purification BIT]] `vector → entity`
- [[Get BIT 4x4]] `BIT → matrix`
- [[Zone Distillation BIT]] `vector, number → [entity]`
- [[Zone Distillation Not BIT]] `vector, number → [entity]`

**[[Ephemera|Ephemera]]**

- [[Particle Burst]] `vec, vec, num, num → —`

**[[HexIntent|HexIntent]]**

- [[Equation Hex Cloud]] `vec|entity, equation, num → none`
- [[Hex Trail]] `vec, vec, vec, num, num, num? → none`
- [[Intent Button]] `list, text → intent`
- [[Intent Checkbox]] `bool, text → intent`
- [[Intent Dropdown]] `list, text → intent`
- [[Intent Input]] `text → intent`
- [[Intent Numeric Input]] `num, text → intent`
- [[Intent Section]] `text → intent`
- [[Intent Select List]] `list, num, text, bool? → intent`
- [[Intent Slider]] `num, num, num, text → intent`
- [[Manifest Grid Menu]] `list, text, num → —`
- [[Manifest List Menu]] `list, text → —`
- [[Manifest Radial Menu]] `list, text → —`
- [[Manifest Spell Circle]] `[pattern], vec, vec, num, num?, vec? → none`

**[[Hexical|Hexical]]**

- [[Alter Speck]] `speck entity, any → —`
- [[Broadcast Vision]] `— → —`
- [[Chromatic Purification]] `id/vector/entity → dye/null`
- [[Clear Vision]] `— → —`
- [[Dismiss Speck]] `speck entity → —`
- [[Hallucinate Click]] `— → —`
- [[Hallucinate Pling]] `— → —`
- [[Identify Importance]] `— → —`
- [[Internalize Pigment II]] `pigment → —`
- [[Move Speck]] `speck entity, vector → —`
- [[Paint Speck]] `speck entity, pigment → —`
- [[Pierce Darkness]] `— → —`
- [[Pigment Exaltation]] `pigment, vec, num → vec`
- [[Pigment Purification]] `dye/entity → pigment`
- [[Resize Speck]] `speck entity, number → —`
- [[Roll Speck]] `speck entity, number → —`
- [[Rotate Speck]] `speck entity, vector → —`
- [[Split Vision]] `— → —`
- [[Tangle Purification]] `entity → list of vectors`
- [[Thicken Speck]] `speck entity, number → —`
- [[Time Speck]] `speck entity, number → —`
- [[Vision Purification]] `dye → vector`
- [[Visualize Forms]] `— → —`
- [[Weave Mesh]] `entity, list of vectors → —`
- [[Zone Distillation Speck]] `vector, number → list of entities`

**[[Scryglass|Scryglass]]**

- [[Clear Icon]] `index → —`
- [[Draw Line]] `index, vector, vector, vector → —`
- [[Draw Rectangle]] `index, vector, vector, vector → —`
- [[Draw Text]] `index, vector, number, any → —`
- [[Icons Reflection]] `— → list of indices`
- [[Periphery Reflection]] `— → vector`
- [[Rotate Icon]] `index, number → —`
- [[Scale Icon]] `index, number → —`

**[[Special Efhexs|Special Efhexs]]**

- [[Audio Reflection]] `— → list of identifiers`
- [[Particulate Reflection]] `— → list of identifiers`

## Book pages on this topic

[[Animated Scrolls]] · [[BIT Manipulation]] · [[Basic Spells]] · [[Dyes]] · [[Externalize Pigment (Book Entry)]] · [[Haunting]] · [[Hexal Spells]] · [[Hexical Spells]] · [[Internalize Pigment (Book Entry)]] · [[Menu Manifestation]] · [[Meshes]] · [[Particle Burst (Book Entry)]] · [[Particles (Book Entry)]] · [[Pigment Manipulation]] · [[Pyrotechnics]] · [[Shaders]] · [[Sounds]] · [[Specks]] · [[Telepathy]] · [[Undocumented Hexic Patterns]] · [[Viewforming]] · [[Visual Manifestation]]
