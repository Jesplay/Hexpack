---
aliases: ["Advanced Escaping"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/control-flow]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/advanced_escaping"
---

# Advanced Escaping

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

[[Introspection|Intro-]] and [[Retrospection]] are very useful, but when building complex **Hexes** I may need more advanced techniques.

This section describes such techniques: [[Interjection]] lets me insert [[Iotas|iotas]] directly into an in-progress list, while [[Meditation]] and [[Recollection]] let me start and finish the list creation process with arbitrary levels of nesting depth.

## [[Interjection|Interjection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 65.5,14.3 75.7,32.0 65.5,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 65.5,14.3 75.7,32.0 65.5,49.7 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="41.3" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,32.0 41.3,34.3 41.3,29.8" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Only works while [[Introspection|introspecting]]. Reads an iota from my offhand, and adds it to the in-progress list. For iotas other than patterns, I may need to [[Escaping Patterns|take steps]] to avoid a [[Mishaps|mishap]] when executing the list.

## [[Meditation|Meditation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,16.2 50.8,16.2 39.2,36.1 27.7,16.2 16.2,36.1 27.7,56.1 39.2,36.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="73.8,16.2 50.8,16.2 39.2,36.1 27.7,16.2 16.2,36.1 27.7,56.1 39.2,36.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="16.2" x2="66.5" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="61.2,16.2 66.5,13.6 66.5,18.7" fill="#fc77be"/><circle cx="73.8" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

Removes a number from the [[The Stack|stack]], then acts as though I'd drawn that many [[Introspection|Introspections]]. Treated as a normal pattern while [[Introspection|introspecting]], so nesting this may require [[Consideration|tricks]] to draw extra [[Retrospection|Retrospections]].

## [[Recollection|Recollection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,16.2 39.2,16.2 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1 39.2,56.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="16.2,16.2 39.2,16.2 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1 39.2,56.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="16.2" x2="23.5" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.8,16.2 23.5,18.7 23.5,13.6" fill="#fc77be"/><circle cx="16.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`—` → `num, list`

Works like [[Retrospection]], but closes all open [[Introspection|Introspections]] at once. The number of layers closed, along with the list of saved patterns, are then added to the stack.

---
### Patterns on this page

- [[Interjection]] · Pattern · Escaping
- [[Meditation]] · Pattern · Escaping
- [[Recollection]] · Pattern · Escaping

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/advanced_escaping) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
