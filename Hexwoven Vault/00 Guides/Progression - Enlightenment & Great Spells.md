---
tags: [guide]
aliases: ["Enlightenment", "How to get enlightened", "Mote Nexus guide", "Great Spells guide", "Progression"]
---

# Progression: Enlightenment, Great Spells & Flaying

Stuck on something like the **Mote Nexus**? Almost every late-game block in Hex Casting and its addons is gated the same way:

```text
find a Great Spell scroll ─► fail to cast it ─► overcast with your health ─► ENLIGHTENMENT
                                                                            │
                         Flay Mind (a Great Spell) ◄────────────────────────┘
                                   │
            villager + block ─► Mote Nexus, Impetuses, Directrices, Akashic Records, Budding Amethyst…
```

The steps below come straight from the mod's advancement files, not from guesswork.

## Step 1: Find Great Spell scrolls
Great Spells (like [[Flay Mind]]) have **different strokes in every world**, so you must find their **Ancient Scrolls**. In this modpack they're injected into these loot tables (the number is the scroll count the pack config gives that table, so higher means more):

| Where | Count |
|---|---|
| Stronghold library, Woodland mansion | 5 |
| Ancient city, Pillager outpost | 4 |
| Shipwreck map chest, Bastion treasure, End city | 3 |
| Jungle temple, Desert pyramid, Village **cartographer** house | 2 |
| Dungeon, Mineshaft, Nether fortress, Bastion | 1 |

**Wandering traders** also have a 20% chance to sell one. Put [[Amethyst|amethyst dust]] on a scroll hung on a wall to see its stroke order. See [[Scrolls]].

## Step 2: Fail to cast it *(advancement "Blind Diversion")*
Draw the Great Spell from the scroll. It **will fail**; that's intended. This unlocks **casting with your own health** when you run out of amethyst (see [[A Frustration]]).

## Step 3: Overcast once *(advancement "Opened Eyes")*
Cast any hex with **no amethyst in your inventory**, so it takes health instead. Your full health is worth about **2 Charged Amethyst = 20 dust**, so roughly **1 dust per half-heart**.

## Step 4: Enlightenment *(advancement "Achieve Enlightenment")*
The exact rule: a **single hex must use at least 80% of your max health, and you must survive it**.

