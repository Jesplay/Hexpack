---
aliases: ["Pehkui"]
tags: [hexbook/entry, mod/hex-casting, category/cross-mod-compatibility, topic/effects]
mod: "Hex Casting"
category: "Cross-Mod Compatibility"
entry_id: "hexcasting:interop/pehkui"
---

# Pehkui

> **Category:** [[Cross-Mod Compatibility (Category)|Cross-Mod Compatibility]]  
> **Source mod:** [[Hex Casting]]

I have discovered methods of changing the size of entities, and querying how much larger or smaller they are than normal.

## [[Gulliver's Purification|Gulliver's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="55.2,67.5 45.0,49.7 34.8,67.5 55.2,67.5 75.7,67.5 65.5,49.7 55.2,32.0 45.0,14.3 34.8,32.0 24.5,49.7 14.3,67.5 34.8,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="55.2,67.5 45.0,49.7 34.8,67.5 55.2,67.5 75.7,67.5 65.5,49.7 55.2,32.0 45.0,14.3 34.8,32.0 24.5,49.7 14.3,67.5 34.8,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="67.5" x2="52.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,57.7 53.9,60.7 50.0,62.9" fill="#fc77be"/><circle cx="55.2" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `num`

Get the scale of the entity, as a proportion of their normal size. For most entities, this will be 1.

## [[Alter Scale|Alter Scale]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 45.0,49.7 55.2,67.5 34.8,67.5 14.3,67.5 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,67.5 55.2,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,67.5 45.0,49.7 55.2,67.5 34.8,67.5 14.3,67.5 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,67.5 55.2,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="38.0" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,57.7 40.0,62.9 36.1,60.7" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`entity, num` → `—`

Set the scale of the entity, passing in a proportion of their normal size. Costs about 1 **Amethyst Shard**.

---
### Patterns on this page

- [[Gulliver's Purification]] · Pattern · Purification
- [[Alter Scale]] · Spell · Spell

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#interop/pehkui) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
