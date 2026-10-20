---
aliases: []
tags: [hexbook/entry, mod/hexical, category/great-spells, topic/devices]
mod: "Hexical"
category: "Great Spells"
entry_id: "hexcasting:patterns/great_spells/charm"
---

# Charm

> **Category:** Patterns › [[Great Spells (Category)|Great Spells]]  
> **Source mod:** [[Hexical]]

The **Charm** spell transforms any item into a self-contained magical item, not unlike a [[Casting Items|Cypher]]. They have a battery of **[[Media|media]]**, disenchant when empty, and can't be recharged or reprogrammed.

*Perfection is achieved, not when there is nothing more to add, but when there is nothing left to take away.*

Charmed items also have a few special features that distinguish them from any other [[Casting Items|casting device]].

They have two patterns for interfacing with an internal [[Iotas|iota]] storage within them. This storage can only be read from and written to within the charmed item.

They also have the power of input interception.

## [[Charm|Charm]] ⭐ *Great Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="74" height="90" viewBox="0 0 73.7 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="50.5" cy="9.6" r="1.6"/><circle cx="64.1" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="57.3" cy="21.4" r="1.6"/><circle cx="71.0" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="50.5" cy="33.2" r="1.6"/><circle cx="64.1" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="57.3" cy="45.0" r="1.6"/><circle cx="71.0" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="50.5" cy="56.8" r="1.6"/><circle cx="64.1" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="57.3" cy="68.6" r="1.6"/><circle cx="71.0" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/><circle cx="50.5" cy="80.4" r="1.6"/><circle cx="64.1" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="36.8,33.2 43.7,45.0 36.8,56.8 30.0,45.0 36.8,33.2 50.5,33.2 57.3,45.0 50.5,56.8 36.8,56.8 30.0,68.6 43.7,68.6 36.8,56.8 23.2,56.8 16.4,45.0 23.2,33.2 36.8,33.2 30.0,21.4 43.7,21.4 57.3,21.4 64.1,33.2 57.3,45.0 64.1,56.8 57.3,68.6 43.7,68.6 36.8,80.4 30.0,68.6 16.4,68.6 9.6,56.8 16.4,45.0 9.6,33.2 16.4,21.4 30.0,21.4 36.8,9.6 43.7,21.4 36.8,33.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:464;--dur:6.0s" points="36.8,33.2 43.7,45.0 36.8,56.8 30.0,45.0 36.8,33.2 50.5,33.2 57.3,45.0 50.5,56.8 36.8,56.8 30.0,68.6 43.7,68.6 36.8,56.8 23.2,56.8 16.4,45.0 23.2,33.2 36.8,33.2 30.0,21.4 43.7,21.4 57.3,21.4 64.1,33.2 57.3,45.0 64.1,56.8 57.3,68.6 43.7,68.6 36.8,80.4 30.0,68.6 16.4,68.6 9.6,56.8 16.4,45.0 9.6,33.2 16.4,21.4 30.0,21.4 36.8,9.6 43.7,21.4 36.8,33.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.8" y1="33.2" x2="39.0" y2="37.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.6,39.7 37.7,37.7 40.3,36.2" fill="#fc77be"/><circle cx="36.8" cy="33.2" r="2.6" fill="#fc77be"/></svg>

`entity, list, num, list of num, list of num` → `—`

Charms an item. Takes the item to charm, the **Hex**, and an amount of **media** in [[Amethyst|Amethyst Dust]]. Consumes that amount and about three [[Amethyst|Charged Amethyst]].

The first and second list are used when I am standing and sneaking respectively. When I hold a charmed item, my inputs have an assigned number up to seven; 0 and 1 are the basics for attacking and using respectively.

The charm replaces the original function with the **Hex** if I provide an input that exists in the list corresponding to my current pose, starting the [[The Stack|stack]] with the number.