> [!warning] Do this carefully, it can kill you
> - Be at **full health** with **no amethyst on you** (put it in a chest), so the whole cost comes out of your health.
> - Pick a hex costing **about 16–18 dust** (80–90% of 20). Example: [[Explosion]] with power `6` aimed **far away from you** costs 6 × 3 = 18 dust. (Explosions don't hurt the caster, per the book.)
> - Too cheap (under 16) and nothing happens; too expensive (20+) and you die.
> - Do it somewhere safe (no mobs, no fall risk), since you'll be left with very little health.

After this you can cast Great Spells and some other enlightened-only patterns. See [[WHAT DID I SEE]] and [[The Work]] for the in-book version.

## Step 5: Flay Mind
[[Flay Mind]] takes **a villager and a block position**: `villager, block-pos → (the block changes)`. It costs **10 Charged Amethyst** per flay.
1. Get the right villager (profession + level from the table below) within reach of where you'll cast. Trap it in a boat or a 1×1 hole so it can't wander off.
2. Place the target block.
3. Hex: target the villager ([[Scout's Distillation]] while looking at it), then target the block ([[Archer's Distillation]]), then **Flay Mind**.

> [!note] Villagers notice
> This pack has `villagersOffendedByMindMurder` on, so nearby villagers get angry if they see it. Flay out of their sight.

Villager levels: 1 Novice · 2 Apprentice · 3 Journeyman · 4 Expert · 5 Master (trade with them to level them up).

## Every Flay Mind recipe in the pack

| Result | Villager / creature | Block to flay into | Cost | Mod | Book page |
|---|---|---|---|---|---|
| **Akashic Record** | Librarian villager, level 5+ (Master) | Akashic Ligature | 10 charged | Hex Casting | [[Akashic Libraries]] |
| **Budding Amethyst** | Any villager, level 3+ (Journeyman) | Amethyst Block | 10 charged | Hex Casting | [[On The Flaying of Minds]] |
| **Cleric Impetus** | Cleric villager, level 2+ (Apprentice) | Empty Impetus | 10 charged | Hex Casting | [[Impeti]] |
| **Fletcher Impetus** | Fletcher villager, level 2+ (Apprentice) | Empty Impetus | 10 charged | Hex Casting | [[Impeti]] |
| **Mason Directrix** | Mason villager, level 1+ (Novice) | Empty Directrix | 10 charged | Hex Casting | [[Directrices]] |
| **Quenched Allay** | Allay | Amethyst Block | 1 charged | Hex Casting | [[Quenching Allays]] |
| **Shepherd Directrix** | Shepherd villager, level 1+ (Novice) | Empty Directrix | 10 charged | Hex Casting | [[Directrices]] |
| **Toolsmith Impetus** | Toolsmith villager, level 2+ (Apprentice) | Empty Impetus | 10 charged | Hex Casting | [[Impeti]] |
| **Hexbox** | Librarian villager, level 4+ (Expert) | Block of Slate | 10 charged | Complex Hex | — |
| **Quenched Record** | Allay | Inert Record | 10 charged | Complex Hex | — |
| **Mindsplice Table** | Toolsmith villager, level 3+ (Journeyman) | Splicing Table | 10 charged | HexDebug | [[Mindsplice Table]] |
| **Splinter Caster** | Any villager, level 1+ (Novice) | Block of Slate | 10 charged | HexIntent | — |
| **Mote Nexus** | Cartographer villager, level 2+ (Apprentice) | Shulker Box | 10 charged | Hexal | [[Motes]] |
| **Imbuement Bed** | Allay | Imbuement Bed | 2.5 charged | Hierophantics | [[Allay Imbuement]] |
| **Imbuement Bed** | Any villager, level 1+ (Novice) | Imbuement Bed | 10 charged | Hierophantics | [[Villager Enhancement]] |
| **Imbuement Bed** | Any villager, level 2+ (Apprentice) | Imbuement Bed | 10 charged | Hierophantics | [[Hierophantics (Book Entry)]] |
| **Amalgamation Container** | Fisherman villager, level 5+ (Master) | Simple Mind Container | 10 charged | Lapisworks | [[Amalgamations]] |
| **Amalgamation Container** | Fisherman villager, level 3+ (Journeyman) | Simple Mind Container | 5 charged | Lapisworks | [[Amalgamations]] |
| **Budding Pseudoamethyst** | Any villager, level 5+ (Master) | Pseudoamethyst Block | 10 charged | Oneironaut | [[Better Media Farming]] |
| **Librarian Impetus** | Librarian villager, level 2+ (Apprentice) | Empty Impetus | 5 charged | Slate Works | [[Slate Work Impeti]] |
| **Patterned Assembler** | Librarian villager, level 2+ (Apprentice) | Crafting Table | 10 charged | Slate Works | [[Patterned Assemblers]] |
| **Pocket Simulator** | Allay | Slate | 1 charged | Slate Works | [[Pocket Simulator]] |
| **Replicated Allay** | Allay | Quenched Allay | 5 charged | Slate Works | [[Slate Works Deco Blocks]] |
| **Sentinel Cache** | Any villager, level 5+ (Master) | Ambit Extender | 20 charged | Slate Works | [[Sentinel Cache]] |
| **Spell Imprinter** | Weaponsmith villager, level 2+ (Apprentice) | Slate | 10 charged | Slate Works | [[Spell Imprinter]] |
| **Storage Vessel** | Fisherman villager, level 1+ (Novice) | Slate | 10 charged | Slate Works | [[Storage Vessels]] |

> [!tip] Allays
> Recipes that list **Allay** use an allay instead of a villager; see [[Quenching Allays]].
