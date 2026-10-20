---
tags: [moc]
---

# All Spells

Spells are patterns that **change the world** (and cost [[Media]]). 283 spells across all installed addons. See also [[All Great Spells]].

## [[Hex Casting|Hex Casting]]

- [[Explosion]] — `vector, number → —` · Costs a negligible amount at power 0, plus 3 extra Amethyst Dust per point of explosion power.
- [[Fireball]] — `vector, number → —` · Costs one Amethyst Dust, plus about 3 extra Amethyst Dusts per point of explosion power.
- [[Impulse]] — `entity, vector → —` · Costs units of Amethyst Dust equal to the square of the length of the vector, plus one for every Impulse except the first targeting an entity.
- [[Blink]] — `entity, number → —` · Costs about one Amethyst Shard per two blocks travelled.
- [[Make Note]] — `vector, number, number → —` · Costs a negligible amount of media.
- [[Place Block]] — `vector → —` · Costs about an eighth of one Amethyst Dust.
- [[Break Block]] — `vector → —` · Costs about an eighth of one Amethyst Dust, or a negligible amount if breaking a Conjured Block or Conjured Light.
- [[Create Water]] — `vector → —` · Costs about one Amethyst Dust.
- [[Destroy Liquid]] — `vector → —` · Costs about two Charged Amethyst.
- [[Conjure Block]] — `vector → —` · Costs about one Amethyst Dust.
- [[Conjure Light]] — `vector → —` · Costs about one Amethyst Dust.
- [[Overgrow]] — `vector → —` · Costs a bit more than one Amethyst Dust.
- [[Edify Sapling]] — `vector → —` · Costs about one Charged Amethyst.
- [[Ignite]] — `entity | vector → —` · Costs about one Amethyst Dust.
- [[Extinguish Area]] — `vector → —` · Costs about six Amethyst Dust.
- [[White Sun's Nadir]] — `entity, number, number → —` · cost is one Amethyst Dust per 10 seconds.
- [[Blue Sun's Nadir]] — `entity, number → —` · cost is one Amethyst Dust per 5 seconds.
- [[Black Sun's Nadir]] — `entity, number, number → —` · cost is one Amethyst Dust per second.
- [[Red Sun's Nadir]] — `entity, number, number → —` · cost is one Amethyst Dust per 3 seconds.
- [[Green Sun's Nadir]] — `entity, number, number → —` · cost is one Amethyst Dust per 5 seconds.
- [[Craft Cypher]] — `entity, list → —` · Costs about one Charged Amethyst.
- [[Craft Trinket]] — `entity, list → —` · Costs about five Charged Amethysts.
- [[Craft Artifact]] — `entity, list → —` · Costs about ten Charged Amethysts.
- [[Recharge Item]] — `entity → —` · Costs about one Amethyst Shard per item.
- [[Erase Item]] — `— → —` · Costs about one Amethyst Dust per item.
- [[Summon Sentinel]] — `vector → —` · Costs about one Amethyst Dust.
- [[Banish Sentinel]] — `— → —` · Costs a negligible amount of media.
- [[Internalize Pigment]] — `— → —` · Costs about one Amethyst Dust.
- [[Caster's Glamour]] — `— → —` · Costs a negligible amount of media.
- [[Anchorite's Flight]] — `entity, number → —` · Costs about 2 Amethyst Dust per meter of safety, though any radius less than 1 meter will still cost 2 Amethyst Dust.
- [[Wayfarer's Flight]] — `entity, number → —` · Costs about 2 Amethyst Dust per second of flight; I believe it is best suited for travel.
- [[Alter Scale]] — `entity, num → —` · Costs about 1 Amethyst Shard.
- [[Chronicler's Gambit]] — `entity, any → —`
- [[Scribe's Gambit]] — `any → —`
- [[Akasha's Gambit]] — `vector, pattern, any → —` · Costs about one Amethyst Dust.

## [[Complex Hex|Complex Hex]]

- [[Translate BIT]] — `BIT, vec → —`
- [[Rotate BIT]] — `BIT, quat → —`
- [[Scale BIT]] — `BIT, vec → —`
- [[Kill BIT]] — `BIT → —`
- [[Set BIT 4x4]] — `BIT, matrix → —`

## [[Ephemera|Ephemera]]

- [[Externalize Pigment]] — `Vector, itemtype → —` · Costs a negligible amount of media if targeting my own block, or 1 Amethyst Dust if not.
- [[Postman's Gambit]] — `Player, any → —`
- [[Network Gateway]] — `entity, linkable, vec → —`
- [[Cleanse Status]] — `entity, status → —`

## [[Hex Machina|Hex Machina]]

- [[Encapsulate]] — `entity → —` · costs 1 Amethyst Dust.

## [[HexDebug|HexDebug]]

- [[Craft Debugger]] — `entity, [pattern] → —` · Costs about ten Charged Amethyst.
- [[Craft Quenched Debugger]] — `entity, [pattern] → —` · Costs about ten Charged Amethyst.
- [[Imbue Mindsplice Table]] — `vec, list → —` · Costs about five Charged Amethyst.
- [[Sprocket's Gambit]] — `vec, num → —`
- [[Splicer's Gambit]] — `vec, num | null, num | null → —`
- [[Compositor's Gambit]] — `vec, any → —`
- [[Projectionist's Gambit]] — `vec, num → —`
- [[Projectionist's Gambit II]] — `vec, num → —`

## [[HexStruction|HexStruction]]

- [[Devour Structure]] — `vec, vec → structure` · Costs about an eighth of one Amethyst Dust for every block consumed.
- [[Actualize Structure]] — `vec, structure → —` · Costs about an eighth of one Amethyst Dust for every block placed.
- [[Visualize Structure]] — `vec, structure, num → —` · Costs about a hundredth of one Amethyst Dust for every block displayed.

## [[Hexal|Hexal]]

- [[Smelt]] — `vec | entity | mote → —` · Costs three quarters of an Amethyst Dust per item smelted (i.
- [[Freeze]] — `vec → —` · Costs one Amethyst Dust.
- [[Falling Block]] — `vec → —` · Costs one and a half Amethyst Dust.
- [[Place Block II]] — `(itemtype | mote), vec → —` · Costs an eighth of an Amethyst Dust.
- [[Particles]] — `vec | [vec] → —` · Costs two thousandths of an Amethyst Dust per location.
- [[Summon Projectile Wisp]] — `[pattern], vec, vec, number → —`
- [[Summon Cyclic Wisp]] — `[pattern], vec, number → —`
- [[Haste]] — `num → —`
- [[Allow Transfer]] — `num → —`
- [[Disallow Transfer]] — `num → —`
- [[Allow Transfer Others]] — `wisp, num → —`
- [[Disallow Transfer Others]] — `wisp, num → —`
- [[Link]] — `linkable → —` · costs one Amethyst Shard.
- [[Link Others]] — `linkable, linkable → —` · costs one Amethyst Shard.
- [[Unlink]] — `num → —` · costs two Amethyst Dust.
- [[Unlink Others]] — `linkable, linkable → —` · costs two Amethyst Dust.
- [[Send Iota]] — `num, any → —` · costs one hundredth of an Amethyst Dust.
- [[Bind Storage]] — `vec → —` · Costs 32 Amethyst Dust.
- [[Bind Storage - Temporary]] — `vec → —` · Costs one thousandth of an Amethyst Dust.
- [[Gate's Opening]] — `gate, entity → —` · Costs one twentieth of an Amethyst Dust.
- [[Gate's Dismissal]] — `gate, entity → —`
- [[Phase Block]] — `vec, num → —` · Costs one ten-thousandth of an Amethyst Dust times the square of the time to phase out for.
- [[Mnemosyne's Gambit]] — `vector, pattern → —`

## [[Hexcassettes|Hexcassettes]]

- [[Dequeue]] — `pattern → —`
- [[Disqueue]] — `— → —`

## [[Hexcellular|Hexcellular]]

- [[Schrödinger's Reflection]] — `— → property` · Costs about five Charged Amethyst to etch this law into the universe.

## [[Hexchanting|Hexchanting]]

- [[Imbue Equipment]] — `[pattern] → —` · Costs 9 Charged Amethyst except on arrows, which cost 5 Shards.

## [[Hexic|Hexic]]

- [[Etch Pattern]] — `pattern, str?, entity, iota → —` · cost 5 dust; replacing/removing macros is free.
- [[Tie Mediaweave]] — `— → —` · Costs a shard if it succeeds, or nothing if it didn't (I am either not wearing a mediaweave, or it is already tied).
- [[Untie Mediaweave]] — `— → —` · Costs a shard if it succeeds, or nothing if it didn't (I am either not wearing a mediaweave, or it is not tied).
- [[Hidden Sun's Nadir]] — `— → —`
- [[Apply Pigment]] — `— → —`
- [[Erase Block]] — `— → —`

## [[Hexical|Hexical]]

- [[Conjure Mage Block]] — `vector → —` · Costs about three Amethyst Dust.
- [[Reset Mage Block]] — `vector → —` · Costs about three Amethyst Dust.
- [[Modify Block Bouncy]] — `vector → —`
- [[Modify Block Energized]] — `vector, number → —`
- [[Modify Block Ephemeral]] — `vector, number → —`
- [[Modify Block Volatile]] — `vector → —`
- [[Rotate Block]] — `vector, vector → —` · Costs about an eighth of an Amethyst Dust.
- [[Rotate Entity]] — `entity, number, number → —`
- [[Autograph]] — `— → —`
- [[Unautograph]] — `— → —` · Costs about one Amethyst Dust similar to Erase Item.
- [[Dye]] — `vector/entity, dye → —` · Costs about an eighth of one Amethyst Dust.
- [[Magic Missile]] — `vector, vector → —` · Costs about one Amethyst Dust.
- [[Prestidigitation]] — `entity/vector → —` · Costs about a tenth of one Amethyst Dust.
- [[Wristpocket]] — `— → —` · Costs about a eighth of one Amethyst Dust.
- [[Sleight]] — `item entity/vector → —` · Costs about a fourth of one Amethyst Dust.
- [[Mage Hand]] — `entity/vector → —` · Costs about one Amethyst Dust.
- [[Mage Mouth]] — `— → —` · Costs about one Amethyst Dust and mishaps if the item is not edible.
- [[Conjure Speck]] — `any, vector, vector → entity` · Costs a negligible amount of media.
- [[Confetti]] — `vector, number/vector → —` · Costs about half of one Amethyst Dust.
- [[Vibrate]] — `vector, entity/vector, number → —` · Costs about a sixteenth of one Amethyst Dust.
- [[Sparkle]] — `vector, vector, number → —` · Costs a negligible amount of media.
- [[Crack Device]] — `— → —` · Costs about one Charged Amethyst.
- [[Conjure Flower]] — `vector, identifer → —` · Costs about a quarter of an Amethyst Dust.
- [[Illuminate]] — `vector, number → —` · Costs about a quarter of an Amethyst Dust.
- [[Gasp]] — `entity → —` · Costs about one Amethyst Dust.
- [[Squawk]] — `vector, identifier → —` · Costs about a half of an Amethyst Dust.
- [[Conjure Egg]] — `vector → entity` · Costs about two Amethyst Dust.
- [[Conjure Spit]] — `vector → entity` · Costs about a fourth of one Amethyst Dust.
- [[Conjure Snowball]] — `vector → entity` · Costs about half of an Amethyst Dust.
- [[Conjure Fireball]] — `vector → entity` · Costs about three Amethyst Dust.
- [[Extract Block]] — `vector, number → —` · Costs about one, three, and five Amethyst Dust when power input is 0, 1, and 2 respectively.
- [[Collect Block]] — `vector → —` · Costs about half of an Amethyst Dust.
- [[Hex Gummy]] — `vector → —` · Costs about one Amethyst Dust.
- [[Conjure Hexburst]] — `vector, any → —` · Costs about one Amethyst Dust and is subject to the Transgress Others mishap.
- [[Conjure Hextito]] — `vector, list of patterns → —` · Costs about two Amethyst Dust and is subject to the Transgress Others mishap.
- [[Conjure Spike]] — `vector, vector, number → —` · Costs about one Amethyst Dust.
- [[Push]] — `vector, vector → —` · Costs an eighth of an Amethyst Dust.
- [[Dispense]] — `item entity, vector, vector → —` · costs about half of one Amethyst Dust.
- [[Cook]] — `item entity → —` · Costs about one Amethyst Dust for every ten seconds that the Furnace would need to have been active.
- [[Roast]] — `item entity → —`
- [[Smoke]] — `item entity → —`
- [[Blast]] — `item entity → —`
- [[Cut Stone]] — `item entity, identifier → —` · Costs about an eighth of one Amethyst Dust.
- [[Displace]] — `entity, vector → —` · Costs about half of an Amethyst Dust.
- [[Appendage]] — `vector → —`
- [[Export Media]] — `vector, number → —`
- [[Inculcate]] — `list of patterns → —` · Costs about one Charged Amethyst.
- [[Deploy Sentinels]] — `list of vectors → —`
- [[Simulate Firework]] — `vector, vector, number → —` · Costs about one Amethyst Shard.
- [[Conjure Firework]] — `vec, vec, num, num, list of vec, list of vec, bool, bool → —`
- [[Name Item]] — `entity, text/null → —`
- [[Describe Item]] — `entity, list of text/null → —`
- [[Horrible]] — `vector → entity`
- [[Charmed Gambit II]] — `any → —`
- [[Discharm]] — `entity → —` · Costs about one Amethyst Dust.
- [[Wish]] — `list of patterns → —`
- [[Genie Gambit]] — `any → —`
- [[Refuel Lamp]] — `number → —` · Costs just the media I'm giving the Lamp.
- [[Write Grimoire]] — `pattern, list → —`
- [[Erase Grimoire]] — `pattern → —`
- [[Restore Scroll]] — `entity → —`
- [[Age Scroll]] — `entity → —`
- [[Vanish Scroll]] — `entity → —`
- [[Dye Ink]] — `entity, vector → —`
- [[Illuminate Ink]] — `entity → —`
- [[Induce Digging]] — `entity → —` · cost, around three Charged Amethyst.
- [[Grok Gambit]] — `list, list → —`
- [[Librarian's Gambit]] — `vector, pattern, any → —`
- [[Librarian's Gambit II]] — `vector → —`

## [[Hexodus|Hexodus]]

- [[Gravitate]] — `entity, vector, number → —` · costs about one-fourth of an Amethyst Dust.
- [[Alter Ballast]] — `vector, number → —`

## [[Hexpats|Hexpats]]

- [[Patter's Gambit]] — `entity → —` · Costs nothing if the target isn't on fire, otherwise costs an Amethyst Dust.

## [[Hexportation|Hexportation]]

- [[Send]] — `conduit, num → —`
- [[Metered Send]] — `conduit, [num, ...] → —`
- [[Slurp]] — `conduit → —`
- [[Spit]] — `conduit → —`
- [[Transplace]] — `conduit → —`

## [[Hexwright|Hexwright]]

- [[Coalesce]] — `vec, num → —`
- [[Wielder's Gambit]] — `num → —`
- [[Engraver's Gambit]] — `any → —`
- [[Talisman's Gambit]] — `num, num → —`
- [[Watcher's Gambit]] — `vec, entity/item type → —`
- [[Warden's Gambit]] — `[vec,vec] → —`
- [[Redirect]] — `entity, vec → —` · Costs 1/20 dust.
- [[Teal's Gambit]] — `[vec, vec], [vec, vec], num → —`
- [[Chorister's Gambit]] — `resonance or null, number, any → —`
- [[Chorister's Gambit (Hexwright)]] — `resonance or null, number, any → —`
- [[Ring Bearer's Gambit]] — `number, [pattern] → —`
- [[Emitter's Gambit]] — `vector, number, [pattern] | null → —`
- [[Tuner's Gambit]] — `vector, number | null → —`
- [[Manifest Dust]] — `num → —`
- [[Direct Dust]] — `vec → —`
- [[Form Dust]] — `region → —`
- [[Impel Dust]] — `vec → —`
- [[Move Dust]] — `vector → —`
- [[Constrain Dust]] — `bool → —`
- [[Recall Dust]] — `— → —`
- [[Manifold's Gambit]] — `item entity, num → —`
- [[Vault's Gambit]] — `— → —`
- [[Vault's Invitation]] — `player entity → —`
- [[Vault's Relocation Gambit]] — `vec → —`
- [[Artisan's Selection]] — `— → —`
- [[Construct's Gesture]] — `— → —`
- [[Teal's Dismissal]] — `— → —`
- [[Patron's Endowment]] — `— → —`
- [[Ward's Invitation]] — `— → —`

## [[Hierophantics|Hierophantics]]

- [[Induce Exhaustion]] — `villager → —` · Costs one Charged Amethyst.
- [[Stabilize Minds]] — `— → —` · Costs one Amethyst Dust per mind restored.
- [[Payload Gambit]] — `mind, [pattern] | null → —` · Costs one Charged Amethyst.
- [[Trigger Gambit]] — `mind, trigger | null → —` · Costs one Amethyst Shard.
- [[Mute Mind]] — `mind, bool → —`
- [[Free Mind]] — `mind → —`

## [[IoticBlocks|IoticBlocks]]

- [[Tachygrapher's Gambit]] — `entity | vec, num, any → —`

## [[Lapisworks|Lapisworks]]

- [[Tune Amethyst]] — `vec, any → —` · Costs a piece of Charged Amethyst and one Amel.
- [[Imbue with Amel]] — `num → —` · Costs 2x the amount of Amel being infused, in dust.
- [[Disenchant]] — `— → —`
- [[Sever Connection]] — `— → —`
- [[Reestablish Connection]] — `— → —`
- [[Tune Ritual]] — `any → —`
- [[Sieve Thoughts]] — `vec → —`
- [[Mind Liquefaction]] — `vec → —`
- [[Phiangle]] — `vec, vec → —` · Costs a flat three Charged Amethyst, and a scalable one Amel per 32 blocks of distance, with a minimum of one.
- [[Dephiangle]] — `vec, vec → —` · Costs three Charged Amethyst.
- [[Allocate Item]] — `wisp, item entity → —` · Costs two and a half dust.
- [[Command Drop]] — `wisp → —` · Costs two and a half dust.
- [[Imbue Lapis]] — `— → —` · Costs one Amethyst Shards in media and in item form (taken from my inventory) each, per Lapis.
- [[Reclaim Amethyst]] — `— → —` · Costs one shard.
- [[Mold Amel]] — `— → —`
- [[Enchant Skin]] — `entity, num → —` · cost is the number given and the cost in media is twice the expended Amel.
- [[Enchant Fists]] — `entity, num → —`
- [[Enchant Feet]] — `entity, num → —` · Costs 1x Amel and 5x Amethyst Shards.
- [[Enchant Arms]] — `entity, bool → —` · Costs 16 Amel (or 0 if disenchanting) and four shards in media to change that player.
- [[Strip Enchantment]] — `entity, num → num`
- [[Enchant with FireyFists]] — `entity → —` · Costs 10 Charged Amethyst 48 Amel.
- [[Enchant with Lightningbending]] — `entity → —`
- [[Envelop Feet in Amel]] — `entity → —`
- [[Grant Amel Air Sacs]] — `entity → —`
- [[Make Fire-born]] — `entity → —`
- [[Deposit Media]] — `M.T.I, num → —` · Costs the media deposited, plus a tax of 10%% of it.
- [[Withdraw Media]] — `M.T.I, num → —` · Costs 10%% of the media withdrawn.
- [[Amanuensis' Gambit]] — `any, num → —`
- [[Conjure Solid Color]] — `vec, num → —`
- [[Dispel Conjureable]] — `vec → —`
- [[Pull]] — `entity, vec, num → —`

## [[MoreIotas|MoreIotas]]

- [[Write]] — `vec, str | [str] → —` · Costs a hundredth of an Amethyst Dust.
- [[Name]] — `str, entity → —` · Costs a hundredth of an Amethyst Dust.

## [[Moves the Mind|Moves the Mind]]

- [[Halt Creature]] — `player → —`
- [[Summon Creature]] — `vec → —` · Costs a charged amethyst.
- [[Banish Creature]] — `player → —`
- [[Bid Walking]] — `player, num → —`
- [[Bid Strafing]] — `player, num → —`
- [[Bid Looking]] — `player, vec → —`
- [[Bid Jumping]] — `player → —`
- [[Bid Sneaking]] — `player, bool → —`
- [[Bid Sprinting]] — `player, bool → —`
- [[Bid Use Item]] — `player, num → —`
- [[Bid Attack]] — `player, num → —`
- [[Bid Select Toolbar]] — `player, num → —`
- [[Bid Swap Hands]] — `player → —`
- [[Bid Drop Item]] — `player → —`

## [[Oneironaut|Oneironaut]]

- [[Conduct Rod]] — `entity, list of patterns → —` · Costs 10 Charged Amethyst.
- [[Craft Bottomless Trinket]] — `list of patterns → —` · Costs ten charged amethyst.
- [[Cellular Generation]] — `— → —`
- [[Circle]] — `— → —`

## [[Slate Works|Slate Works]]

- [[Lay Item]] — `entity → —` · Costs an 1/8th of a dust per activated Vessel.
- [[Exchange Mind]] — `vector, entity → —` · Costs 1 Charged Amethyst to reapply the knowledge of Working.

## [[Special Efhexs|Special Efhexs]]

- [[Conjure Particle]] — `identifier, vector, vector → —` · Costs about 1/32th of an Amethyst Dust.
- [[Conjure Dust]] — `..., vector, number → —`
- [[Conjure Sparkly Dust]] — `..., vector, vector, number → —`
- [[Conjure Debris]] — `..., identifier → —`
- [[Conjure Sediment]] — `..., identifier → —`
- [[Conjure Fragments]] — `..., item → —`
- [[Conjure Media]] — `..., vector → —`
- [[Play Sound]] — `id, vec, num, num → —` · Costs about a sixteenth of an Amethyst Dust.
- [[Haunt]] — `null | list of entities → —`

## [[YAHA|YAHA]]

- [[Conjure Paper Plane]] — `vector, entity → entity` · Costs one Amethyst Dust.
- [[Conjure Time Bomb]] — `vec, [pattern], num, num → entity` · Costs one Charged Amethyst, plus the media given to the bomb.
- [[Conjure Anvil]] — `vec → entity` · Costs one Charged Amethyst.
- [[Conjure Trident]] — `vec → entity` · Costs one Charged Amethyst.
- [[ShowHide Arms]] — `entity → —`
- [[ShowHide Baseplate]] — `entity → —`
- [[ShrinkGrow Armor Stand]] — `entity → —`
- [[Rotate Armor Stand]] — `entity, number → —`
- [[Orient HeadBody]] — `entity, vector → —`
- [[Orient LeftRight Arm]] — `entity, vector → —`
- [[Orient LeftRight Leg]] — `entity, vector → —`
- [[Bury Item]] — `vector, entity → —` · Costs an eighth of an Amethyst Dust.
- [[Pluck]] — `entity → —` · Costs one Charged Amethyst.
- [[Orient Body]] — `— → —`
- [[Orient Head]] — `— → —`
- [[Orient Left Arm]] — `— → —`
- [[Orient Left Leg]] — `— → —`
- [[Orient Right Arm]] — `— → —`
- [[Orient Right Leg]] — `— → —`
