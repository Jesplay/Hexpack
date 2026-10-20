---
aliases: ["Nadirs"]
tags: [hexbook/entry, mod/hex-casting, category/spells, topic/combat, topic/effects]
mod: "Hex Casting"
category: "Spells"
entry_id: "hexcasting:patterns/spells/nadirs"
---

# Nadirs

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hex Casting]]

This family of spells all impart a negative potion effect upon an entity. They all take an entity, the recipient, and one or two numbers, the first being the duration and the second, if present, being the potency (starting at 1).

Each one has a "base cost;" the actual cost is equal to that base cost, multiplied by the potency squared.

According to certain legends, these spells and their sisters, the [[Zeniths]], were "\[...\] inspired by a world near to this one, where powerful wizards would gather magic from the land and hold duels to the death. Unfortunately, much was lost in translation..."

Perhaps that is the reason for their peculiar names.

## [[White Sun's Nadir|White Sun's Nadir]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:293;--dur:3.6s" points="60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="60.8" y1="35.3" x2="57.2" y2="29.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.6,24.5 59.3,27.8 55.0,30.3" fill="#fc77be"/><circle cx="60.8" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity, number, number` → `—`

Inflicts **weakness**. Base cost is one [[Amethyst|Amethyst Dust]] per 10 seconds.

## [[Blue Sun's Nadir|Blue Sun's Nadir]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3 60.8,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:3.9s" points="49.5,15.8 27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3 60.8,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="15.8" x2="42.3" y2="15.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,15.8 42.3,13.3 42.3,18.2" fill="#fc77be"/><circle cx="49.5" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`entity, number` → `—`

Inflicts **levitation**. Base cost is one [[Amethyst|Amethyst Dust]] per 5 seconds.

## [[Black Sun's Nadir|Black Sun's Nadir]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3 49.5,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:3.9s" points="27.0,15.8 15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3 49.5,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="23.4" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.5 21.3,20.8 25.6,23.2" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`entity, number, number` → `—`

Inflicts **withering**. Base cost is one [[Amethyst|Amethyst Dust]] per second.

## [[Red Sun's Nadir|Red Sun's Nadir]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:3.9s" points="15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3 27.0,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="35.3" x2="19.4" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,46.0 17.2,42.7 21.5,40.3" fill="#fc77be"/><circle cx="15.8" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity, number, number` → `—`

Inflicts **poison**. Base cost is one [[Amethyst|Amethyst Dust]] per 3 seconds.

## [[Green Sun's Nadir|Green Sun's Nadir]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 76.5 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:293;--dur:3.6s" points="15.8,35.3 27.0,54.7 49.5,54.7 60.8,35.3 49.5,15.8 27.0,15.8 15.8,35.3 38.3,35.3 27.0,54.7 15.8,74.2 38.3,74.2 60.8,74.2 49.5,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="35.3" x2="19.4" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,46.0 17.2,42.7 21.5,40.3" fill="#fc77be"/><circle cx="15.8" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity, number, number` → `—`

Inflicts **slowness**. Base cost is one [[Amethyst|Amethyst Dust]] per 5 seconds.

---
### Patterns on this page

- [[White Sun's Nadir]] · Spell · Spell
- [[Blue Sun's Nadir]] · Spell · Spell
- [[Black Sun's Nadir]] · Spell · Spell
- [[Red Sun's Nadir]] · Spell · Spell
- [[Green Sun's Nadir]] · Spell · Spell

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/spells/nadirs) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
