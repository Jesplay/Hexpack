---
tags: [guide]
---

# Example Hexes

Small, complete hexes you can draw right away, with what the [[The Stack|stack]] looks like after every pattern (top of the stack on the **right**). Open a pattern's note to see how to draw it; the drawing animates.

> [!tip] The universal "where am I looking" opener
> Steps 1–5 of the first hex appear in almost everything. Learn them once and you have half of every hex.

## 1. Torch where I look

| # | Pattern | Stack afterwards |
|---|---|---|
| 1 | [[Mind's Reflection]] | `me` |
| 2 | [[Compass' Purification]] | `eye-pos` |
| 3 | [[Mind's Reflection]] | `eye-pos, me` |
| 4 | [[Alidade's Purification]] | `eye-pos, look-dir` |
| 5 | [[Archer's Distillation]] | `block-pos` |
| 6 | [[Conjure Light]] | *(empty)* |

Swap step 6 for [[Break Block]] for a **mining laser**, or for [[Numerical Reflection]] `2` + [[Explosion]] for… an explosion.

> [!example]- Shorter version with stack tricks
> Instead of drawing [[Mind's Reflection]] twice, copy it: [[Mind's Reflection]] → [[Gemini Decomposition]] (`me, me`) → [[Compass' Purification]] (`me, eye-pos`) → [[Jester's Gambit]] (`eye-pos, me`) → [[Alidade's Purification]] → [[Archer's Distillation]].

## 2. Rocket jump

| # | Pattern | Stack afterwards |
|---|---|---|
| 1 | [[Mind's Reflection]] | `me` |
| 2 | [[Vector Rfln. +Y-Y]] | `me, (0,1,0)` |
| 3 | [[Numerical Reflection]] `3` (`aqaawww`) | `me, (0,1,0), 3` |
| 4 | [[Multiplicative Distillation]] | `me, (0,3,0)` |
| 5 | [[Impulse]] | *(empty)*, and you fly up |

Replace step 2 with [[Mind's Reflection]] → [[Alidade's Purification]] to **dash** in the direction you're looking.

## 3. Short-range teleport

| # | Pattern | Stack afterwards |
|---|---|---|
| 1 | [[Mind's Reflection]] | `me` |
| 2 | [[Numerical Reflection]] `8` (`aqaawaaa`) | `me, 8` |
| 3 | [[Blink]] | *(empty)*, 8 blocks forward |

## 4. Place a block against the face I'm looking at

| # | Pattern | Stack afterwards |
|---|---|---|
| 1–5 | the opener from hex 1 | `block-pos` |
| 6–10 | the opener again, but end with [[Architect's Distillation]] | `block-pos, face-normal` |
| 11 | [[Additive Distillation]] | `spot-in-front` |
| 12 | [[Place Block]] | *(empty)* |

## 5. Launch every creature around me

Uses [[Thoth's Gambit]] to repeat a mini-hex for each entity.

| # | Pattern | Stack afterwards |
|---|---|---|
| 1 | [[Introspection]] | *(start recording)* |
| 2 | [[Vector Rfln. +Y-Y]] | *(recorded, not run)* |
| 3 | [[Impulse]] | *(recorded, not run)* |
| 4 | [[Retrospection]] | `[Vector Rfln, Impulse]` |
| 5 | [[Mind's Reflection]] → [[Compass' Purification]] | `[…], my-pos` |
| 6 | [[Numerical Reflection]] `5` | `[…], my-pos, 5` |
| 7 | [[Zone Distillation Living]] | `[…], [entities]` |
| 8 | [[Thoth's Gambit]] | `[]`: each entity got `(0,1,0)` pushed onto it, then [[Impulse]] |

> [!warning] You are a living creature too
> You'll get launched as well. Filter the list first, or use [[Zone Distillation Monster]] instead.

## 6. Save a hex to a Focus and reuse it

1. Hold an empty [[Focus]] in your off-hand.
2. [[Introspection]] → *(draw hex 1, steps 1–6)* → [[Retrospection]], which leaves the whole hex as a [[List]].
3. [[Scribe's Gambit]]: the list is written into the focus.

Later, with the focus in your off-hand: [[Scribe's Reflection]] → [[Hermes' Gambit]] runs the saved hex. Two patterns instead of six. For a real one-click item, see [[Casting Items]] ([[Craft Cypher]], [[Craft Trinket]], [[Craft Artifact]]).

## Where to go next
- [[Meta-Evaluation]]: loops and running hexes as data
- [[Reading and Writing]]: storing iotas
- [[Patterns by Family]]: browse everything by what it does to the stack
