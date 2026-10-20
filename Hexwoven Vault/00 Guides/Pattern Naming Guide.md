---
tags: [guide]
---

# Pattern Naming Guide

Hex Casting names its patterns by **what they do to the stack**. Once you know the suffixes, a name alone tells you how a pattern behaves. (The full in-book explanation is [[Naming Actions]].)

| Suffix | Takes → Leaves | Example |
|---|---|---|
| **Reflection** | nothing → 1 iota | [[Mind's Reflection]] pushes the caster |
| **Purification** | 1 → 1 | [[Compass' Purification]]: entity → its position |
| **Distillation** | 2 → 1 | [[Additive Distillation]]: a, b → a + b |
| **Exaltation** | 3+ → 1 | [[Augur's Exaltation]]: bool, a, b → a or b |
| **Decomposition** | 1 → 2 | [[Gemini Decomposition]]: duplicates the top iota |
| **Disintegration** | 1 → many | [[Flock's Disintegration]]: unpacks a list onto the stack |
| **Gambit** | rearranges the stack / controls evaluation | [[Jester's Gambit]], [[Hermes' Gambit]] |
| **Ploy** / other | addon-specific | |
| *(plain name)* | **spells**: change the world, cost media | [[Explosion]], [[Impulse]], [[Blink]] |

The name before the suffix is usually a **figure or object** whose meaning hints at the effect: *Compass* = where something is, *Alidade* = which way it faces, *Archer* = what's in the line of sight, *Scribe* = reading and writing, *Hermes* = carrying out (evaluating) a hex, *Thoth* = repeating (for-each), and so on.

## Escaping
Normally a drawn pattern runs right away. To push a pattern *as data* instead (to build lists of patterns, write them to a focus, and so on):
- [[Consideration]]: escapes the very next pattern
- [[Introspection]] … [[Retrospection]]: escapes everything between them as a list

See [[Escaping Patterns]] and [[Advanced Escaping]].
