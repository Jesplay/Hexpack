---
aliases: []
tags: [hexbook/entry, mod/hexpose, category/scrying, topic/sensing]
mod: "Hexpose"
category: "Scrying"
entry_id: "hexcasting:patterns/scrying/media"
---

# Media

> **Category:** Patterns › [[Scrying (Category)|Scrying]]  
> **Source mod:** [[Hexpose]]

## [[Media Reflection|Media Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,45.0 20.1,45.0 34.5,20.1 48.8,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:115;--dur:1.6s" points="48.8,45.0 20.1,45.0 34.5,20.1 48.8,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="45.0" x2="39.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.0,45.0 39.7,41.8 39.7,48.2" fill="#fc77be"/><circle cx="48.8" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`—` → `number`

Pushes the **[[Media|media]]** available to the current cast.

## [[Media Purification|Media Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:113;--dur:1.6s" points="38.3,35.3 15.8,35.3 27.0,15.8 38.3,35.3 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="35.3" x2="31.1" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.9,35.3 31.1,32.8 31.1,37.7" fill="#fc77be"/><circle cx="38.3" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`entity | item | vector` → `number`

Pushes the amount of **media** inside an item, that a creature contains or holds, or that a specialized **media**-holding block contains.

## [[Potential Purification|Potential Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,64.4 45.0,41.5 58.2,18.5 31.8,18.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,41.5 71.5,41.5 58.2,64.4 45.0,41.5 58.2,18.5 31.8,18.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`item` → `number/null`

Takes in an item and pushes either Null if the item does not have any **media**-holding potential, or the amount of **media** it is capable of holding at its peak.

---
### Patterns on this page

- [[Media Reflection]] · Pattern · Reflection
- [[Media Purification]] · Pattern · Purification
- [[Potential Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/media) · 🧩 [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