## [[Charmed Gambit|Charmed Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,61.0 40.7,45.0 50.0,29.0 31.5,29.0 22.2,45.0 31.5,61.0 50.0,61.0 59.2,45.0 50.0,29.0 59.2,13.0 68.5,29.0 59.2,45.0 68.5,61.0 50.0,61.0 40.7,77.0 31.5,61.0 13.0,61.0 22.2,45.0 13.0,29.0 31.5,29.0 40.7,13.0 59.2,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:389;--dur:5.9s" points="31.5,61.0 40.7,45.0 50.0,29.0 31.5,29.0 22.2,45.0 31.5,61.0 50.0,61.0 59.2,45.0 50.0,29.0 59.2,13.0 68.5,29.0 59.2,45.0 68.5,61.0 50.0,61.0 40.7,77.0 31.5,61.0 13.0,61.0 22.2,45.0 13.0,29.0 31.5,29.0 40.7,13.0 59.2,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="61.0" x2="34.4" y2="55.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,52.2 36.2,56.9 32.7,54.9" fill="#fc77be"/><circle cx="31.5" cy="61.0" r="2.6" fill="#fc77be"/></svg>

`any` → `—`

Writes an iota to the charmed item’s internal storage. Subject to the Transgress Others [[Mishaps|mishap]].

## [[Charmed Reflection|Charmed Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,61.0 40.7,45.0 50.0,29.0 31.5,29.0 22.2,45.0 31.5,61.0 50.0,61.0 59.2,45.0 50.0,29.0 59.2,13.0 40.7,13.0 31.5,29.0 13.0,29.0 22.2,45.0 13.0,61.0 31.5,61.0 40.7,77.0 50.0,61.0 68.5,61.0 59.2,45.0 68.5,29.0 59.2,13.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:389;--dur:5.9s" points="31.5,61.0 40.7,45.0 50.0,29.0 31.5,29.0 22.2,45.0 31.5,61.0 50.0,61.0 59.2,45.0 50.0,29.0 59.2,13.0 40.7,13.0 31.5,29.0 13.0,29.0 22.2,45.0 13.0,61.0 31.5,61.0 40.7,77.0 50.0,61.0 68.5,61.0 59.2,45.0 68.5,29.0 59.2,13.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="61.0" x2="34.4" y2="55.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,52.2 36.2,56.9 32.7,54.9" fill="#fc77be"/><circle cx="31.5" cy="61.0" r="2.6" fill="#fc77be"/></svg>

`—` → `any`

Pushes the stored iota from the charmed item’s internal storage.

## [[Charmed Gambit II|Charmed Gambit II]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,45.0 40.7,61.0 31.5,77.0 22.2,61.0 31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 50.0,45.0 40.7,61.0 22.2,61.0 13.0,45.0 22.2,29.0 40.7,29.0 50.0,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="31.5,45.0 40.7,61.0 31.5,77.0 22.2,61.0 31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 50.0,45.0 40.7,61.0 22.2,61.0 13.0,45.0 22.2,29.0 40.7,29.0 50.0,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="45.0" x2="34.4" y2="50.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,53.8 32.7,51.1 36.2,49.1" fill="#fc77be"/><circle cx="31.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`any` → `—`

If the item charged features an iota storage by default, such as a [[Focus]], writes an iota into it.

## [[Charmed Reflection II|Charmed Reflection II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,61.0 31.5,77.0 22.2,61.0 31.5,45.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 22.2,61.0 40.7,61.0 50.0,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,61.0 31.5,77.0 22.2,61.0 31.5,45.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 22.2,61.0 40.7,61.0 50.0,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="45.0" x2="34.4" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,36.2 36.2,40.9 32.7,38.9" fill="#fc77be"/><circle cx="31.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`—` → `any`

If the item charged features an iota storage by default, such as a [[Focus]], reads an iota from it.

## [[Discharm|Discharm]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,61.0 22.2,61.0 31.5,45.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 31.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 40.7,61.0 22.2,61.0 31.5,45.0 13.0,45.0 22.2,61.0 13.0,77.0 31.5,77.0 50.0,77.0 40.7,61.0 50.0,45.0 31.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="45.0" x2="34.4" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,36.2 36.2,40.9 32.7,38.9" fill="#fc77be"/><circle cx="31.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`entity` → `—`

Removes a charm from an item without affecting any other property. Costs about one [[Amethyst|Amethyst Dust]].

---
### Patterns on this page

- [[Charm]] · Great Spell · Great Spell
- [[Charmed Gambit]] · Pattern · Gambit
- [[Charmed Reflection]] · Pattern · Reflection
- [[Charmed Gambit II]] · Spell · Gambit
- [[Charmed Reflection II]] · Pattern · Reflection
- [[Discharm]] · Spell · Spell
