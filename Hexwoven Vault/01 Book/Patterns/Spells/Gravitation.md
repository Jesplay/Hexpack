---
aliases: ["Gravitation"]
tags: [hexbook/entry, mod/hexodus, category/spells, topic/movement]
mod: "Hexodus"
category: "Spells"
entry_id: "hexcasting:patterns/spells/gravity"
---

# Gravitation

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexodus]]

While I can use a [[Ballasts|Ballast]] to alter **Gravity**'s effects on my own body, I have also discovered a spell capable of adjusting **Gravity**'s pull on *any* entity, albeit very shortly.

*There is an art, it says, or rather, a knack to flying. The knack lies in learning how to throw yourself at the ground and miss*.

## [[Gravitate|Gravitate]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,56.1 62.3,36.1 73.8,16.2 50.8,16.2 27.7,16.2 39.2,36.1 50.8,56.1 73.8,56.1 62.3,36.1 39.2,36.1 16.2,36.1 27.7,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:254;--dur:3.1s" points="50.8,56.1 62.3,36.1 73.8,16.2 50.8,16.2 27.7,16.2 39.2,36.1 50.8,56.1 73.8,56.1 62.3,36.1 39.2,36.1 16.2,36.1 27.7,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="56.1" x2="54.5" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="57.1,45.1 56.7,51.0 52.3,48.5" fill="#fc77be"/><circle cx="50.8" cy="56.1" r="2.6" fill="#fc77be"/></svg>

`entity, vector, number` → `—`

Takes an entity, an axial vector for new gravity, and a strength scalar relative to standard pull. Specifics are detailed on the next page.

The scalar can be any number 0 to 5 inclusive.

If the target is myself, this spell costs about one-fourth of an [[Amethyst|Amethyst Dust]]. If the target is another player, costs about one [[Amethyst|Amethyst Shard]]. Otherwises, costs about one [[Amethyst|Amethyst Dust]].

Each cast of the spell only lasts for about two and a half seconds; for sustained gravitation, I will need some kind of automated **Hexcasting** to periodically reapply the effects.

## [[Grasp Purification|Grasp Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 15.8,54.7 27.0,35.3 49.5,35.3 72.0,35.3 60.8,54.7 49.5,74.2 72.0,74.2 60.8,54.7 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:338;--dur:4.2s" points="38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 15.8,54.7 27.0,35.3 49.5,35.3 72.0,35.3 60.8,54.7 49.5,74.2 72.0,74.2 60.8,54.7 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="54.7" x2="34.7" y2="48.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,44.0 36.8,47.3 32.5,49.7" fill="#fc77be"/><circle cx="38.3" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`entity` → `number`

Pushes the remaining twentieths of a second that [[Gravitate]] will affect an entity. Zero if the entity is not under the effects of [[Gravitate]].

## [[Falling Purification|Falling Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 24.5,49.7 14.3,32.0 34.8,32.0 55.2,32.0 45.0,49.7 34.8,67.5 14.3,67.5 24.5,49.7 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="34.8,67.5 24.5,49.7 14.3,32.0 34.8,32.0 55.2,32.0 45.0,49.7 34.8,67.5 14.3,67.5 24.5,49.7 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="31.5" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,57.7 33.4,60.7 29.6,62.9" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `vector`

Pushes the direction that [[Gravitate]] is attempting to drag an entity, or Null if the entity is not under the effects of [[Gravitate]].

## [[Impact Purification|Impact Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 15.8,54.7 27.0,35.3 49.5,35.3 72.0,35.3 60.8,54.7 49.5,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:270;--dur:3.4s" points="38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 38.3,54.7 15.8,54.7 27.0,35.3 49.5,35.3 72.0,35.3 60.8,54.7 49.5,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="54.7" x2="34.7" y2="48.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,44.0 36.8,47.3 32.5,49.7" fill="#fc77be"/><circle cx="38.3" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`entity` → `number`

Pushes the scaling factor of [[Gravitate]]'s gravity compared to baseline, or Null if the entity is not under the effects of [[Gravitate]].

I have discovered some theorywork about **Gravity**. It appears every **Gravity**-generating phenomenon has a numerical *priority*. The direction **Gravity** pulls a given entity along is dictated by whichever force has most priority upon that entity.

The **Gravity** created by [[Gravitate]] is notably rather weak; other sources of **Gravity** often overpower it. I can use this to anchor myself against hostile **Hexcasters**.

Gravitational strength is settled through a much more democratic process. Each source of **Gravity** declares some scaling factor. For [[Gravitate]], that is a number between and including 0 and 5.

When pulling, **Gravity** multiplies all the provided scaling factors together to determine the force with which to pull.

On the next few pages are some patterns used to measure "true" gravity.

## [[Falling Purification II|Falling Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 34.8,67.5 14.3,67.5 24.5,49.7 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:266;--dur:3.6s" points="34.8,67.5 24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 34.8,67.5 14.3,67.5 24.5,49.7 45.0,49.7 65.5,49.7 75.7,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="31.5" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,57.7 33.4,60.7 29.6,62.9" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `vector`

Pushes the direction of **Gravity** acting on an entity.

## [[Impact Purification II|Impact Purification II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,61.0 22.2,45.0 13.0,29.0 22.2,13.0 40.7,13.0 50.0,29.0 40.7,45.0 31.5,61.0 13.0,61.0 22.2,45.0 40.7,45.0 59.2,45.0 50.0,61.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:241;--dur:3.6s" points="31.5,61.0 22.2,45.0 13.0,29.0 22.2,13.0 40.7,13.0 50.0,29.0 40.7,45.0 31.5,61.0 13.0,61.0 22.2,45.0 40.7,45.0 59.2,45.0 50.0,61.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="61.0" x2="28.5" y2="55.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.4,52.2 30.3,54.9 26.7,56.9" fill="#fc77be"/><circle cx="31.5" cy="61.0" r="2.6" fill="#fc77be"/></svg>

`entity` → `number`

Pushes strength of **Gravity** acting on an entity compared to baseline.

---
### Patterns on this page

- [[Gravitate]] · Spell · Spell
- [[Grasp Purification]] · Pattern · Purification
- [[Falling Purification]] · Pattern · Purification
- [[Impact Purification]] · Pattern · Purification
- [[Falling Purification II]] · Pattern · Purification
- [[Impact Purification II]] · Pattern · Purification

**Sources:** 🧩 [Hexodus on Modrinth](https://modrinth.com/mod/bN6IKmdE)
