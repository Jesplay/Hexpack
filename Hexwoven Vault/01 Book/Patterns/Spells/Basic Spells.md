---
aliases: ["Basic Spells"]
tags: [hexbook/entry, mod/hex-casting, category/spells, topic/combat, topic/movement, topic/visuals]
mod: "Hex Casting"
category: "Spells"
entry_id: "hexcasting:patterns/spells/basic"
---

# Basic Spells

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hex Casting]]

## [[Explosion|Explosion]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 58.2,64.4 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 58.2,64.4 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `—`

Remove a number and vector from the [[The Stack|stack]], then create an explosion at the given location with the given power.

A power of 3 is about as much as a Creeper's blast; 4 is about as much as a TNT blast. Nature refuses to give me a blast of more than 10 power, though.

Strangely, this explosion doesn't seem to harm me. Perhaps it's because I am the one exploding?

Costs a negligible amount at power 0, plus 3 extra [[Amethyst|Amethyst Dust]] per point of explosion power.

## [[Fireball|Fireball]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,64.4 45.0,41.5 31.8,18.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="45.0,41.5 71.5,41.5 58.2,64.4 45.0,41.5 31.8,18.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`vector, number` → `—`

Remove a number and vector from the stack, then create a fiery explosion at the given location with the given power.

Costs one [[Amethyst|Amethyst Dust]], plus about 3 extra [[Amethyst|Amethyst Dust]]s per point of explosion power. Otherwise, the same as [[Explosion]], except with fire.

## [[Impulse|Impulse]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.7" y1="36.1" x2="24.0" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,47.1 21.8,41.3 26.2,43.8" fill="#fc77be"/><circle cx="27.7" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`entity, vector` → `—`

Remove an entity and direction from the stack, then give a shove to the given entity in the given direction. The strength of the impulse is determined by the length of the vector.
Costs units of [[Amethyst|Amethyst Dust]] equal to the square of the length of the vector, plus one for every Impulse except the first targeting an entity.

## [[Blink|Blink]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:208;--dur:2.5s" points="27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.7" y1="36.1" x2="24.0" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,47.1 21.8,41.3 26.2,43.8" fill="#fc77be"/><circle cx="27.7" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`entity, number` → `—`

Remove an entity and length from the stack, then teleport the given entity along its look vector by the given length.
Costs about one [[Amethyst|Amethyst Shard]] per two blocks travelled.

## [[Make Note|Make Note]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="77.6" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="63.2,20.1 34.5,20.1 48.8,45.0 20.1,45.0 34.5,69.9 48.8,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="63.2,20.1 34.5,20.1 48.8,45.0 20.1,45.0 34.5,69.9 48.8,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="63.2" y1="20.1" x2="54.0" y2="20.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="47.4,20.1 54.0,17.0 54.0,23.3" fill="#fc77be"/><circle cx="63.2" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`vector, number, number` → `—`

Remove a vector and two numbers from the stack. Plays an **instrument** defined by the first number at the given location, with a **note** defined by the second number. Costs a negligible amount of **[[Media|media]]**.

There appear to be 16 different **instruments** and 25 different **notes**. Both are indexed by zero.

These seem to be the same instruments I can produce with a **Note Block**, though the reason for each instrument's number being what it is eludes me.

Either way, I can find the numbers I need to use by inspecting a **Note Block** through a [[Scrying Lens]].

---
### Patterns on this page

- [[Explosion]] · Spell · Spell
- [[Fireball]] · Spell · Spell
- [[Impulse]] · Spell · Spell
- [[Blink]] · Spell · Spell
- [[Make Note]] · Spell · Spell

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/basic) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
