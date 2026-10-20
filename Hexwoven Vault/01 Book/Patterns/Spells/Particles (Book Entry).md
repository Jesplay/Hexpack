---
aliases: []
tags: [hexbook/entry, mod/special-efhexs, category/spells, topic/visuals]
mod: "Special Efhexs"
category: "Spells"
entry_id: "hexcasting:patterns/spells/particles"
---

# Particles

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Special Efhexs]]

Somehow, my [[Microphone]] is able to record the particles in the air! I can query and replay them to add a bit of flair to my **Hexes** for essentially free.

*All those moments will be lost in time, like tears in rain.*

## [[Particulate Reflection|Particulate Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 38.3,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="15.8,74.2 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 38.3,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`—` → `list of identifiers`

Pushes the last sixteen unique particles picked up by my [[Microphone]].

## [[Conjure Particle|Conjure Particle]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="38.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,57.7 40.0,62.9 36.1,60.7" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`identifier, vector, vector` → `—`

Conjures a particular particle with a specified position and velocity (blocks per twentieth of a second). Costs about 1/32th of an [[Amethyst|Amethyst Dust]].

This spell only works for "simple" particles, which are the majority. The remaining are too complicated to be boiled down into just a position and velocity but fortunately, I have documented various patterns for these more complicated effects.

They are all priced like the general spell and take a position and a velocity (compressed as "...") along with extra parameters.

## [[Conjure Dust|Conjure Dust]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,74.2 38.3,54.7 60.8,54.7 72.0,35.3 60.8,15.8 38.3,15.8 27.0,35.3 38.3,54.7 49.5,35.3 27.0,35.3 15.8,54.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:2.8s" points="27.0,74.2 38.3,54.7 60.8,54.7 72.0,35.3 60.8,15.8 38.3,15.8 27.0,35.3 38.3,54.7 49.5,35.3 27.0,35.3 15.8,54.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="74.2" x2="30.6" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.2,63.5 32.8,69.2 28.5,66.8" fill="#fc77be"/><circle cx="27.0" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`..., vector, number` → `—`

Conjures a small mote of dust of a given color. The number relates vaguely to scale and lifespan.

## [[Conjure Sparkly Dust|Conjure Sparkly Dust]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="40.4,60.6 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 49.6,44.7 58.8,28.8 40.4,28.8 31.2,44.7 22.0,28.8 12.9,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:220;--dur:3.4s" points="40.4,60.6 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 49.6,44.7 58.8,28.8 40.4,28.8 31.2,44.7 22.0,28.8 12.9,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="60.6" x2="43.3" y2="55.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="45.5,51.8 45.1,56.5 41.6,54.5" fill="#fc77be"/><circle cx="40.4" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`..., vector, vector, number` → `—`

Conjures a gradient-like mote of dust, which smoothly transitions between the two colors given over the given number of seconds.

## [[Conjure Debris|Conjure Debris]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="12.9" cy="12.9" r="1.6"/><circle cx="31.2" cy="12.9" r="1.6"/><circle cx="49.6" cy="12.9" r="1.6"/><circle cx="68.0" cy="12.9" r="1.6"/><circle cx="86.3" cy="12.9" r="1.6"/><circle cx="3.7" cy="28.8" r="1.6"/><circle cx="22.0" cy="28.8" r="1.6"/><circle cx="40.4" cy="28.8" r="1.6"/><circle cx="58.8" cy="28.8" r="1.6"/><circle cx="77.1" cy="28.8" r="1.6"/><circle cx="12.9" cy="44.7" r="1.6"/><circle cx="31.2" cy="44.7" r="1.6"/><circle cx="49.6" cy="44.7" r="1.6"/><circle cx="68.0" cy="44.7" r="1.6"/><circle cx="86.3" cy="44.7" r="1.6"/><circle cx="3.7" cy="60.6" r="1.6"/><circle cx="22.0" cy="60.6" r="1.6"/><circle cx="40.4" cy="60.6" r="1.6"/><circle cx="58.8" cy="60.6" r="1.6"/><circle cx="77.1" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="40.4,60.6 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 49.6,44.7 58.8,28.8 40.4,28.8 22.0,28.8 12.9,44.7 22.0,60.6 40.4,60.6 31.2,44.7 40.4,28.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:276;--dur:4.2s" points="40.4,60.6 49.6,44.7 68.0,44.7 77.1,28.8 68.0,12.9 49.6,12.9 40.4,28.8 49.6,44.7 58.8,28.8 40.4,28.8 22.0,28.8 12.9,44.7 22.0,60.6 40.4,60.6 31.2,44.7 40.4,28.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.4" y1="60.6" x2="43.3" y2="55.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="45.5,51.8 45.1,56.5 41.6,54.5" fill="#fc77be"/><circle cx="40.4" cy="60.6" r="2.6" fill="#fc77be"/></svg>

`..., identifier` → `—`

Conjures a visual bit of debris, similar to that caused by breaking a block. Takes in a block identifier.

## [[Conjure Sediment|Conjure Sediment]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 55.2,32.0 34.8,32.0 24.5,49.7 14.3,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 55.2,32.0 34.8,32.0 24.5,49.7 14.3,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="38.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,57.7 40.0,62.9 36.1,60.7" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`..., identifier` → `—`

Conjures particles similar to that caused by suspended gravity-affected blocks. Takes in a block identifier.

## [[Conjure Fragments|Conjure Fragments]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 55.2,32.0 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="34.8,67.5 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3 45.0,14.3 34.8,32.0 45.0,49.7 55.2,32.0 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="38.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,57.7 40.0,62.9 36.1,60.7" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`..., item` → `—`

Conjures a visual bit of fragments, similar to that caused by a tool breaking after wearing. Takes in an item [[Iotas|iota]].

## [[Conjure Media|Conjure Media]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="68.5" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="77.7" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="68.5" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="77.7" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/><circle cx="68.5" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="13.0,77.0 22.2,61.0 40.7,61.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 22.2,61.0 31.5,45.0 50.0,45.0 68.5,45.0 59.2,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 31.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="13.0,77.0 22.2,61.0 40.7,61.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 22.2,61.0 31.5,45.0 50.0,45.0 68.5,45.0 59.2,29.0 50.0,13.0 31.5,13.0 13.0,13.0 22.2,29.0 31.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="13.0" y1="77.0" x2="15.9" y2="71.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.0,68.2 17.7,72.9 14.1,70.9" fill="#fc77be"/><circle cx="13.0" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`..., vector` → `—`

Conjures a blob of visible **[[Media|media]]**, not unlike the waste product of my **Hexes**. Takes in a color.

---
### Patterns on this page

- [[Particulate Reflection]] · Pattern · Reflection
- [[Conjure Particle]] · Spell · Spell
- [[Conjure Dust]] · Spell · Spell
- [[Conjure Sparkly Dust]] · Spell · Spell
- [[Conjure Debris]] · Spell · Spell
- [[Conjure Sediment]] · Spell · Spell
- [[Conjure Fragments]] · Spell · Spell
- [[Conjure Media]] · Spell · Spell

**Sources:** 🌐 [Official online docs](https://efhexs.hexxy.media/v/latest/main/en_us/#patterns/spells/particles)
