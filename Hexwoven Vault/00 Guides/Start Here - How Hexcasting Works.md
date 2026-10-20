---
tags: [moc, guide]
---

# Start Here: How Hexcasting Works

This page walks through the whole system in the order the book teaches it. Each heading links to the full book entry.

## 1. Media: the fuel
All magic runs on **media**, a kind of thought-energy. In the world it appears as **amethyst**:
- **Amethyst Dust** = 1 unit · **Amethyst Shard** = 5 dust · **Charged Amethyst** = 10 dust

You find it in [[Geodes|geodes]] (charged amethyst comes from crystal clusters). Spells take media from the amethyst in your inventory. Later, after the "Blind Diversion" unlock described in [[Progression - Enlightenment & Great Spells]], they can also take it from your **health** when you run out, which is dangerous. Before that, a hex without enough amethyst simply fails.
→ [[Media]] · [[Amethyst]] · [[Geodes]] · [[Phials of Media]] (rechargeable media batteries)

## 2. The staff and the hex grid
Hold a [[Staves|staff]] and **right-click** to open the casting grid, a field of dots in a hexagonal lattice. Drag across the dots to draw a **pattern**. When you release, the pattern runs instantly.
→ [[Hex Casting 101]]

Each pattern is defined by:
- its **start direction** (`EAST`, `NORTH_EAST`, …): the standard orientation it's shown in (any rotation works when you draw it)
- its **stroke signature**, a string of turns: `w` straight · `e` slight right · `d` sharp right · `s` back · `a` sharp left · `q` slight left

Every pattern note in this vault has a drawing of the pattern (the pink dot and arrow mark the start and the first stroke) plus its signature.

> [!tip] Rotation doesn't matter
> Hex Casting recognises a pattern **only by its sequence of turns**. You can draw it rotated any way you like. The start direction shown is just the standard orientation. A *mirrored* pattern, though, is a different pattern (or nothing at all).

## 3. The stack and iotas
Patterns communicate through **[[The Stack|the stack]]**, a pile of values called **[[Iotas|iotas]]**. A pattern pops its inputs off the top and pushes its outputs back.
- Example: [[Mind's Reflection]] pushes *you* (an [[Entity]]). [[Compass' Purification]] pops an entity and pushes its eye position (a [[Vector]]).
- Notation: `entity, vector → number` means "takes an entity and a vector (the vector on top) and leaves a number".

Common iota types: [[Number]] · [[Vector]] · [[Entity]] · [[Boolean]] · [[List]] · [[Pattern]] · [[Null]] (+ [[String]], [[Matrix]], etc. from addons)
→ [[A Primer On Vectors]] · [[Types]]

## 4. What is a Hex?
A **hex** is a sequence of patterns drawn one after another. A typical one gathers information, transforms it, then casts a spell with it. For example, "blow up the block I'm looking at":
1. [[Mind's Reflection]] → me
2. [[Compass' Purification]] → my eye position
3. [[Mind's Reflection]], [[Alidade's Purification]] → the direction I'm looking
4. [[Archer's Distillation]] → the block I'm looking at
5. [[Numerical Reflection]] → a strength number
6. [[Explosion]] 💥

→ [[What is a Hex]] · [[How to Read This Section]]

## 5. Kinds of patterns
Pattern names follow a strict naming scheme that tells you their stack effect. See **[[Pattern Naming Guide]]**. The big groups:
- **Basic & constants**: getting information ([[Basic Patterns]], [[Constants]], [[Number Literals]])
- **Math & logic**: [[Mathematics]], [[Advanced Mathematics]], [[Logical Operators]], [[Sets]]
- **Stack manipulation**: [[Stack Manipulation]] (e.g. [[Jester's Gambit]], [[Gemini Decomposition]])
- **Lists & patterns as data**: [[List Manipulation]], [[Escaping Patterns]] (Consideration / Introspection / Retrospection)
- **Meta-evaluation**: running hexes from data, loops and recursion ([[Meta-Evaluation]]: [[Hermes' Gambit]], [[Thoth's Gambit]])
- **Reading & writing**: storing iotas in items like foci ([[Reading and Writing]])
- **Entities**: finding entities near a point ([[Entities]])
- **Spells**: patterns that actually *do* something and cost media. See [[All Spells]].
- **Great Spells**: hugely powerful, randomised per world, gated behind enlightenment. See [[All Great Spells]].

## 6. Mishaps
If something goes wrong (wrong iota type, not enough items on the stack, an invalid pattern…), the hex **mishaps**: it stops, you get a negative effect, and a [[Garbage]] iota may appear.
→ [[Mishaps]] · [[Enlightened Mishaps]]

## 7. Storing and reusing hexes
Drawing everything by hand gets old fast. These items store iotas or whole hexes:
- [[Focus]]: stores one iota (often a list of patterns). Read it with [[Scribe's Reflection]], write it with [[Scribe's Gambit]].
- [[Spellbook]]: many pages of foci in one item
- [[Scrolls]] and [[Slates]]: a single pattern on paper or stone
- [[Casting Items]]: **Cyphers** (single-use), **Trinkets** (rechargeable), **Artifacts** (draw from your inventory). Make them with [[Craft Cypher]], [[Craft Trinket]], [[Craft Artifact]].
- [[Phials of Media]]: portable media storage. See also [[Pigments]] (spell particle colour).

## 8. The Great Work
The late game lives in [[The Great Work (Category)|The Great Work]]:
- [[Spell Circles]]: hexes laid out in slates and powered by an [[Impeti|impetus]], with [[Directrices|directrices]] to route them. They run without you.
- [[Akashic Libraries]]: a block database that maps patterns to iotas
- [[On The Flaying of Minds]]: transferring a villager's mind into a block ([[Flay Mind]])
- [[The Work]]: **enlightenment**, which unlocks Great Spells and lets you draw outside your normal ambit
- [[Quenching Allays]] · [[Fanciful Staves]]

## 9. Addons in this pack
Hexwoven adds a lot on top: wisps and motes ([[Hexal]]), strings and matrices ([[MoreIotas]]), lamps and many utilities ([[Hexical]]), dimensions and dream magic ([[Oneironaut]]), a separate crafting and weapons system ([[Hexwright]]), lapis enchantment magic ([[Lapisworks]]) and more. See [[Hexwoven Modpack]].
