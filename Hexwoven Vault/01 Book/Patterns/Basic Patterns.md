---
aliases: ["Basic Patterns"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/debugging, topic/sensing, topic/text]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/basics"
---

# Basic Patterns

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Mind's Reflection|Mind's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,69.9 48.8,45.0 34.5,20.1 20.1,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="34.5,69.9 48.8,45.0 34.5,20.1 20.1,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="69.9" x2="39.1" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="42.4,56.2 41.8,63.5 36.3,60.3" fill="#fc77be"/><circle cx="34.5" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`—` → `entity | null`

Adds me, the caster, to the [[The Stack|stack]].

## [[Compass' Purification|Compass' Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="78" viewBox="0 0 83.1 78.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.2" r="1.6"/><circle cx="41.5" cy="24.2" r="1.6"/><circle cx="76.2" cy="24.2" r="1.6"/><circle cx="24.2" cy="54.2" r="1.6"/><circle cx="58.8" cy="54.2" r="1.6"/></g><polyline class="hexpat-base" points="24.2,54.2 58.8,54.2 41.5,24.2 24.2,54.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:104;--dur:1.6s" points="24.2,54.2 58.8,54.2 41.5,24.2 24.2,54.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.2" y1="54.2" x2="35.3" y2="54.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="43.3,54.2 35.3,58.0 35.3,50.4" fill="#fc77be"/><circle cx="24.2" cy="54.2" r="2.6" fill="#fc77be"/></svg>

`entity` → `vector`

Transforms an entity on the stack into the position of its eyes. I should probably use this on myself.

## [[Compass' Purification II|Compass' Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="78" viewBox="0 0 83.1 78.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.2" r="1.6"/><circle cx="41.5" cy="24.2" r="1.6"/><circle cx="76.2" cy="24.2" r="1.6"/><circle cx="24.2" cy="54.2" r="1.6"/><circle cx="58.8" cy="54.2" r="1.6"/></g><polyline class="hexpat-base" points="24.2,54.2 41.5,24.2 58.8,54.2 24.2,54.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:104;--dur:1.6s" points="24.2,54.2 41.5,24.2 58.8,54.2 24.2,54.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.2" y1="54.2" x2="29.8" y2="44.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.8,37.7 33.1,46.5 26.5,42.7" fill="#fc77be"/><circle cx="24.2" cy="54.2" r="2.6" fill="#fc77be"/></svg>

`entity` → `vector`

Transforms an entity on the stack into the position it is standing. I should probably use this on other entities.

## [[Alidade's Purification|Alidade's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:79;--dur:1.6s" points="18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="27.0" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,41.5 27.0,44.4 27.0,38.5" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `vector`

Transforms an entity on the stack into the direction it's looking in, as a unit vector.

## [[Archer's Distillation|Archer's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,36.1 39.2,36.1 62.3,36.1 73.8,16.2 50.8,16.2 62.3,36.1 73.8,56.1 50.8,56.1 62.3,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="16.2,36.1 39.2,36.1 62.3,36.1 73.8,16.2 50.8,16.2 62.3,36.1 73.8,56.1 50.8,56.1 62.3,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="36.1" x2="23.5" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.8,36.1 23.5,38.7 23.5,33.6" fill="#fc77be"/><circle cx="16.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`vector, vector` → `vector | null`

Combines two vectors (a position and a direction) into the answer to the question: If I stood at the position and looked in the direction, what block would I be looking at? Costs a negligible amount of **[[Media|media]]**.

If it doesn't hit anything, the vectors will combine into **Null**.

