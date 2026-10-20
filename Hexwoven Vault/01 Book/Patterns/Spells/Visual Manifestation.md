---
aliases: ["Visual Manifestation"]
tags: [hexbook/entry, mod/hexintent, category/spells, topic/visuals]
mod: "HexIntent"
category: "Spells"
entry_id: "hexcasting:manifestation/visual_manifestation"
---

# Visual Manifestation

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[HexIntent]]

## [[Equation Hex Cloud|Equation Hex Cloud]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 53.7 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 22.2,61.0 40.7,61.0 31.5,77.0 22.2,61.0 13.0,45.0 22.2,29.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.8s" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 22.2,61.0 40.7,61.0 31.5,77.0 22.2,61.0 13.0,45.0 22.2,29.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="45.0" x2="34.4" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,36.2 36.2,40.9 32.7,38.9" fill="#fc77be"/><circle cx="31.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`vec|entity, equation, num` → `none`

Creates large particle effects from an Equation Particle [[Iotas|iota]] to avoid large vector lists. The Iota can be created using an Equation Synthesizer block. Uses an ID to keep positional state like Hex Trail.

Inputs:
target: vector where the point cloud is created,
OR
anchor: entity center to follow,
OR
anchor + offset: entity center plus offset vector.
equation: Equation Particle iota that defines point layout and colour behavior.
id: numeric cloud id.

Using the same id smoothly moves the cloud between positions.

## [[Hex Trail|Hex Trail]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="44" height="90" viewBox="0 0 44.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,45.0 31.5,29.0 22.2,13.0 13.0,29.0 22.2,45.0 13.0,61.0 31.5,61.0 22.2,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:130;--dur:2.0s" points="22.2,45.0 31.5,29.0 22.2,13.0 13.0,29.0 22.2,45.0 13.0,61.0 31.5,61.0 22.2,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="45.0" x2="25.2" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.3,36.2 26.9,40.9 23.4,38.9" fill="#fc77be"/><circle cx="22.2" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`vec, vec, vec, num, num, num?` → `none`

Hex Trail creates a particle trail as it moves. By using the same ID across casts you can create a continuous trail that moves smoothy between positions.

The positions have to be within 10 blocks of each other for the trail to smoothy move between them, otherwise it will snap to the new position. You also specify two colours for the trail to cycle between and a duration in ticks for the colour transition. There are a variety of particle types to choose from, some new and some from vanilla Minecraft. Not all particles can be tinted. The ID allows you to have multiple trails active and make sure they all animate independently.

Inputs:
position: vector for the particle to appear at or move to.
color from: RGB vector (0..1) at the start of the color cycle.
color to: RGB vector (0..1) at the end of the color cycle.
duration: time in ticks it takes for the colour to transition
id: trail identifier. This is what's used to look up the last position and animate between the position.
type?: optional particle style as a number (default 0).

Types:
0 dust ribbon
1 sparks
2 trail
3 pulse beads
4 end rod
5 witch plume
6 enchant flecks
7 soul flames
8 solid beams
9 ring lattices
10 lightning arcs
11 hex conjure ribbons
12 hex conjure sparkss
13 hex conjure trails
14 dual-helix.

## [[Manifest Spell Circle|Manifest Spell Circle]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 36.7,26.1 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:317;--dur:5.3s" points="36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 36.7,26.1 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="26.1" x2="34.0" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,34.0 32.4,29.8 35.6,31.6" fill="#fc77be"/><circle cx="36.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`[pattern], vec, vec, num, num?, vec?` → `none`

Projects a solid [[Spell Circles|spell circle]] into the world displaying a pattern for a duration before disintegrating.
Cost: 1/500 dust.

Inputs:
patterns: non-empty list of patterns to embed around the ring (max 48).
origin: vector for circle center.
facing: vector used as the circle plane.

duration: lifetime in ticks (1..1200).

size?: optional number 1..6 (default 3).

---
### Patterns on this page

- [[Equation Hex Cloud]] · Pattern · Other
- [[Hex Trail]] · Pattern · Other
- [[Manifest Spell Circle]] · Pattern · Other

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
