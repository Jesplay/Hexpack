---
aliases: ["Wisps"]
tags: [hexbook/entry, mod/hexal, category/spells, topic/wisps]
mod: "Hexal"
category: "Spells"
entry_id: "hexcasting:patterns/spells/wisps"
---

# Wisps

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexal]]

I have discovered a way to create constructs from the [[Media|media]] itself, visible to others and able to move and act somewhat on their own. They are able to cast hexes in my place, similar to how a [[Hermes' Gambit]] evaluates patterns. They do take some of my attention to operate this however, and if too many try and execute at once I will only be able to run some of them, leaving the rest for the next instant.

When they are created I assign them a 'reservoir' of media that they have access to, which is taken from me and used to form the wisp. A wisp uses up part of its reservoir simply existing, and when it evaluates the pattern list it is carrying any costs incurred are removed from its reservoir. All wisps seem to take three [[Amethyst|Amethyst Dust]] to summon, in addition to whatever media is used for their reservoir.

## [[Summon Projectile Wisp|Summon Projectile Wisp]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.1s" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 45.0,26.1 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 53.3,40.5 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="26.1" x2="25.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.8,18.2 27.3,20.6 24.1,22.4" fill="#fc77be"/><circle cx="28.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`[pattern], vec, vec, number` → `—`

This wisp is summoned and launched like an arrow, casting its hex on whatever it hits. The initial [[The Stack|stack]] when it executes will be a reference to itself, then a reference to what it hit.

This could be a block position or an entity reference. The first vector accepted is the location the wisp will be summoned, while the direction of the second is the direction the wisp will be launched, and the second's magnitude seems to affect the wisp's velocity. The cost of the spell increases with the squared magnitude of velocity. The number accepted determines the wisp's reservoir in [[Amethyst|Amethyst Dust]]. It loses approximately 3 tenths of an [[Amethyst|Amethyst Dust]] of media every second once summoned, and has a range of influence of 4 blocks.

## [[Summon Cyclic Wisp|Summon Cyclic Wisp]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="26.1" x2="25.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.8,18.2 27.3,20.6 24.1,22.4" fill="#fc77be"/><circle cx="28.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`[pattern], vec, number` → `—`

Similar to [[Summon Projectile Wisp]], but with only one vector input for the position to summon the wisp. Cyclic wisps are unmoving once summoned, instead executing their hex up to 20 times each second.

The first time it casts the stack will start with a reference to itself, from which point it will always start with the stack and ravenmind in the state they finished on the last cast. This wisp has a less stable form making it lose significantly more media every second, losing approximately seven tenths of an [[Amethyst|Amethyst Dust]] per second once summoned. It has a range of influence of 8 blocks.

## [[Identity Reflection|Identity Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="61.7,26.1 70.0,11.7 78.3,26.1 70.0,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 20.0,40.5 11.7,26.1 20.0,11.7 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 45.0,26.1 28.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:4.5s" points="61.7,26.1 70.0,11.7 78.3,26.1 70.0,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 20.0,40.5 11.7,26.1 20.0,11.7 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 45.0,26.1 28.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="61.7" y1="26.1" x2="64.3" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="66.2,18.2 65.9,22.4 62.7,20.6" fill="#fc77be"/><circle cx="61.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `entity`

Add a reference to the casting wisp to the stack. This must be cast by a wisp; trying to cast it through a [[Staves|staff]] will fail rather spectacularly.

## [[Reservoir Reflection|Reservoir Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 45.0,26.1 61.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:4.5s" points="28.3,26.1 20.0,11.7 11.7,26.1 20.0,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 45.0,26.1 61.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="26.1" x2="25.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.8,18.2 27.3,20.6 24.1,22.4" fill="#fc77be"/><circle cx="28.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Add the amount of media in the wisp's reservoir to the stack. This must be cast by a wisp; trying to cast it through a staff will fail rather spectacularly.

## [[Manager's Purification|Manager's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 20.0,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 20.0,11.7 11.7,26.1 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="11.7,26.1 20.0,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 70.0,40.5 78.3,26.1 70.0,11.7 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 20.0,11.7 11.7,26.1 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="14.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="16.2,34.0 12.7,31.6 15.9,29.8" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`entity` → `[pattern]`

Removes a reference to a wisp from the stack, and replaces it with the wisp's contained hex. Must be a wisp you summoned, or a wisp that has [[Allow Transfer|allowed]] transfer with the caster or caster's owner.

## [[Allegiance Distillation|Allegiance Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="78.3,26.1 70.0,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 20.0,40.5 11.7,26.1 20.0,11.7 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 70.0,11.7 78.3,26.1 61.7,26.1 45.0,26.1 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="78.3,26.1 70.0,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 20.0,40.5 11.7,26.1 20.0,11.7 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 70.0,11.7 78.3,26.1 61.7,26.1 45.0,26.1 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="78.3" y1="26.1" x2="75.7" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="73.8,34.0 74.1,29.8 77.3,31.6" fill="#fc77be"/><circle cx="78.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`entity, entity` → `bool`

Removes a reference to a wisp and an entity from the stack, and returns true if that entity owns that wisp, and false otherwise.

## [[Pathfinder's Gambit|Pathfinder's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 45.0,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 45.0,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="28.2" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.5,14.3 28.2,12.1 28.2,16.6" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`vec` → `—`

Removes a position vector from the stack; The Cyclic Wisp that executed this pattern will move in a straight line towards that position (colliding with terrain in its path) until it reaches it.

## [[Pathfinder's Reflection|Pathfinder's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 24.5,32.0 14.3,14.3 34.8,14.3 45.0,32.0 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="14.3" x2="41.3" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,14.3 41.3,16.6 41.3,12.1" fill="#fc77be"/><circle cx="34.8" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`—` → `vec`

Adds to the stack the position vector that the executing Cyclic Wisp is currently moving towards.

## [[Haste|Haste]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 39.2,16.2 50.8,36.1 39.2,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 39.2,16.2 50.8,36.1 39.2,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="39.2" y1="16.2" x2="31.8" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.5,16.2 31.8,13.6 31.8,18.7" fill="#fc77be"/><circle cx="39.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

Removes a number from the stack, and sets the executing Cyclic Wisp's maximum speed to that number.

## [[Speedometer's Reflection|Speedometer's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="39.2,16.2 62.3,16.2 73.8,36.1 62.3,56.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 39.2,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="39.2,16.2 62.3,16.2 73.8,36.1 62.3,56.1 39.2,56.1 16.2,56.1 27.7,36.1 16.2,16.2 39.2,16.2 50.8,36.1 39.2,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="39.2" y1="16.2" x2="46.6" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="51.9,16.2 46.6,18.7 46.6,13.6" fill="#fc77be"/><circle cx="39.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Gets the executing Cyclic Wisp's current maximum speed.

## [[Allow Transfer|Allow Transfer]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="34" viewBox="0 0 90.0 33.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.1" cy="7.5" r="1.6"/><circle cx="12.9" cy="7.5" r="1.6"/><circle cx="23.6" cy="7.5" r="1.6"/><circle cx="34.3" cy="7.5" r="1.6"/><circle cx="45.0" cy="7.5" r="1.6"/><circle cx="55.7" cy="7.5" r="1.6"/><circle cx="66.4" cy="7.5" r="1.6"/><circle cx="77.1" cy="7.5" r="1.6"/><circle cx="87.9" cy="7.5" r="1.6"/><circle cx="7.5" cy="16.8" r="1.6"/><circle cx="18.2" cy="16.8" r="1.6"/><circle cx="28.9" cy="16.8" r="1.6"/><circle cx="39.6" cy="16.8" r="1.6"/><circle cx="50.4" cy="16.8" r="1.6"/><circle cx="61.1" cy="16.8" r="1.6"/><circle cx="71.8" cy="16.8" r="1.6"/><circle cx="82.5" cy="16.8" r="1.6"/><circle cx="2.1" cy="26.1" r="1.6"/><circle cx="12.9" cy="26.1" r="1.6"/><circle cx="23.6" cy="26.1" r="1.6"/><circle cx="34.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="55.7" cy="26.1" r="1.6"/><circle cx="66.4" cy="26.1" r="1.6"/><circle cx="77.1" cy="26.1" r="1.6"/><circle cx="87.9" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="28.9,16.8 23.6,7.5 12.9,7.5 7.5,16.8 12.9,26.1 23.6,26.1 28.9,16.8 39.6,16.8 50.4,16.8 61.1,16.8 66.4,7.5 77.1,7.5 82.5,16.8 77.1,26.1 66.4,26.1 61.1,16.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:161;--dur:4.2s" points="28.9,16.8 23.6,7.5 12.9,7.5 7.5,16.8 12.9,26.1 23.6,26.1 28.9,16.8 39.6,16.8 50.4,16.8 61.1,16.8 66.4,7.5 77.1,7.5 82.5,16.8 77.1,26.1 66.4,26.1 61.1,16.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.9" y1="16.8" x2="27.2" y2="13.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.0,11.7 28.2,13.2 26.2,14.4" fill="#fc77be"/><circle cx="28.9" cy="16.8" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

When two wisps are [[Links|linked]] together, they transfer media between each other until they are balanced. By default this only happens between wisps with the same owner. This takes an index, and explicitly allows the wisp to exchange media with the linkable at that index.

## [[Disallow Transfer|Disallow Transfer]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="34" viewBox="0 0 90.0 33.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.1" cy="7.5" r="1.6"/><circle cx="12.9" cy="7.5" r="1.6"/><circle cx="23.6" cy="7.5" r="1.6"/><circle cx="34.3" cy="7.5" r="1.6"/><circle cx="45.0" cy="7.5" r="1.6"/><circle cx="55.7" cy="7.5" r="1.6"/><circle cx="66.4" cy="7.5" r="1.6"/><circle cx="77.1" cy="7.5" r="1.6"/><circle cx="87.9" cy="7.5" r="1.6"/><circle cx="7.5" cy="16.8" r="1.6"/><circle cx="18.2" cy="16.8" r="1.6"/><circle cx="28.9" cy="16.8" r="1.6"/><circle cx="39.6" cy="16.8" r="1.6"/><circle cx="50.4" cy="16.8" r="1.6"/><circle cx="61.1" cy="16.8" r="1.6"/><circle cx="71.8" cy="16.8" r="1.6"/><circle cx="82.5" cy="16.8" r="1.6"/><circle cx="2.1" cy="26.1" r="1.6"/><circle cx="12.9" cy="26.1" r="1.6"/><circle cx="23.6" cy="26.1" r="1.6"/><circle cx="34.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="55.7" cy="26.1" r="1.6"/><circle cx="66.4" cy="26.1" r="1.6"/><circle cx="77.1" cy="26.1" r="1.6"/><circle cx="87.9" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="28.9,16.8 23.6,7.5 12.9,7.5 7.5,16.8 12.9,26.1 23.6,26.1 28.9,16.8 39.6,16.8 45.0,7.5 50.4,16.8 45.0,26.1 39.6,16.8 50.4,16.8 61.1,16.8 66.4,26.1 77.1,26.1 82.5,16.8 77.1,7.5 66.4,7.5 61.1,16.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:204;--dur:5.3s" points="28.9,16.8 23.6,7.5 12.9,7.5 7.5,16.8 12.9,26.1 23.6,26.1 28.9,16.8 39.6,16.8 45.0,7.5 50.4,16.8 45.0,26.1 39.6,16.8 50.4,16.8 61.1,16.8 66.4,26.1 77.1,26.1 82.5,16.8 77.1,7.5 66.4,7.5 61.1,16.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.9" y1="16.8" x2="27.2" y2="13.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.0,11.7 28.2,13.2 26.2,14.4" fill="#fc77be"/><circle cx="28.9" cy="16.8" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

Correspondingly, this takes an index and disallows the wisp from exchanging media with the linkable at that index.

## [[Allow Transfer Others|Allow Transfer Others]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="34" viewBox="0 0 90.0 33.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.1" cy="7.5" r="1.6"/><circle cx="12.9" cy="7.5" r="1.6"/><circle cx="23.6" cy="7.5" r="1.6"/><circle cx="34.3" cy="7.5" r="1.6"/><circle cx="45.0" cy="7.5" r="1.6"/><circle cx="55.7" cy="7.5" r="1.6"/><circle cx="66.4" cy="7.5" r="1.6"/><circle cx="77.1" cy="7.5" r="1.6"/><circle cx="87.9" cy="7.5" r="1.6"/><circle cx="7.5" cy="16.8" r="1.6"/><circle cx="18.2" cy="16.8" r="1.6"/><circle cx="28.9" cy="16.8" r="1.6"/><circle cx="39.6" cy="16.8" r="1.6"/><circle cx="50.4" cy="16.8" r="1.6"/><circle cx="61.1" cy="16.8" r="1.6"/><circle cx="71.8" cy="16.8" r="1.6"/><circle cx="82.5" cy="16.8" r="1.6"/><circle cx="2.1" cy="26.1" r="1.6"/><circle cx="12.9" cy="26.1" r="1.6"/><circle cx="23.6" cy="26.1" r="1.6"/><circle cx="34.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="55.7" cy="26.1" r="1.6"/><circle cx="66.4" cy="26.1" r="1.6"/><circle cx="77.1" cy="26.1" r="1.6"/><circle cx="87.9" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="28.9,16.8 23.6,26.1 12.9,26.1 7.5,16.8 12.9,7.5 23.6,7.5 28.9,16.8 39.6,16.8 50.4,16.8 61.1,16.8 66.4,26.1 77.1,26.1 82.5,16.8 77.1,7.5 66.4,7.5 61.1,16.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:161;--dur:4.2s" points="28.9,16.8 23.6,26.1 12.9,26.1 7.5,16.8 12.9,7.5 23.6,7.5 28.9,16.8 39.6,16.8 50.4,16.8 61.1,16.8 66.4,26.1 77.1,26.1 82.5,16.8 77.1,7.5 66.4,7.5 61.1,16.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.9" y1="16.8" x2="27.2" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.0,21.9 26.2,19.2 28.2,20.3" fill="#fc77be"/><circle cx="28.9" cy="16.8" r="2.6" fill="#fc77be"/></svg>

`wisp, num` → `—`

This takes a wisp that you own and an index, and allows the wisp to exchange media with the linkable at that index.

## [[Disallow Transfer Others|Disallow Transfer Others]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="34" viewBox="0 0 90.0 33.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.1" cy="7.5" r="1.6"/><circle cx="12.9" cy="7.5" r="1.6"/><circle cx="23.6" cy="7.5" r="1.6"/><circle cx="34.3" cy="7.5" r="1.6"/><circle cx="45.0" cy="7.5" r="1.6"/><circle cx="55.7" cy="7.5" r="1.6"/><circle cx="66.4" cy="7.5" r="1.6"/><circle cx="77.1" cy="7.5" r="1.6"/><circle cx="87.9" cy="7.5" r="1.6"/><circle cx="7.5" cy="16.8" r="1.6"/><circle cx="18.2" cy="16.8" r="1.6"/><circle cx="28.9" cy="16.8" r="1.6"/><circle cx="39.6" cy="16.8" r="1.6"/><circle cx="50.4" cy="16.8" r="1.6"/><circle cx="61.1" cy="16.8" r="1.6"/><circle cx="71.8" cy="16.8" r="1.6"/><circle cx="82.5" cy="16.8" r="1.6"/><circle cx="2.1" cy="26.1" r="1.6"/><circle cx="12.9" cy="26.1" r="1.6"/><circle cx="23.6" cy="26.1" r="1.6"/><circle cx="34.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="55.7" cy="26.1" r="1.6"/><circle cx="66.4" cy="26.1" r="1.6"/><circle cx="77.1" cy="26.1" r="1.6"/><circle cx="87.9" cy="26.1" r="1.6"/></g><polyline class="hexpat-base" points="28.9,16.8 23.6,26.1 12.9,26.1 7.5,16.8 12.9,7.5 23.6,7.5 28.9,16.8 39.6,16.8 45.0,26.1 50.4,16.8 45.0,7.5 39.6,16.8 50.4,16.8 61.1,16.8 66.4,7.5 77.1,7.5 82.5,16.8 77.1,26.1 66.4,26.1 61.1,16.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:204;--dur:5.3s" points="28.9,16.8 23.6,26.1 12.9,26.1 7.5,16.8 12.9,7.5 23.6,7.5 28.9,16.8 39.6,16.8 45.0,26.1 50.4,16.8 45.0,7.5 39.6,16.8 50.4,16.8 61.1,16.8 66.4,7.5 77.1,7.5 82.5,16.8 77.1,26.1 66.4,26.1 61.1,16.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.9" y1="16.8" x2="27.2" y2="19.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.0,21.9 26.2,19.2 28.2,20.3" fill="#fc77be"/><circle cx="28.9" cy="16.8" r="2.6" fill="#fc77be"/></svg>

`wisp, num` → `—`

This takes a wisp that you own and an index, and disallows the wisp from exchanging media with the linkable at that index.

## [[Entity Purification Wisp|Entity Purification: Wisp]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="45.0,19.0 51.1,29.6 63.2,29.6 69.3,19.0 75.4,8.5 81.5,19.0 75.4,29.6 69.3,19.0 63.2,8.5 51.1,8.5 45.0,19.0 38.9,8.5 32.8,19.0 38.9,29.6 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:207;--dur:4.8s" points="45.0,19.0 51.1,29.6 63.2,29.6 69.3,19.0 75.4,8.5 81.5,19.0 75.4,29.6 69.3,19.0 63.2,8.5 51.1,8.5 45.0,19.0 38.9,8.5 32.8,19.0 38.9,29.6 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="19.0" x2="46.9" y2="22.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.3,24.8 45.8,23.1 48.1,21.7" fill="#fc77be"/><circle cx="45.0" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`vec` → `entity | null`

Transform the position on the stack into the wisp at that location (or Null if there isn't one).

## [[Zone Distillation Wisp|Zone Distillation: Wisp]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="45.0,19.0 51.1,29.6 63.2,29.6 69.3,19.0 75.4,8.5 81.5,19.0 75.4,29.6 69.3,19.0 63.2,8.5 51.1,8.5 45.0,19.0 38.9,29.6 32.8,19.0 38.9,8.5 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:207;--dur:4.8s" points="45.0,19.0 51.1,29.6 63.2,29.6 69.3,19.0 75.4,8.5 81.5,19.0 75.4,29.6 69.3,19.0 63.2,8.5 51.1,8.5 45.0,19.0 38.9,29.6 32.8,19.0 38.9,8.5 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="19.0" x2="46.9" y2="22.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.3,24.8 45.8,23.1 48.1,21.7" fill="#fc77be"/><circle cx="45.0" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `[entity]`

Take a position and maximum distance on the stack, and combine them into a list of wisps near the position.

## [[Zone Distillation Non-Wisp|Zone Distillation: Non-Wisp]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="45.0,19.0 51.1,8.5 63.2,8.5 69.3,19.0 75.4,29.6 81.5,19.0 75.4,8.5 69.3,19.0 63.2,29.6 51.1,29.6 45.0,19.0 38.9,8.5 32.8,19.0 38.9,29.6 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:207;--dur:4.8s" points="45.0,19.0 51.1,8.5 63.2,8.5 69.3,19.0 75.4,29.6 81.5,19.0 75.4,8.5 69.3,19.0 63.2,29.6 51.1,29.6 45.0,19.0 38.9,8.5 32.8,19.0 38.9,29.6 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="19.0" x2="46.9" y2="15.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.3,13.3 48.1,16.3 45.8,15.0" fill="#fc77be"/><circle cx="45.0" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`vec, num` → `[entity]`

Take a position and maximum distance on the stack, and combine them into a list of non-wisp entities near the position.

## [[Delay Wisp|Delay Wisp]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="45.0,41.5 31.8,18.5 18.5,41.5 31.8,64.4 45.0,41.5 58.2,18.5 71.5,41.5 58.2,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="40.8" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.7,28.8 43.3,32.7 38.2,35.6" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`number` → `—`

Removes an integer from the stack, then delays the wisp's next cast until at least that many twentieth's of a second have passed. This and other triggers like it will halve the wisp's upkeep cost while it isn't casting.

## [[Listen|Listen]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="28.3,26.1 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 53.3,11.7 61.7,26.1 53.3,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:217;--dur:3.6s" points="28.3,26.1 45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 53.3,11.7 61.7,26.1 53.3,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="28.3" y1="26.1" x2="33.7" y2="26.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.5,26.1 33.7,27.9 33.7,24.3" fill="#fc77be"/><circle cx="28.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

The wisp's next cast will not occur until it has received at least one [[Send Iota|communication]].

## [[Wander|Wander]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 70.0,40.5 61.7,26.1 53.3,11.7 36.7,11.7 20.0,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:3.4s" points="11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 70.0,40.5 61.7,26.1 53.3,11.7 36.7,11.7 20.0,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="17.0" y2="26.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.1 17.0,27.9 17.0,24.3" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

The wisp's next cast will not occur until it has reached the position it is [[Pathfinder's Gambit|navigating]] towards.

---
### Patterns on this page

- [[Summon Projectile Wisp]] · Spell · Spell
- [[Summon Cyclic Wisp]] · Spell · Spell
- [[Identity Reflection]] · Pattern · Reflection
- [[Reservoir Reflection]] · Pattern · Reflection
- [[Manager's Purification]] · Pattern · Purification
- [[Allegiance Distillation]] · Pattern · Distillation
- [[Pathfinder's Gambit]] · Pattern · Gambit
- [[Pathfinder's Reflection]] · Pattern · Reflection
- [[Haste]] · Spell · Spell
- [[Speedometer's Reflection]] · Pattern · Reflection
- [[Allow Transfer]] · Spell · Spell
- [[Disallow Transfer]] · Spell · Spell
- [[Allow Transfer Others]] · Spell · Spell
- [[Disallow Transfer Others]] · Spell · Spell
- [[Entity Purification Wisp]] · Pattern · Purification
- [[Zone Distillation Wisp]] · Pattern · Distillation
- [[Zone Distillation Non-Wisp]] · Pattern · Distillation
- [[Delay Wisp]] · Pattern · Other
- [[Listen]] · Pattern · Other
- [[Wander]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/wisps) · 🧩 [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