A common sequence of patterns, the so-called "raycast mantra," is [[Mind's Reflection]], [[Compass' Purification]], [[Mind's Reflection]], [[Alidade's Purification|Alidade Purification]], [[Archer's Distillation]]. Together, they return the vector position of the block I am looking at.

## [[Architect's Distillation|Architect's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,36.1 39.2,36.1 62.3,36.1 73.8,56.1 50.8,56.1 62.3,36.1 73.8,16.2 50.8,16.2 62.3,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="16.2,36.1 39.2,36.1 62.3,36.1 73.8,56.1 50.8,56.1 62.3,36.1 73.8,16.2 50.8,16.2 62.3,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="36.1" x2="23.5" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.8,36.1 23.5,38.7 23.5,33.6" fill="#fc77be"/><circle cx="16.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`vector, vector` → `vector | null`

Like [[Archer's Distillation]], but instead returns a vector representing the answer to the question: Which side of the block am I looking at? Costs a negligible amount of **media**.

More specifically, it returns the normal vector of the face hit, or a unit vector pointing perpendicular to the face.
- If I am looking at a floor, it will return (0, 1, 0).
- If I am looking at the south face of a block, it will return (0, 0, 1).

## [[Scout's Distillation|Scout's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 34.8,32.0 55.2,32.0 65.5,49.7 75.7,32.0 65.5,14.3 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:123;--dur:1.7s" points="14.3,32.0 34.8,32.0 55.2,32.0 65.5,49.7 75.7,32.0 65.5,14.3 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="20.9" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.6,32.0 20.9,34.3 20.9,29.8" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`vector, vector` → `entity | null`

Like [[Archer's Distillation]], but instead returns the entity I am looking at. Costs a negligible amount of **media**.

## [[Reveal|Reveal]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,45.0 34.5,20.1 48.8,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:86;--dur:1.6s" points="20.1,45.0 34.5,20.1 48.8,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="45.0" x2="24.7" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,31.3 27.4,38.6 22.0,35.5" fill="#fc77be"/><circle cx="20.1" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`any` → `any`

Displays the top [[Iotas|iota]] of the stack to me.

## [[Stadiometer's Purification|Stadiometer's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="62.3,36.1 73.8,16.2 50.8,16.2 27.7,16.2 16.2,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:92;--dur:1.6s" points="62.3,36.1 73.8,16.2 50.8,16.2 27.7,16.2 16.2,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="62.3" y1="36.1" x2="66.0" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="68.7,25.1 68.2,31.0 63.8,28.5" fill="#fc77be"/><circle cx="62.3" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`entity` → `num`

Transforms an entity on the stack into its height.

## [[Pace Purification|Pace Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,36.1 39.2,36.1 62.3,36.1 73.8,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:69;--dur:1.6s" points="16.2,36.1 39.2,36.1 62.3,36.1 73.8,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="36.1" x2="23.5" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.8,36.1 23.5,38.7 23.5,33.6" fill="#fc77be"/><circle cx="16.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`entity` → `vector`

Transforms an entity on the stack into the direction in which it's moving, with the speed of that movement as that direction's magnitude.

## [[Media Reflection (Hex Casting)|Media Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,45.0 20.1,45.0 34.5,20.1 48.8,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="48.8,45.0 20.1,45.0 34.5,20.1 48.8,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="45.0" x2="39.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.0,45.0 39.7,41.8 39.7,48.2" fill="#fc77be"/><circle cx="48.8" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Pushes the amount of **media** available to the current cast, in units of [[Amethyst|Amethyst Dust]].

---
### Patterns on this page

- [[Mind's Reflection]] · Pattern · Reflection
- [[Compass' Purification]] · Pattern · Purification
- [[Compass' Purification II]] · Pattern · Purification
- [[Alidade's Purification]] · Pattern · Purification
- [[Archer's Distillation]] · Pattern · Distillation
- [[Architect's Distillation]] · Pattern · Distillation
- [[Scout's Distillation]] · Pattern · Distillation
- [[Reveal]] · Pattern · Other
- [[Stadiometer's Purification]] · Pattern · Purification
- [[Pace Purification]] · Pattern · Purification
- [[Media Reflection (Hex Casting)]] · Pattern · Reflection

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/basics) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
