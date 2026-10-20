---
aliases: ["Pocket Simulator Patterns"]
tags: [hexbook/entry, mod/slate-works, category/patterns, topic/circles]
mod: "Slate Works"
category: "Patterns"
entry_id: "hexcasting:patterns/hotbar_loci_patterns"
---

# Pocket Simulator Patterns

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Slate Works]]

As nice as the [[Pocket Simulator]] is, it amplifies its power through these patterns and spells.

These [[Enlightened Mishaps|mishap when not cast in a Spell Circle]]. However, these preform nothing if the current [[Spell Circles|Spell Circle]] is not bound to a [[Pocket Simulator]].

## [[Set Slot|Set Slot]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 34.8,32.0 14.3,32.0 24.5,49.7 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 45.0,14.3 24.5,14.3 34.8,32.0 45.0,49.7 55.2,32.0 65.5,14.3 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="45.0,14.3 34.8,32.0 14.3,32.0 24.5,49.7 45.0,49.7 65.5,49.7 75.7,32.0 55.2,32.0 45.0,14.3 24.5,14.3 34.8,32.0 45.0,49.7 55.2,32.0 65.5,14.3 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="41.7" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,24.1 39.8,18.9 43.7,21.1" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`int` → `—`

This takes a number between 0 and 5 (inclusive), and sets the held slot of the bound [[Pocket Simulator]]. Free.

## [[List Items|List Items]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,32.0 34.8,49.7 14.3,49.7 24.5,67.5 45.0,67.5 65.5,67.5 75.7,49.7 55.2,49.7 45.0,32.0 24.5,32.0 34.8,49.7 45.0,67.5 55.2,49.7 65.5,32.0 45.0,32.0 34.8,14.3 55.2,14.3 45.0,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:348;--dur:4.8s" points="45.0,32.0 34.8,49.7 14.3,49.7 24.5,67.5 45.0,67.5 65.5,67.5 75.7,49.7 55.2,49.7 45.0,32.0 24.5,32.0 34.8,49.7 45.0,67.5 55.2,49.7 65.5,32.0 45.0,32.0 34.8,14.3 55.2,14.3 45.0,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="32.0" x2="41.7" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,41.8 39.8,36.6 43.7,38.8" fill="#fc77be"/><circle cx="45.0" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `[item stack]`

This returns all the items held within the [[Pocket Simulator]] as [[Items|Item Stack Iotas]]. Free.

---
### Patterns on this page

- [[Set Slot]] · Pattern · Other
- [[List Items]] · Pattern · Other

**Sources:** 🧩 [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
