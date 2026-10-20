---
aliases: ["Pattern Manipulation"]
tags: [hexbook/entry, mod/hexical, category/patterns, topic/control-flow]
mod: "Hexical"
category: "Patterns"
entry_id: "hexcasting:patterns/pattern_manipulation"
---

# Pattern Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hexical]]

I have discovered a set of patterns that concern the construction and manipulation of other [[Escaping Patterns|patterns]]. Strangely, they can be used to construct patterns that are *impossible* to draw with a [[Staves|Staff]], although fortunately I don't believe Nature has associated an effect with any of these "illegal" patterns. They can be considered free to use with my [[Scarab Beetle]].

## [[Glyphmaker's Distillation|Glyphmaker's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 31.5,61.0 50.0,61.0 59.2,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:148;--dur:2.2s" points="31.5,29.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0 31.5,61.0 50.0,61.0 59.2,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="29.0" x2="34.4" y2="23.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,20.2 36.2,24.9 32.7,22.8" fill="#fc77be"/><circle cx="31.5" cy="29.0" r="2.6" fill="#fc77be"/></svg>

`pattern, number` → `pattern`

Uses the number to shuffle the pattern into one of the same shape, but different stroke order. My notes hint that ancient **Hexcasters** used this for some *grand library*.

## [[Congruence Distillation|Congruence Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 63.2,45.0 48.8,20.1 34.5,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="34.5,45.0 63.2,45.0 48.8,20.1 34.5,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="43.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.3,45.0 43.7,48.2 43.7,41.8" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`pattern, pattern` → `boolean`

Pushes whether two patterns have the same shape, stroke order, and orientation.

## [[Similarity Distillation|Similarity Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 48.8,69.9 34.5,45.0 48.8,20.1 63.2,45.0 34.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 48.8,69.9 34.5,45.0 48.8,20.1 63.2,45.0 34.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="29.3" y2="69.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.9,69.9 29.3,73.0 29.3,66.7" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`pattern, pattern` → `boolean`

Pushes whether two patterns have same shape but not necessarily the same stroke order.

## [[Chirographer's Purification|Chirographer's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,44.7 31.2,44.7 49.6,44.7 58.8,28.8 40.4,28.8 31.2,12.9 49.6,12.9 58.8,28.8 77.1,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:165;--dur:2.5s" points="12.9,44.7 31.2,44.7 49.6,44.7 58.8,28.8 40.4,28.8 31.2,12.9 49.6,12.9 58.8,28.8 77.1,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="44.7" x2="18.7" y2="44.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,44.7 18.7,46.7 18.7,42.6" fill="#fc77be"/><circle cx="12.9" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`pattern` → `list of numbers`

Turns a pattern into a list of numbers for my splitting, analysis, and dissection. The inverse of [[Calligrapher's Purification]].

## [[Calligrapher's Purification|Calligrapher's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="12.9,44.7 31.2,44.7 49.6,44.7 58.8,28.8 49.6,12.9 31.2,12.9 40.4,28.8 58.8,28.8 77.1,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:165;--dur:2.5s" points="12.9,44.7 31.2,44.7 49.6,44.7 58.8,28.8 49.6,12.9 31.2,12.9 40.4,28.8 58.8,28.8 77.1,28.8 68.0,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="12.9" y1="44.7" x2="18.7" y2="44.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.0,44.7 18.7,46.7 18.7,42.6" fill="#fc77be"/><circle cx="12.9" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`list of numbers` → `pattern`

Turns a list of numbers into a pattern for my viewing and execution. The inverse of [[Chirographer's Purification]].

The numbers returned by [[Chirographer's Purification]] correspond to individual strokes in the pattern. The diagonal stroke that travels upwards and rightwards is labeled 0 and all the other strokes are labelled incrementally in a clockwise manner.

If I pass in a list of numbers outside the 0-5 range into [[Calligrapher's Purification]], it will apply a [[Modulus Distillation|modulo]] to coerce numbers into the 0-5 range.

These lists have multiple fascinating properties. By adding or subtracting constant values, I can rotate the pattern. I can splice off the beginning and ends to check for pattern prefixes and suffixes. In cyclical patterns, I can rotate the list by moving the first element to the end or vice versa to cycle the starting position. By carefully altering numbers with list manipulation, I can also make some kind of *backwards stroke*.

## [[Handwriting Distillation|Handwriting Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="68.5" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="68.5" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/><circle cx="68.5" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="13.0,77.0 22.2,61.0 40.7,61.0 31.5,45.0 50.0,45.0 59.2,29.0 50.0,13.0 31.5,13.0 40.7,29.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:148;--dur:2.2s" points="13.0,77.0 22.2,61.0 40.7,61.0 31.5,45.0 50.0,45.0 59.2,29.0 50.0,13.0 31.5,13.0 40.7,29.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="13.0" y1="77.0" x2="15.9" y2="71.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.0,68.2 17.7,72.9 14.1,70.9" fill="#fc77be"/><circle cx="13.0" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`pattern` → `list of vectors`

Takes a pattern and produces a normalized list of vectors. The z-component of these vectors is zero. I feel this may be useful for artistic **Hexes**.

---
### Patterns on this page

- [[Glyphmaker's Distillation]] · Pattern · Distillation
- [[Congruence Distillation]] · Pattern · Distillation
- [[Similarity Distillation]] · Pattern · Distillation
- [[Chirographer's Purification]] · Pattern · Purification
- [[Calligrapher's Purification]] · Pattern · Purification
- [[Handwriting Distillation]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/pattern_manipulation)
