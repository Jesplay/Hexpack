---
tags: [moc]
---

# All Patterns

Every pattern documented in the guidebook (1240 total), grouped by mod. Non-spell patterns manipulate the [[The Stack|stack]] and [[Iotas|iotas]] without touching the world.

## [[Hex Casting|Hex Casting]]

- [[Locate Sentinel]] — `— → vector` · Costs a negligible amount of media.
- [[Wayfind Sentinel]] — `vector → vector` · Costs a negligible amount of media.
- [[Aviator's Purification]] — `entity → boolean`
- [[Additive Distillation]] — `num|vec, num|vec → num|vec`
- [[Subtractive Distillation]] — `num|vec, num|vec → num|vec`
- [[Multiplicative Distillation]] — `num|vec, num|vec → num|vec`
- [[Division Distillation]] — `num|vec, num|vec → num|vec`
- [[Length Purification]] — `num|vec → number`
- [[Power Distillation]] — `num|vec, num|vec → num|vec`
- [[Conjunction Distillation]] — `bool, bool → bool`
- [[Disjunction Distillation]] — `bool, bool → bool`
- [[Negation Purification]] — `bool → bool`
- [[Exclusion Distillation]] — `bool, bool → bool`
- [[An Example]] — `— → —`
- [[Gulliver's Purification]] — `entity → num`
- [[Surgeon's Exaltation]] — `list, num, any → list`
- [[Excisor's Distillation]] — `list, num → list`
- [[Selection Distillation]] — `list, number → any`
- [[Flock's Disintegration]] — `list → many`
- [[Derivation Decomposition]] — `list → list, any`
- [[Speaker's Decomposition]] — `list → list, any`
- [[Mind's Reflection]] — `— → entity | null`
- [[Compass' Purification]] — `entity → vector`
- [[Compass' Purification II]] — `entity → vector`
- [[Alidade's Purification]] — `entity → vector`
- [[Archer's Distillation]] — `vector, vector → vector | null` · Costs a negligible amount of media.
- [[Architect's Distillation]] — `vector, vector → vector | null` · Costs a negligible amount of media.
- [[Scout's Distillation]] — `vector, vector → entity | null` · Costs a negligible amount of media.
- [[Reveal]] — `any → any`
- [[Stadiometer's Purification]] — `entity → num`
- [[Pace Purification]] — `entity → vector`
- [[Media Reflection (Hex Casting)]] — `— → num`
- [[Locator's Distillation]] — `list, any → num`
- [[Chronicler's Purification]] — `entity → any`
- [[Auditor's Purification]] — `entity → bool`
- [[Assessor's Purification]] — `entity → bool`
- [[Numerical Reflection]] — `— → number`
- [[Floor Purification]] — `num|vec → num|vec`
- [[Ceiling Purification]] — `num|vec → num|vec`
- [[Vector Exaltation]] — `num, num, num → vector`
- [[Vector Disintegration]] — `vector → num, num, num`
- [[Modulus Distillation]] — `num|vec, num|vec → num|vec`
- [[Axial Purification]] — `vec|num → vec|num`
- [[Entropy Reflection]] — `— → num`
- [[True Reflection]] — `— → bool`
- [[False Reflection]] — `— → bool`
- [[Nullary Reflection]] — `— → null`
- [[Vector Reflection Zero]] — `— → vector`
- [[Vector Rfln. +X-X]] — `— → vector`
- [[Vector Rfln. +Y-Y]] — `— → vector`
- [[Vector Rfln. +Z-Z]] — `— → vector`
- [[Circle's Reflection]] — `— → num`
- [[Arc's Reflection]] — `— → num`
- [[Euler's Reflection]] — `— → num`
- [[Harmony Reflection]] — `— → num`
- [[Selection Exaltation]] — `list, num, num → list`
- [[Retrograde Purification]] — `list → list`
- [[Uniqueness Purification]] — `list → list`
- [[Novice's Gambit]] — `any → —`
- [[Jester's Gambit]] — `any, any → any, any`
- [[Rotation Gambit]] — `any, any, any → any, any, any`
- [[Rotation Gambit II]] — `any, any, any → any, any, any`
- [[Gemini Decomposition]] — `any → any, any`
- [[Prospector's Gambit]] — `any, any → any, any, any`
- [[Undertaker's Gambit]] — `any, any → any, any, any`
- [[Gemini Gambit]] — `any, number → many`
- [[Dioscuri Gambit]] — `any, any → any, any, any, any`
- [[Flock's Reflection]] — `— → number`
- [[Fisherman's Gambit]] — `number → any`
- [[Fisherman's Gambit II]] — `number → any`
- [[Bookkeeper's Gambit]] — `many → many`
- [[Swindler's Gambit]] — `many, number → many`
- [[Maximus Distillation]] — `number, number → bool`
- [[Minimus Distillation]] — `number, number → bool`
- [[Maximus Distillation II]] — `number, number → bool`
- [[Minimus Distillation II]] — `number, number → bool`
- [[Augur's Purification]] — `any → bool`
- [[Augur's Exaltation]] — `bool, any, any → any`
- [[Equality Distillation]] — `any, any → bool`
- [[Inequality Distillation]] — `any, any → bool`
- [[Similarity Distillation (Hex Casting)]] — `any, any → bool`
- [[Dissimilarity Distillation]] — `any, any → bool`
- [[Sorter's Distillation]] — `entity, entity → bool`
- [[Stacking Distillation]] — `entity, entity → bool`
- [[Taxonomer's Distillation]] — `entity, entity → bool`
- [[Surveyor's Distillation]] — `vec, vec → bool`
- [[Inspector's Distillation]] — `vec, vec → bool`
- [[Entity Purification]] — `vector → entity or null`
- [[Entity Purification Animal]] — `vector → entity or null`
- [[Entity Purification Monster]] — `vector → entity or null`
- [[Entity Purification Item]] — `vector → entity or null`
- [[Entity Purification Player]] — `vector → entity or null`
- [[Entity Purification Living]] — `vector → entity or null`
- [[Zone Distillation Animal]] — `vector, number → list`
- [[Zone Distillation Non-Animal]] — `vector, number → list`
- [[Zone Distillation Monster]] — `vector, number → list`
- [[Zone Distillation Non-Monster]] — `vector, number → list`
- [[Zone Distillation Item]] — `vector, number → list`
- [[Zone Distillation Non-Item]] — `vector, number → list`
- [[Zone Distillation Player]] — `vector, number → list`
- [[Zone Distillation Non-Player]] — `vector, number → list`
- [[Zone Distillation Living]] — `vector, number → list`
- [[Zone Distillation Non-Living]] — `vector, number → list`
- [[Zone Distillation Any]] — `vector, number → list`
- [[Integration Distillation]] — `list, any → list`
- [[Vacant Reflection]] — `— → list`
- [[Single's Purification]] — `any → list`
- [[Flock's Gambit]] — `many, num → list`
- [[Speaker's Distillation]] — `list, any → list`
- [[Contemplation]] — `— → —`
- [[Introspection]] — `— → —`
- [[Retrospection]] — `— → —`
- [[Evanition]] — `— → —`
- [[Consideration]] — `— → —`
- [[Interjection]] — `— → —`
- [[Meditation]] — `num → —`
- [[Recollection]] — `— → num, list`
- [[Scribe's Reflection]] — `— → any`
- [[Auditor's Reflection]] — `— → bool`
- [[Assessor's Reflection]] — `— → bool`
- [[Huginn's Gambit]] — `any → —`
- [[Muninn's Reflection]] — `— → any`
- [[Sine Purification]] — `num → num`
- [[Cosine Purification]] — `num → num`
- [[Tangent Purification]] — `num → num`
- [[Inverse Sine Purification]] — `num → num`
- [[Inverse Cosine Purification]] — `num → num`
- [[Inverse Tangent Purification]] — `num → num`
- [[Inverse Tangent Distillation]] — `num, num → num`
- [[Logarithmic Distillation]] — `num, num → num`
- [[Hermes' Gambit]] — `evaluatable → many`
- [[Iris' Gambit]] — `evaluatable → many`
- [[Thoth's Gambit]] — `evaluatable, list → list`
- [[Charon's Gambit]] — `— → —`
- [[Thanatos' Reflection]] — `— → number`
- [[Waystone Reflection]] — `— → vector`
- [[Lodestone Reflection]] — `— → vector`
- [[Lesser Fold Reflection]] — `— → vector`
- [[Greater Fold Reflection]] — `— → vector`
- [[Akasha's Distillation]] — `vector, pattern → any` · Costs about one Amethyst Dust.
- [[Vector Reflection -X]] — `— → —`
- [[Vector Reflection -Y]] — `— → —`
- [[Vector Reflection -Z]] — `— → —`
- [[Vector Reflection +X]] — `— → —`
- [[Vector Reflection +Y]] — `— → —`
- [[Vector Reflection +Z]] — `— → —`

## [[Caduceus|Caduceus]]

- [[Thetis' Gambit]] — `[pattern] | pattern → many`
- [[Thetis' Gambit II]] — `[pattern] | pattern, any → many`
- [[Arke's Gambit]] — `[pattern] | pattern → many`
- [[Arke's Gambit II]] — `[pattern] | pattern, any → many`
- [[Elbrus' Gambit]] — `[pattern] → many`
- [[Prometheus' Gambit]] — `number → many`
- [[Ewer's Reflection]] — `— → any`
- [[Ewer's Purification]] — `jump → any`
- [[Ewer's Gambit]] — `any → —`

## [[Complex Hex|Complex Hex]]

- [[Symbolic Purification]] — `str → expression`
- [[Substitution Exaltation]] — `expr, expr, expr | num → expr`
- [[Equality Purification]] — `expr, expr | num → expr`
- [[Piecewise Exaltation]] — `expr, expr|num, expr|num → expr`
- [[Derivation Purification]] — `expr, expr → expr`
- [[Neo's Exaltation]] — `num, num, expr → matrix`
- [[Parametric Line]] — `vec, expr → entity`
- [[Parametric Surface]] — `vec, expr → entity`
- [[Constant Imagination]] — `— → complex`
- [[Constant Realisation]] — `— → complex`
- [[Realising Complexities]] — `complex → num`
- [[Imagining Complexities]] — `complex → num`
- [[Conjugation Prfn]] — `complex → complex`
- [[Long Reflection]] — `— → long`
- [[Example 43L]] — `— → —`
- [[Long Purification]] — `num/long → long/num`
- [[Left Shift Distillation]] — `long, num → long`
- [[Logical Right Shift Dstl]] — `long, num → long`
- [[Arithmetic R. Shift Dstl]] — `long, num → long`
- [[Quaternionic Exal]] — `num, vec → quaternion`
- [[Quaternionic Dntg]] — `quaternion → num, vec`
- [[Quaternionic Rfln. 1]] — `— → quaternion`
- [[Quaternionic Rfln. i]] — `— → quaternion`
- [[Quaternionic Rfln j]] — `— → quaternion`
- [[Quaternionic Rfln k]] — `— → quaternion`
- [[Quaterionic Prfn. 1]] — `quaternion → num`
- [[Quaterionic Prfn. i]] — `quaternion → num`
- [[Quaterionic Prfn. j]] — `quaternion → num`
- [[Quaterionic Prfn k]] — `quaternion → num`
- [[Quaternionic Inverse]] — `quaternion → quat`
- [[Matrixification]] — `quaternion → matrix`
- [[Quaterniation]] — `matrix → quaternion`
- [[Axis Angle Distillation]] — `vec, num → quaternion`
- [[Hyp. Sine Prfn]] — `num → num`
- [[Hyp. Cosine Prfn]] — `num → num`
- [[Hyp. Tangent Prfn]] — `num → num`
- [[Inv. Hyp. Sine Prfn]] — `num → num`
- [[Inv. Hyp. Cosine Prfn]] — `num → num`
- [[Inv. Hyp. Tangent Prfn]] — `num → num`
- [[Entity Purification BIT]] — `vector → entity`
- [[Zone Distillation BIT]] — `vector, number → [entity]`
- [[Zone Distillation Not BIT]] — `vector, number → [entity]`
- [[Get BIT 4x4]] — `BIT → matrix`
- [[UTF Purification]] — `num/string → string/num`
- [[Bubbles' Purification]] — `any → {any}`

## [[Ephemera|Ephemera]]

- [[Particle Burst]] — `vec, vec, num, num → —` · Costs a negligible amount of media.
- [[Postage Purification]] — `Player → num` · cost to send them a message via Postman's Gambit, between 1/10 of a dust and 10 dust.
- [[Postman's Gambit II]] — `— → —` · cost of sending me a message via Postman's Gambit back to 1/10 of a dust.
- [[Terminate Flight]] — `— → —` · cost of one tenth of an Amethyst Dust.
- [[Aviator's Purification II]] — `player → bool | null`
- [[Aviator's Purification III]] — `player → num | null`
- [[Network Distillation]] — `linkable, num → [linkable]` · Costs a negligible amount of media.
- [[Routing Distillation]] — `linkable, iota → vec` · Costs half an amethyst dust.
- [[Menderbug's Reflection]] — `— → num`
- [[Menderbug's Reflection II]] — `— → num`
- [[Hashing Purification]] — `any → hash`
- [[Decorator's Purification]] — `item frame → number`
- [[Decorator's Gambit]] — `item frame, number → —` · Costs a negligible amount of media.
- [[Scout's Exaltation]] — `[type], vec, vec → entity | NULL` · Costs an amount of media slightly less negligible than that of the aforementioned pattern.
- [[Entropy Reflection II]] — `— → num`
- [[Clearance Purification]] — `vec → bool`
- [[Flock's Reflection II]] — `— → num`
- [[Flock's Purification]] — `— → num`
- [[State Purification]] — `vec → [string]`
- [[State Distillation]] — `vec, string → bool|num|vec|string`
- [[Ride Wisp]] — `wisp → —` · Costs one amethyst shard.
- [[Dismount]] — `— → —` · Costs 1/100 of a dust.
- [[Equestrian Purification]] — `entity → [entity]`
- [[Equestrian Purification II]] — `entity → entity`
- [[Apothecary's Purification]] — `entity → [status]`
- [[Apothecary's Purification II]] — `status → num`
- [[Pharmacist's Distillation]] — `entity, status → num`
- [[Pharmacist's Dstln. II]] — `entity, status → num`
- [[Entity Distillation Status]] — `status, vector → entity | NULL`
- [[Zone Exaltation Status]] — `status, vec, num → [entity]`
- [[Zone Exaltation Not-Status]] — `status, vec, num → [entity]`
- [[Ablation Purification]] — `entity → num`
- [[Seshat's Gambit]] — `— → any | null`
- [[Seshat's Reflection]] — `— → num`
- [[undocumented hash-to-numbers pattern]] — `— → —`
- [[No]] — `— → —`

## [[Hex Machina|Hex Machina]]

- [[Deploy]] — `vec → —` · costs 1 Amethyst Shard.
- [[Ketchum's Reflection]] — `— → bool`
- [[Gunslinger's Reflection]] — `— → num`
- [[Gunslinger's Prog]] — `— → —`
- [[Gunslinger's Regr]] — `— → —`
- [[Gunslinger's Roulette]] — `num → —`
- [[Colt's Reflection]] — `— → entity`

## [[HexDebug|HexDebug]]

- [[Debugger's Reflection]] — `— → bool`
- [[Cognitohazard Reflection]] — `— → cognitohazard`
- [[Set Breakpoint Before]] — `— → —`
- [[Set Breakpoint After]] — `— → —`
- [[Mindsplice Purification]] — `vec → list | null`
- [[Sprocket's Purification]] — `vec → num`
- [[Splicer's Decomposition]] — `vec → num | null, num | null`
- [[Compositor's Purification]] — `vec → any`
- [[Producer's Purification]] — `vec → bool`
- [[Director's Purification]] — `vec → bool`
- [[Projectionist's Purification]] — `vec → num`
- [[Shutter's Purification]] — `vec → bool`
- [[Projectionist's Purification II]] — `vec → num`
- [[Shutter's Purification II]] — `vec → bool`

## [[HexIntent|HexIntent]]

- [[Manifest List Menu]] — `list, text → —`
- [[Manifest Grid Menu]] — `list, text, num → —`
- [[Manifest Radial Menu]] — `list, text → —`
- [[Intent Button]] — `list, text → intent`
- [[Intent Input]] — `text → intent`
- [[Intent Numeric Input]] — `num, text → intent`
- [[Intent Slider]] — `num, num, num, text → intent`
- [[Intent Checkbox]] — `bool, text → intent`
- [[Intent Select List]] — `list, num, text, bool? → intent`
- [[Intent Section]] — `text → intent`
- [[Intent Dropdown]] — `list, text → intent`
- [[Manifest Threshold]] — `vec, presence, num, text?, pigment? → none` · Cost: 2 dust per second.
- [[Presence Intent]] — `vec, vec → presenceIntent`
- [[Raze Manifestation]] — `vec, num → none`
- [[Manifest Relay]] — `vec, vec, vec, num? → none`
- [[Equation Hex Cloud]] — `vec|entity, equation, num → none`
- [[Hex Trail]] — `vec, vec, vec, num, num, num? → none`
- [[Manifest Spell Circle]] — `[pattern], vec, vec, num, num?, vec? → none` · Cost: 1/500 dust.
- [[Manifest Splinter]] — `vec, num, list → none` · Cost: 5 dust x (active splinters + 1).
- [[Renew Splinter]] — `vec, num → none` · Cost: replacing at the same position is free but moving to a new position costs 0.5 dust.
- [[Saturn's Gambit]] — `pattern → none`
- [[Get Splinter Location]] — `— → vec`
- [[Destroy Current Splinter]] — `— → none`
- [[Destroy Splinters]] — `— → none`
- [[Set Charm Cast Sound]] — `string | text | null, item entity? → none` · Cost: 10 dust.
- [[Exit If Interacting]] — `none → none`
- [[Open Casting Screen]] — `none → none`
- [[Clear Stack]] — `none → none`
- [[Mind Vault Counts]] — `vec → [num, num]`
- [[Resonance Reflection]] — `none → memory`
- [[Resonance Purification]] — `memory → [pattern] | null`
- [[Resonance Gambit]] — `memory, any → none`

## [[HexStruction|HexStruction]]

- [[Mirror Purification Z]] — `structure → structure`
- [[Mirror Purification X]] — `structure → structure`
- [[Mirror Purification Y]] — `structure → structure`
- [[Rotation Purification Clockwise Y]] — `structure → structure`
- [[Rotation Purification Counter-Clockwise Y]] — `structure → structure`
- [[Rotation Purification Clockwise X]] — `structure → structure`
- [[Rotation Purification Counter-Clockwise X]] — `structure → structure`
- [[Rotation Purification Clockwise Z]] — `structure → structure`
- [[Rotation Purification Counter-Clockwise Z]] — `structure → structure`
- [[Bounding Purification]] — `structure → [number]`
- [[Structure Transformation Purification]] — `structure → [number]`

## [[HexThings|HexThings]]

- [[Impasse Reflection]] — `— → dict`
- [[Accessor's Purification]] — `dict → list`
- [[Accessed's Purification]] — `dict → list`
- [[Entry Purification]] — `dict → list`
- [[Patience]] — `— → —`
- [[Patience Patterns]] — `— → —`
- [[Shard Reflection]] — `— → bool | null`
- [[Introjection]] — `— → —`
- [[Retrojection]] — `— → —`
- [[Retention Distillation (HexThings)]] — `list, (num | [num]) → list`
- [[Flattening Exaltation]] — `list, num, list → list`
- [[Ranger's Exaltation]] — `num, num, (num | null) → list`
- [[Rejection Distillation (HexThings)]] — `list, (num | [num]) → list`
- [[Ferris Distillation (HexThings)]] — `list, num → list`
- [[Deduction Purification]] — `list → list`

## [[Hexal|Hexal]]

- [[Identity Reflection]] — `— → entity`
- [[Reservoir Reflection]] — `— → num`
- [[Manager's Purification]] — `entity → [pattern]`
- [[Allegiance Distillation]] — `entity, entity → bool`
- [[Pathfinder's Gambit]] — `vec → —`
- [[Pathfinder's Reflection]] — `— → vec`
- [[Speedometer's Reflection]] — `— → num`
- [[Entity Purification Wisp]] — `vec → entity | null`
- [[Zone Distillation Wisp]] — `vec, num → [entity]`
- [[Zone Distillation Non-Wisp]] — `vec, num → [entity]`
- [[Delay Wisp]] — `number → —`
- [[Listen]] — `— → —`
- [[Wander]] — `— → —`
- [[Phonebook Purification]] — `num → linkable`
- [[Recognition Purification]] — `linkable → num`
- [[Popularity Reflection]] — `— → num`
- [[Recitation Reflection]] — `— → any`
- [[Postmaster's Reflection]] — `— → num`
- [[Unclogging Gambit]] — `— → —`
- [[Open Transmit]] — `num → —`
- [[Close Transmit]] — `— → —`
- [[Stocktake Reflection]] — `— → [itemtype]`
- [[Stocktake Purification]] — `mote | itemtype → [mote]`
- [[Capacity Reflection]] — `— → int`
- [[Containment Distillation]] — `vec, (itemtype | mote) → bool`
- [[Mediafy Item]] — `item | item, mote → mote` · costs a tenth of an Amethyst Dust.
- [[Return Item]] — `mote, vec | mote, vec, num → —` · Costs a tenth of an Amethyst Dust.
- [[Stacking Distillation II]] — `mote, (mote | item) → bool`
- [[Splitting Gambit]] — `mote, int → mote, mote`
- [[Depot Purification]] — `mote → vec`
- [[Depot Gambit]] — `mote, vec → mote`
- [[Craft]] — `mote | [mote] | [[mote]] → [mote]` · Costs a tenth of an Amethyst Dust.
- [[Preview Craft]] — `item | [item] | [[item]] → [item]`
- [[Seniority Purification]] — `villager → int`
- [[Offering Purification]] — `villager → [complicated!]`
- [[Trade]] — `villager, [mote], int? → item` · Costs a tenth of an Amethyst Dust.
- [[Use Item On]] — `mote, (entity | vec, vec) → —`
- [[Familiar's Reflection]] — `— → entity | null`
- [[Marked Distillation]] — `gate, entity → bool`
- [[Gate's Closing]] — `gate | gate, vec → —` · Costs half an Amethyst Shard, plus an Amethyst Dust per metre transported per entity, if the gate was drifting.
- [[Timekeeper's Reflection]] — `— → number`
- [[Diver's Purification]] — `entity → num`
- [[Nurse's Purification]] — `entity → num`
- [[Squire's Purification]] — `entity → num`
- [[Boxer's Purification]] — `entity → num`
- [[Lamplighter's Purification]] — `vec → num`
- [[Factorial Purification]] — `num → num`
- [[Running Sum Purification]] — `[num] → [num]`
- [[Running Product Purification]] — `[num] → [num]`
- [[Elysium's Gambit]] — `vector, pattern → —`
- [[Lethe's Gambit]] — `pattern → —`
- [[Brigh's Gambit]] — `pattern → —`

## [[Hexcassettes|Hexcassettes]]

- [[Enqueue]] — `list, number → text`
- [[Threading Reflection]] — `— → number`
- [[Operating Reflection]] — `— → list of text iotas`
- [[Program Distillation]] — `player, text → boolean`
- [[Oracle Purification]] — `text → null/number`
- [[State Reflection]] — `— → null/text`

## [[Hexcellular|Hexcellular]]

- [[Observation Purification]] — `property → any`
- [[Schrödinger's Gambit]] — `property, any → —` · Costs about a tenth of an Amethyst Dust.
- [[Schrödinger's Purification]] — `property → property`

## [[Hexic|Hexic]]

- [[Gearbox Reflection]] — `— → vector`
- [[Engineer's Distraction]] — `number → —`
- [[Engineer's Departure]] — `— → —`
- [[Engineer's Resignation]] — `— → ...`
- [[Greater Reveal]] — `[iota] | iota → —`
- [[Ferris Distillation]] — `[a], int → [a]`
- [[Retention Distillation]] — `[a], int → [a]`
- [[Rejection Distillation]] — `[a], int → [a]`
- [[Moses's Gambit]] — `[a], (a, a → bool) → [[a]]`
- [[Deductive Purification]] — `[bool] → [int]`
- [[Excisor's Gambit]] — `[a], int → [a], [a], a`
- [[Refinement Distillation]] — `[a], [a → bool] → [a]`
- [[Lani's Gambit]] — `[… → …] → —`
- [[Vulpine Gambit]] — `— → —`
- [[Thoth's Pseudogambit]] — `— → —`
- [[Cessation]] — `— → —`
- [[Resumption]] — `— → —`
- [[Vulpine Expulsion]] — `— → —`

## [[Hexical|Hexical]]

- [[Authenticator's Distillation]] — `item stack, entity → boolean`
- [[Chromatic Purification]] — `id/vector/entity → dye/null`
- [[Vision Purification]] — `dye → vector`
- [[Pigment Purification]] — `dye/entity → pigment`
- [[Pigment Exaltation]] — `pigment, vec, num → vec`
- [[Internalize Pigment II]] — `pigment → —`
- [[Wristpocket Reflection]] — `— → item`
- [[Move Speck]] — `speck entity, vector → —`
- [[Rotate Speck]] — `speck entity, vector → —`
- [[Roll Speck]] — `speck entity, number → —`
- [[Alter Speck]] — `speck entity, any → —`
- [[Time Speck]] — `speck entity, number → —`
- [[Resize Speck]] — `speck entity, number → —`
- [[Thicken Speck]] — `speck entity, number → —`
- [[Paint Speck]] — `speck entity, pigment → —`
- [[Dismiss Speck]] — `speck entity → —`
- [[Zone Distillation Speck]] — `vector, number → list of entities`
- [[Evocation Reflection]] — `— → list of patterns`
- [[Evoker Reflection]] — `— → number`
- [[Hopper]] — `source, num?, dest., num? → —` · Cost scales based on the amount of mass transferred - a stack of 64 items will cost about three Amethyst Dust.
- [[Hopper Purification]] — `source → list`
- [[Handy Reflection]] — `— → number`
- [[Switch]] — `number → —`
- [[Infiltration Reflection]] — `— → list of vectors`
- [[Clear Vision]] — `— → —`
- [[Pierce Darkness]] — `— → —`
- [[Visualize Forms]] — `— → —`
- [[Broadcast Vision]] — `— → —`
- [[Identify Importance]] — `— → —`
- [[Split Vision]] — `— → —`
- [[Charmed Gambit]] — `any → —`
- [[Charmed Reflection]] — `— → any`
- [[Charmed Reflection II]] — `— → any`
- [[Weave Mesh]] — `entity, list of vectors → —`
- [[Tangle Purification]] — `entity → list of vectors`
- [[Genie Reflection Spatial]] — `— → vector`
- [[Genie Reflection Rotational]] — `— → vector`
- [[Genie Reflection Kinetic]] — `— → vector`
- [[Genie Reflection Temporal]] — `— → number`
- [[Genie Reflection Media]] — `— → number`
- [[Genie Reflection Memory]] — `— → any`
- [[Finale Reflection]] — `— → boolean`
- [[Archgenie Purification]] — `entity → boolean`
- [[Archgenie Reflection Spatial]] — `— → vector`
- [[Archgenie Reflection Rotational]] — `— → vector`
- [[Archgenie Reflection Kinetic]] — `— → vector`
- [[Archgenie Reflection Temporal]] — `— → number`
- [[Archgenie Reflection Media]] — `— → number`
- [[Archgenie Gambit]] — `any → —`
- [[Archgenie Reflection Memory]] — `— → any`
- [[Archivist Reflection]] — `— → list of patterns`
- [[Glyphmaker's Distillation]] — `pattern, number → pattern`
- [[Congruence Distillation]] — `pattern, pattern → boolean`
- [[Similarity Distillation]] — `pattern, pattern → boolean`
- [[Chirographer's Purification]] — `pattern → list of numbers`
- [[Calligrapher's Purification]] — `list of numbers → pattern`
- [[Handwriting Distillation]] — `pattern → list of vectors`
- [[Grok Reflection]] — `— → list, list`
- [[Telepathic Reflection]] — `— → number`
- [[Send Thought]] — `any → —`
- [[Shout Thought]] — `any → —`
- [[Hallucinate Pling]] — `— → —`
- [[Hallucinate Click]] — `— → —`
- [[Offensive Reflection]] — `— → number`
- [[Manipulative Reflection]] — `— → number`
- [[Charge Reflection]] — `— → number`
- [[Retreat Reflection]] — `— → number`
- [[Dodge Reflection]] — `— → number`
- [[Evade Reflection]] — `— → number`
- [[Leaping Reflection]] — `— → number`
- [[Stealthy Reflection]] — `— → number`
- [[Rolling Reflection]] — `— → number`
- [[Librarian's Purification]] — `vector → pattern/null`
- [[Librarian's Purification II]] — `vector → any`

## [[Hexodus|Hexodus]]

- [[Grasp Purification]] — `entity → number`
- [[Falling Purification]] — `entity → vector`
- [[Impact Purification]] — `entity → number`
- [[Falling Purification II]] — `entity → vector`
- [[Impact Purification II]] — `entity → number`

## [[Hexpats|Hexpats]]

- [[Patter's Purification]] — `entity → bool`

## [[Hexportation|Hexportation]]

- [[Source Data]] — `conduit → [view, ...]`
- [[Sink Data]] — `conduit → [view, ...]`
- [[View Prfn. Amount]] — `view → num`
- [[View Prfn. Capacity]] — `view → num`
- [[View Prfn. Name]] — `view → string`

## [[Hexpose|Hexpose]]

- [[Miner's Purification]] — `identifier → number`
- [[Demoman's Purification]] — `identifier → number`
- [[Orientation Purification]] — `vector → vector/null`
- [[Farmer's Purification]] — `vector → number/null`
- [[Facet Purification]] — `vector → list of identifiers`
- [[Facet Distillation]] — `vector, identifier → any`
- [[Cartographer's Purification]] — `identifier → vector`
- [[News Reflection]] — `— → text, text, number`
- [[Newspaper Reflection]] — `— → list of lists`
- [[Caliper's Purification]] — `entity → number`
- [[Theodolite's Purification]] — `entity → vector`
- [[Vitality Purification]] — `entity → number`
- [[Fitness Purification]] — `entity → number`
- [[Suffocation Purification]] — `entity → number`
- [[Lung Purification]] — `entity → number`
- [[Inferno Purification]] — `entity → number`
- [[Enderman's Purification]] — `entity → boolean`
- [[Youth Purification]] — `entity → boolean`
- [[Reproduction Purification]] — `entity → boolean/null`
- [[Sloth's Purification]] — `entity → boolean`
- [[Racer's Purification]] — `entity → boolean`
- [[Vehicle Purification]] — `entity → entity/null`
- [[Jockey Purification]] — `entity → list of entities`
- [[Shooter Purification]] — `entity → entity/null`
- [[Adoration Purification]] — `entity → entity/null`
- [[Absorption Purification]] — `entity → number`
- [[Ambit Purification]] — `entity/vector → boolean`
- [[Staff Reflection]] — `— → boolean`
- [[Dexterity Reflection]] — `— → boolean`
- [[Device Reflection]] — `— → boolean`
- [[Constructed Reflection]] — `— → boolean`
- [[Hunger Purification]] — `player entity → number`
- [[Stamina Purification]] — `entity entity → number`
- [[Edibility Purification]] — `identifier → boolean`
- [[Calorie Purification]] — `identifier → number`
- [[Satiation Purification]] — `identifier → number`
- [[Flesh Purification]] — `identifier → boolean`
- [[Dessert Purification]] — `identifier → boolean`
- [[Detective's Purification]] — `identifiable → identifier`
- [[Modicum Purification]] — `any → identifier`
- [[Item Purification]] — `entity → item`
- [[Offer Purification]] — `identifier, number → item`
- [[Tool Purification]] — `entity → item`
- [[Accessory Purification]] — `entity → item`
- [[Cart Purification]] — `entity → list of items | null`
- [[Chest Purification]] — `vector → list of items | null`
- [[Aegis Purification]] — `entity → list of items`
- [[Pocket Reflection]] — `— → list of items`
- [[Storage Purification]] — `item → number`
- [[Warehouse Purification]] — `identifier → number`
- [[Deterioration Purification]] — `item → number`
- [[Fragility Purification]] — `identifier → number`
- [[Thaumaturgist's Purification]] — `item → list of identifiers`
- [[Charm Distillation]] — `item, identifier → number`
- [[Appellation Purification]] — `item → text`
- [[Legacy Purification]] — `item → list of text`
- [[Glamour Purification]] — `item → number`
- [[Glamour Purification II]] — `item → number`
- [[Reading Purification]] — `item → list of text`
- [[Bibliography Purification]] — `item → text, number`
- [[Collector Purification]] — `item → number`
- [[Media Reflection]] — `— → number`
- [[Media Purification]] — `entity | item | vector → number`
- [[Potential Purification]] — `item → number/null`
- [[Feline Purification]] — `entity → identifier`
- [[Anger Purification]] — `entity → number`
- [[Showcase Purification]] — `entity → number`
- [[Artistic Purification]] — `entity → identifier`
- [[Diagnosis Purification]] — `entity → list of identifiers`
- [[Prescription Purification]] — `item → list of identifiers`
- [[Condition Purification]] — `identifier → number`
- [[Concentration Distillation]] — `entity, identifier → number`
- [[Clearance Distillation]] — `entity, identifier → number`
- [[Tier Purification]] — `entity → number`
- [[Professional Purification]] — `entity → identifier`
- [[Culture Purification]] — `entity → identifier`
- [[Nurture Purification]] — `identifier → identifier/null`
- [[Luminance Purification]] — `vector → number`
- [[Meterologist's Reflection]] — `— → number`
- [[Battery Purification]] — `vector → number`
- [[Peripheral Purification]] — `vector → number/null`
- [[Circadian Reflection]] — `— → number`
- [[Temporal Reflection]] — `— → number`
- [[Geographical Purification]] — `— → identifier`
- [[Plane Reflection]] — `— → identifier`
- [[Lunar Reflection]] — `— → number`
- [[Distortion Reflection]] — `— → boolean`
- [[Epiphany Purification]] — `player entity → boolean`
- [[Sentience Purification]] — `entity → boolean`
- [[Reading Purification (Hexpose)]] — `any → text`
- [[Glyph Purification]] — `text → list`
- [[Collage Purification]] — `list → text`
- [[Lumière Gambit]] — `text, vector → text`
- [[Gothic Gambit]] — `text, boolean → text`
- [[Manutius' Gambit]] — `text, boolean → text`
- [[Notetaker's Gambit]] — `text, boolean → text`
- [[Editor's Gambit]] — `text, boolean → text`
- [[Censor's Gambit]] — `text, boolean → text`
- [[Dousing Distillation]] — `vector, vector → vector/null`
- [[Lilypad Distillation]] — `vector, vector → vector/null`
- [[Railgun Exaltation]] — `vec, vec, id → vec/null`
- [[Laser Exaltation]] — `vec, vec, id → vec/null`
- [[Name Purification]] — `entity → display`

## [[Hextra Patterns|Hextra Patterns]]

- [[Archer's Distillation II]] — `vec, vec → vec | null` · Costs a negligible amount of media.
- [[Architect's Distillation II]] — `vec, vec → vec | null` · Costs a negligible amount of media.
- [[Sextant Decomposition]] — `entity → number, number`
- [[Sextant Purification]] — `entity → number`
- [[Vector Reflection X II]] — `— → vector`
- [[Vector Reflection Y II]] — `— → vector`
- [[Vector Reflection Z II]] — `— → vector`
- [[Vector Reflection 1 II]] — `— → vector`
- [[Example 1]] — `— → —`
- [[Example 2]] — `— → —`
- [[Example 3]] — `— → —`
- [[Example 4]] — `— → —`
- [[Radian Purification]] — `num → num`
- [[Degree Purification]] — `num → num`
- [[Reflection Purification]] — `num/vec → num/vec`
- [[Incrementation Purification]] — `num → num`
- [[Decrementation Purification]] — `num → num`
- [[Rounding Purification]] — `num/vec → num/vec`
- [[Rounding Distillation]] — `num/vec, num → num/vec`
- [[Entropy Purification]] — `num → num`
- [[Entropy Distillation]] — `num, num → num`
- [[Scientific Purification]] — `num/vec → num/vec`
- [[Approach Exaltation]] — `num, num, num → num`
- [[Theta Distillation]] — `num, num → num`
- [[Turning Exaltation]] — `num, num, num → num`
- [[Norm Purification]] — `vec → vec`
- [[Length Distillation]] — `vec, num → vec`
- [[Chaos Reflection]] — `— → vec`
- [[Distance Distillation]] — `vec, vec → num`
- [[Polar Distillation]] — `num, num → vec`
- [[Polar Decomposition]] — `vec → num, num`
- [[Pitch Distillation]] — `vec, num → vec`
- [[Yaw Distillation]] — `vec, num → vec`
- [[Roll Distillation]] — `vec, num → vec`
- [[Pitch Purification]] — `num → vec`
- [[Yaw Purification]] — `num → vec`
- [[Roll Purification]] — `num → vec`
- [[Coordinate Purification X]] — `vec → num`
- [[Coordinate Distillation X]] — `vec, num → vec`
- [[Swizzling Purification]] — `vec → vec`
- [[Vector Purification]] — `[num] | vec → vec | [num]`
- [[Vector Reflection +1]] — `— → vector`
- [[Capsizing Gambit]] — `any → any`
- [[Capsizing Gambit II]] — `any → any`
- [[Dredging Gambit]] — `any → any`
- [[Dredging Gambit II]] — `any → any`
- [[Angler's Gambit]] — `many → many`
- [[Length Equality Distillation]] — `vec, vec → bool`
- [[Length Inequality Distillation]] — `vec, vec → bool`
- [[Range Exaltation]] — `num/vec, num, num, num → bool`
- [[Range Exaltation II]] — `num/vec, num, num, num → bool`
- [[Coalescing Distillation]] — `any, any → any`
- [[Coalescing Gambit]] — `any, [pattern] | pattern → any`
- [[Retention Gambit Augur's]] — `A → A, bool`
- [[Retention Gambit Equality]] — `A, B → A, B, bool`
- [[Retention Gambit Inequality]] — `A, B → A, B, bool`
- [[Retention Gambit Maximus]] — `A, B → A, B, bool`
- [[Retention Gambit Minimus]] — `A, B → A, B, bool`
- [[Retention Gambit Maximus II]] — `A, B → A, B, bool`
- [[Retention Gambit Minimus II]] — `A, B → A, B, bool`
- [[Retention Gambit Length Equality]] — `A, B → A, B, bool`
- [[Retention Gambit Length Inequality]] — `A, B → A, B, bool`
- [[Speaker's Decomposition II]] — `list → list, any`
- [[Derivation Decomposition II]] — `list → list, any`
- [[Viewer's Gambit]] — `list, num → list, any`
- [[Separation Gambit]] — `list, num → list, list`
- [[Elimination Gambit]] — `list, any → list, bool`
- [[Elimination Gambit II]] — `list, any → list, num`
- [[Swindler's Distillation]] — `list, num → list`
- [[Shuffling Purification]] — `list → list`
- [[Birdkeeper's Gambit]] — `many → many, list`
- [[Vacant Reflection Number Set]] — `— → num set`
- [[Vacant Reflection Vector Set]] — `— → vec set`
- [[Vacant Reflection Entity Set]] — `— → entity set`
- [[Insertion Gambit]] — `set, (num|vec|entity) → set, bool`
- [[Removal Gambit]] — `set, (num|vec|entity) → set, bool`
- [[Ordering Purification]] — `set → list`
- [[Disordering Purification]] — `[num]|[vec]|[entity] → set`
- [[Bucketing Gambit]] — `many, num → set`
- [[Spilling Disintegration]] — `set → many`
- [[Helios' Gambit]] — `any, ([pattern] | pattern) → many`
- [[Selene's Gambit]] — `any, ([pattern] | pattern) → many`
- [[Acheron's Gambit]] — `any → —`
- [[Styx's Gambit]] — `any → —`
- [[Djehuty's Gambit]] — `[pattern], list → list`
- [[Anubis' Gambit]] — `many, [pattern], list → many`
- [[Ra's Gambit]] — `many, [pattern], list → many`
- [[Vector Reflection -1]] — `— → —`
- [[Coordinate Purification Y]] — `— → —`
- [[Coordinate Purification Z]] — `— → —`
- [[Coordinate Distillation Y]] — `— → —`
- [[Coordinate Distillation Z]] — `— → —`

## [[Hexwright|Hexwright]]

- [[Essence Purification]] — `vec → [string | null]`
- [[Aspect Distillation]] — `vec, num → num`
- [[Warden's Reflection]] — `vec → [vec,vec]`
- [[Anchors' Purification]] — `num or str → list`
- [[Resonance Reflection (Hexwright)]] — `— → resonance or null`
- [[Emitter's Reflection]] — `— → vector`
- [[Cantor's Gambit]] — `slot num, any → —`
- [[Cantor's Reflection]] — `slot num → any`
- [[Await Dust]] — `[pattern] | pattern → —`
- [[Conveyance's Reflection]] — `— → [state]`
- [[Locomotion's Reflection]] — `— → num, num`
- [[Speedy's Reflection]] — `— → [num]`
- [[Vessel's Dismissal]] — `bool → —`
- [[Vessel's Reflection]] — `— → entity`
- [[Manifold's Reflection]] — `— → [itemstack]`
- [[Manifold's Distillation]] — `stack, num → item` · Costs 1 Dust When this pattern is used in a Sealed Satchel or Hexbound Chest withdraw hex, the item is placed at your feet and then under your cursor in one tick, fast enough that the item never appears on the floor for someone else to pick up.
- [[Sphere Region]] — `vector, number → region`
- [[Box Region]] — `vector, vector → region`
- [[Cylinder Region]] — `vector, vector, number → region`
- [[Region Union]] — `region, region → region`
- [[Region Intersection]] — `region, region → region`
- [[Region Difference]] — `region, region → region`
- [[Region Cut]] — `region, vector, vector → region, region`
- [[Region Query]] — `region, vector → boolean`
- [[Reach Purification]] — `region → boolean`
- [[Entities Within]] — `region → [entities]`
- [[Region Thoth]] — `region, [pattern] → list`
- [[Region Fill]] — `region → —`
- [[Region Break]] — `region → —`
- [[Reaper's Purification]] — `entity → remnants` · Costs 1 Dust.
- [[Remnant's Reflection]] — `remnants → list`
- [[Remnant Distillation]] — `remnants, num | str → remnant` · Costs 1/4 Dust.
- [[Vessel's Reflection (Hexwright)]] — `item → str, num`
- [[Remnant Decanting]] — `num → remnant`
- [[Construct's Reach]] — `— → —`

## [[Hierophantics|Hierophantics]]

- [[Hierophant's Reflection]] — `— → [mind]`
- [[Payload Purification]] — `mind → [pattern] | null`
- [[Trigger Purification]] — `mind → trigger | null`
- [[Trigger Reflection Damage]] — `— → trigger`
- [[Trigger Reflection Damage Type]] — `— → trigger`
- [[Trigger Purification Health]] — `num → trigger`
- [[Trigger Purification Breath]] — `num → trigger`
- [[Trigger Purification Hunger]] — `num → trigger`
- [[Trigger Purification Velocity]] — `num → trigger`
- [[Trigger Purification Fall]] — `num → trigger`
- [[Trigger Reflection Drop]] — `— → trigger`
- [[Trigger Reflection Attack]] — `— → trigger`
- [[Trigger Reflection Break]] — `— → trigger`
- [[Trigger Reflection Jump]] — `— → trigger`
- [[Trigger Reflection Teleport]] — `— → trigger`
- [[Inversion Purification]] — `trigger → trigger`

## [[Iotaworks|Iotaworks]]

- [[Patchwork Exaltation]] — `iota, iota, vec, property → patchwork`

## [[IoticBlocks|IoticBlocks]]

- [[Tachygrapher's Distillation]] — `entity | vec, num → any`

## [[Lapisworks|Lapisworks]]

- [[Lapidarist's Purification]] — `vec → any`
- [[Dowser's Reflection]] — `— → any`
- [[Cognition Purification]] — `— → vec`
- [[Jeweller's Reflection]] — `— → any`
- [[Jeweller's Gambit]] — `any → —`
- [[Gemologist's Reflection]] — `— → bool`
- [[Epigrapher's Reflection]] — `— → bool`
- [[Phianglement Purification]] — `vec → [vec]`
- [[Enhancement Purification]] — `entity, num → —`
- [[Enchantment Purification]] — `entity, num → num`
- [[Inner Media Purification]] — `M.T.I → num`
- [[Cubic Exaltation]] — `[pattern], vec, vec, bool → [any]`
- [[Spherical Exaltation]] — `[pattern], vec, num → [any]`
- [[Spherical Exaltation II]] — `[pattern], vec, num → [any]`
- [[Mintiest Gambit]] — `num, num, [pattern] → [any]`
- [[Kitkat's Gambit]] — `num, [pattern] → —`
- [[Erebus' Gambit]] — `positive num → —`
- [[Queue Reflection]] — `— → [any]`
- [[Queue Gambit]] — `[any] → —`
- [[Executioner's Reflection]] — `— → [pattern]`
- [[Judge's Gambit]] — `[pattern] → —`
- [[Empty Purification]] — `vec → bool`
- [[Empty Distillation]] — `vec, vec → bool`
- [[Visible Distillation]] — `entity, vec → bool`
- [[Block Equivalency Distillation]] — `vec, vec → bool`
- [[Block Equality Distillation]] — `vec, vec → bool`
- [[Cleaner's Purification]] — `vec, vec → bool`
- [[Secretary's Purification]] — `num → any`
- [[Reviewer's Purification]] — `num → bool`
- [[Sanctioner's Purification]] — `num → bool`
- [[Hadamard Distillation]] — `vec, vec → vec`
- [[Robbie's Exaltation]] — `[any], int, any → [any]`
- [[Dealer's Purification]] — `[any] → any`
- [[Minimum Distillation]] — `num, num → num`
- [[Maximum Distillation]] — `num, num → num`
- [[Locator's Reflection]] — `— → vec|null`
- [[Singer's Purification]] — `vec → ([0-24], 0-20)`
- [[Interviewer's Prfn]] — `vec → pattern|null`
- [[Amalgamation Purification]] — `vec → amalgam`
- [[The Mind's Decomp]] — `amalgam → amalgam, bool`
- [[Loudener's Decomposition]] — `amalgam → amalgam, int`
- [[Loudener's Distillation]] — `amalgam, num → amalgam`
- [[Fisherman's Decomposition]] — `amalgam → amalgam, num`
- [[Fisherman's Distillation]] — `amalgam, num → amalgam`
- [[My Mind's Decomposition]] — `— → num`
- [[Amalgamation Gambit]] — `num → —`
- [[Mistake Decomposition]] — `amalgam → amalgam, num`
- [[Etched Decomposition]] — `amalgam → amalgam, [pattern]`
- [[Etched Distillation]] — `amalgam, [pattern] → amalgam`

## [[MoreIotas|MoreIotas]]

- [[Sorter's Purification]] — `itemtypable → itemtype`
- [[Classifier's Purification]] — `any → iotatype`
- [[Physician's Purification]] — `entity → entitytype`
- [[Sorter's Reflection]] — `— → itemtype`
- [[Entity Distillation Type]] — `entitytype, pos → entity | null`
- [[Zone Exaltation Type]] — `entitytype, pos, num → [entity]`
- [[Zone Exaltation Not-Type]] — `entitytype, pos, num → [entity]`
- [[Duelist's Purification]] — `entity → item stack`
- [[Shieldbearer's Purification]] — `entity → item stack`
- [[Hoarder's Distillation]] — `vector, vector → list`
- [[Collector's Distillation]] — `vector, vector → list`
- [[Treasurer's Distillation]] — `itemtype|item stack, int → item stack`
- [[Transformation Purification]] — `num | vec | list → mat`
- [[Restoration Purification]] — `mat → num | vec | list`
- [[Restoration Purification II]] — `mat → [[num]]`
- [[Identity Purification]] — `int ≥ 0 → mat`
- [[Zero Distillation]] — `int ≥ 0, int ≥ 0 → mat`
- [[Rotation Distillation]] — `vec, num → mat`
- [[Multiplicative Distillation II]] — `mat, mat → mat`
- [[Division Distillation II]] — `mat, mat → mat`
- [[Power Distillation II]] — `mat, mat → mat`
- [[Inverse Purification]] — `mat → mat`
- [[Determinant Purification]] — `mat → num`
- [[Tower Distillation]] — `mat, mat → mat`
- [[Sprawling Distillation]] — `mat, mat → mat`
- [[Toppling Gambit]] — `mat, num → mat, mat`
- [[Mitosis Gambit]] — `mat, num → mat, mat`
- [[Separation Distillation]] — `str, str → [str]`
- [[Input Purification]] — `str → num | null`
- [[Blank Reflection]] — `— → str`
- [[Spacing Reflection]] — `— → str`
- [[Comma Reflection]] — `— → str`
- [[Breaking Reflection]] — `— → str`
- [[Whisper Reflection]] — `— → str`
- [[Listener's Reflection]] — `— → str`
- [[Sifter's Gambit]] — `str | null → —`
- [[Sifter's Reflection]] — `— → str | null`
- [[Reader's Purification]] — `vec → str | [str]`
- [[Scrivener's Purification]] — `any → str`
- [[Patternmaster's Purification]] — `pattern → str | null`
- [[Moniker Purification]] — `entity → str`
- [[Case Distillation]] — `str, bool | null → str`
- [[Additive Distillation II]] — `— → —`

## [[Oneironaut|Oneironaut]]

- [[Discern Shroud]] — `vector, number → [vector]`
- [[Baton Reflection]] — `— → vector`
- [[Baton Reflection II]] — `— → vector`
- [[Metronome Reflection]] — `— → number`
- [[Metronome Gambit]] — `number → —`
- [[Finale]] — `— → —`
- [[Encore]] — `number → —`
- [[Williams' Gambit]] — `non-list → —`
- [[Williams' Reflection]] — `— → non-list`
- [[Conductor's Reflection]] — `— → bool`
- [[Williams' Gambit II]] — `non-list → —`
- [[Williams' Reflection II]] — `— → non-list`
- [[Conductor's Reflection II]] — `— → bool`
- [[Scrutinize Sentinel]] — `vec, player → num | NULL` · Costs a negligible amount of media.
- [[Spatial Reflection]] — `— → Imprint` · Costs a negligible amount of media.
- [[Spatial Reflection II]] — `— → Imprint | NULL` · Costs one-tenth of an amethyst dust.
- [[Homeland Reflection]] — `— → Imprint` · Costs one-tenth of an amethyst dust.
- [[Inferno Reflection]] — `— → Imprint` · Costs one-tenth of an amethyst dust.
- [[Atlas' Decomposition]] — `Imprint → num, num`
- [[Ratio Purification]] — `Imprint → num`
- [[Shift Sentinel]] — `Imprint → —` · Costs one amethyst shard.
- [[Inscribe Idea]] — `key, any → —` · Costs a quarter of an amethyst dust.
- [[Retrieve Idea]] — `key → any` · Costs one-eigth of an amethyst dust.
- [[Metadata Purification]] — `key → number` · Costs no media.
- [[Metadata Distillation]] — `key, player → bool` · Costs no media.
- [[Signature Purification]] — `soulprint → bool` · Costs a negligible amount of media.
- [[Soulprint Reflection]] — `— → soulprint` · Costs a negligible amount of media.
- [[Sign Item]] — `— → —` · Costs 1 amethyst dust.

## [[Overevaluate|Overevaluate]]

- [[Sekhmet's Gambit]] — `— → varies`
- [[Geb's Gambit]] — `many → many`
- [[Nut's Gambit]] — `many → many`
- [[Reflecting Gambit]] — `any, any, any → any, any, any`
- [[Bubbling Gambit]] — `any, any, any → any, any, any`
- [[Dioscuri Gambit II]] — `many, number → many`
- [[Jumbling Gambit]] — `number, list → jumble`
- [[Jumbling Decomposition]] — `jumble → number, list`
- [[Soroban Reflection]] — `— → number`
- [[Soroban Gambit]] — `— → —`
- [[Soroban Gambit II]] — `number → —`
- [[Athena's Gambit]] — `list of patterns → boolean`
- [[Athena's Revelation]] — `— → —`
- [[Athena's Reflection]] — `— → garbage/text`
- [[Apep's Gambit]] — `pattern/list, list → any`
- [[Tutu's Gambit]] — `— → —`
- [[Janus' Gambit]] — `— → —`
- [[Ma'at's Gambit]] — `boolean, any → —`
- [[Nephthys' Gambit]] — `pattern/list → varies`
- [[Atalanta's Gambit]] — `— → —`
- [[Castor's Gambit]] — `any → —`
- [[Pollux's Gambit]] — `any → —`
- [[Sisyphus' Gambit]] — `list → —`
- [[Themis' Gambit]] — `pattern/list, list → list`

## [[PhLib|PhLib]]

- [[Uncharted Reflection]] — `— → map<⊥, ⊥>`

## [[Phlexiful|Phlexiful]]

- [[Read Catcher Threshold]] — `— → num`
- [[Write Catcher Threshold]] — `num → —`
- [[Read Catcher Media]] — `— → num`

## [[Scryglass|Scryglass]]

- [[Periphery Reflection]] — `— → vector`
- [[Icons Reflection]] — `— → list of indices`
- [[Clear Icon]] — `index → —`
- [[Draw Text]] — `index, vector, number, any → —`
- [[Rotate Icon]] — `index, number → —`
- [[Scale Icon]] — `index, number → —`
- [[Draw Rectangle]] — `index, vector, vector, vector → —`
- [[Draw Line]] — `index, vector, vector, vector → —`

## [[Slate Works|Slate Works]]

- [[Set Recipe]] — `vec, [item stack|item type|null] → —` · Costs a negligible amount of media.
- [[Set Craft Count]] — `vec, number → —`
- [[Get Craft Count]] — `vec → number`
- [[Apply Pseudosentinels]] — `[vectors] → —` · Costs 1 dust to command the Pseudosentinel, and due to fatigue, an extra 1/8th of a dust (stacking) for each Pseudosentinel that was already moved in the same instance.
- [[Locate Pseudosentinels]] — `— → [vectors]`
- [[Get Vessels]] — `— → [vecs]`
- [[Reawaken Item]] — `[patterns] → —` · Costs a 1/4th of a dust per activated Vessel.
- [[Check Item]] — `[patterns] → bool`
- [[Reorient Items]] — `— → —`
- [[Bind Macro]] — `vec, any iota, pattern → —` · Costs a negligible amount of media.
- [[Review Broadcast]] — `vec → any`
- [[Merchant's Purification]] — `vec → complicated`
- [[Set Slot]] — `int → —`
- [[List Items]] — `— → [item stack]`
- [[Wave Location Rfln]] — `— → vector`
- [[Wave Facing Rfln]] — `— → vector`
- [[Wave Speed Rfln]] — `— → number`
- [[Circle Media Rfln]] — `— → number`

## [[Special Efhexs|Special Efhexs]]

- [[Particulate Reflection]] — `— → list of identifiers`
- [[Audio Reflection]] — `— → list of identifiers`

## [[YAHA|YAHA]]

- [[Aima Purification]] — `entity → entity | null`
- [[Kavruka Reflection]] — `— → vector`
