---
aliases: ["Pigment Manipulation"]
tags: [hexbook/entry, mod/hexical, category/spells, topic/visuals]
mod: "Hexical"
category: "Spells"
entry_id: "hexcasting:patterns/spells/pigments"
---

# Pigment Manipulation

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexical]]

Like how I can manipulate [[Dyes|dyes]] with **Hexcasting**, I can also manipulate [[Pigments|pigments]]. I can even store multiple of them and have my **Hexes** randomly choose one to me.

## [[Pigment Purification|Pigment Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="53.3,26.1 45.0,11.7 36.7,26.1 45.0,40.5 53.3,55.0 45.0,69.4 36.7,55.0 45.0,40.5 53.3,26.1 70.0,26.1 78.3,40.5 70.0,55.0 53.3,55.0 36.7,55.0 20.0,55.0 11.7,40.5 20.0,26.1 36.7,26.1 53.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="53.3,26.1 45.0,11.7 36.7,26.1 45.0,40.5 53.3,55.0 45.0,69.4 36.7,55.0 45.0,40.5 53.3,26.1 70.0,26.1 78.3,40.5 70.0,55.0 53.3,55.0 36.7,55.0 20.0,55.0 11.7,40.5 20.0,26.1 36.7,26.1 53.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="53.3" y1="26.1" x2="50.7" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.8,18.2 52.3,20.6 49.1,22.4" fill="#fc77be"/><circle cx="53.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`dye/entity` → `pigment`

Takes a colored dye [[Iotas|iota]] and transforms it into a pigment, takes a player and returns their pigment, or takes an item containing a pigment and returns its pigment.

## [[Pigment Exaltation|Pigment Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,45.0 50.0,61.0 40.7,77.0 31.5,61.0 40.7,45.0 50.0,29.0 40.7,13.0 31.5,29.0 40.7,45.0 59.2,45.0 68.5,29.0 59.2,13.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:296;--dur:4.5s" points="40.7,45.0 50.0,61.0 40.7,77.0 31.5,61.0 40.7,45.0 50.0,29.0 40.7,13.0 31.5,29.0 40.7,45.0 59.2,45.0 68.5,29.0 59.2,13.0 40.7,13.0 22.2,13.0 13.0,29.0 22.2,45.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="45.0" x2="43.7" y2="50.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="45.8,53.8 41.9,51.1 45.4,49.1" fill="#fc77be"/><circle cx="40.7" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`pigment, vec, num` → `vec`

Samples the pigment at a certain place and time as a vector, where each component is from 0 to 1 and represents the red, green, and blueness respectively.

## [[Internalize Pigment II|Internalize Pigment II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="22.2,45.0 40.7,45.0 59.2,45.0 68.5,61.0 59.2,77.0 40.7,77.0 22.2,77.0 13.0,61.0 22.2,45.0 13.0,29.0 22.2,13.0 40.7,13.0 50.0,29.0 40.7,45.0 31.5,61.0 40.7,77.0 50.0,61.0 40.7,45.0 31.5,29.0 40.7,13.0 59.2,13.0 68.5,29.0 59.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:407;--dur:6.0s" points="22.2,45.0 40.7,45.0 59.2,45.0 68.5,61.0 59.2,77.0 40.7,77.0 22.2,77.0 13.0,61.0 22.2,45.0 13.0,29.0 22.2,13.0 40.7,13.0 50.0,29.0 40.7,45.0 31.5,61.0 40.7,77.0 50.0,61.0 40.7,45.0 31.5,29.0 40.7,13.0 59.2,13.0 68.5,29.0 59.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="22.2" y1="45.0" x2="28.1" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.4,45.0 28.1,47.0 28.1,43.0" fill="#fc77be"/><circle cx="22.2" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`pigment` → `—`

Internalizes a pigment iota.

---
### Patterns on this page

- [[Pigment Purification]] · Pattern · Purification
- [[Pigment Exaltation]] · Pattern · Exaltation
- [[Internalize Pigment II]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/pigments)
