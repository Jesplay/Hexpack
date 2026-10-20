---
aliases: ["Gates"]
tags: [hexbook/entry, mod/hexal, category/great-spells, topic/movement]
mod: "Hexal"
category: "Great Spells"
entry_id: "hexcasting:patterns/great_spells/gates"
---

# Gates

> **Category:** Patterns › [[Great Spells (Category)|Great Spells]]  
> **Source mod:** [[Hexal]]

Gates are the latest in my long line of experimentation with breaking space to connect distant points and allow instantaneous travel. What gates allow me to do, that none of my previous attempts have succeeded at, is to do it *efficiently*. There are three varieties of gates that I have learned how to construct, all of which take a large amount of [[Media|media]] to burn into reality, but are significantly cheaper to operate afterwards.

Location Anchored Gates are bound to a specific position in the world, and send all that move through them to that position. Entity Anchored Gates are bound to a being that inhabits the world, and send those moving through them to the position of that being (plus an offset, chosen when burning in the gate).

Drifting Gates are not bound at all, and can be used to send entities anywhere, however they lose efficiency as a result, and can only send entities to places that are in my ambit. When a gate is burnt in I will obtain on my [[The Stack|stack]] a reference to that gate. With that gate reference, I can mark entities with that gate, and I can close the gate, which sends all entities marked with that gate to the gate's destination. Note that closing a gate doesn't use it up; once I have acquired a gate [[Iotas|iota]] I can use it in perpetuity.

## [[Gate's Reflection|Gate's Reflection]] ⭐ *Great Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 20.0,55.0 36.7,55.0 45.0,69.4 53.3,55.0 70.0,55.0 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:400;--dur:6.0s" points="45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 20.0,55.0 36.7,55.0 45.0,69.4 53.3,55.0 70.0,55.0 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="11.7" x2="39.7" y2="11.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,11.7 39.7,9.8 39.7,13.5" fill="#fc77be"/><circle cx="45.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`null | vec | vec, entity` → `gate`

Adds a new gate iota to the stack. Any copy of this iota will point to the same numbered gate. Costs 32 [[Amethyst|Charged Amethyst]]. If passed null, makes a Drifting Gate. If passed a vector, makes a Location Anchored Gate, If passed a vector and an entity, makes an Entity Anchored Gate.

## [[Gate's Opening|Gate's Opening]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="38.5" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.8,14.3 38.5,12.1 38.5,16.6" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`gate, entity` → `—`

Consumes a gate iota and an entity. Marks that entity to be teleported the next time that gate is closed. Costs one twentieth of an [[Amethyst|Amethyst Dust]].

## [[Gate's Dismissal|Gate's Dismissal]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="51.5" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="56.2,14.3 51.5,16.6 51.5,12.1" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`gate, entity` → `—`

Consumes a gate iota and an entity. Unmarks that entity so that if it was marked with [[Gate's Opening]] it will no longer be teleported the next time that gate is closed.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`gate` → `num`

Consumes a gate iota, returns how many entities are marked by that gate.

## [[Marked Distillation|Marked Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 34.8,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 45.0,49.7 34.8,32.0 45.0,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="14.3" x2="51.5" y2="14.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="56.2,14.3 51.5,16.6 51.5,12.1" fill="#fc77be"/><circle cx="45.0" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`gate, entity` → `bool`

Consumes a gate iota and an entity, returns whether that entity is marked by that gate.

## [[Gate's Closing|Gate's Closing]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 53.3,11.7 61.7,26.1 53.3,40.5 45.0,26.1 36.7,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 53.3,40.5 70.0,40.5 78.3,26.1 70.0,11.7 53.3,11.7 36.7,11.7 28.3,26.1 36.7,40.5 45.0,26.1 53.3,11.7 61.7,26.1 53.3,40.5 45.0,26.1 36.7,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="11.7" x2="31.3" y2="11.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="27.5,11.7 31.3,9.8 31.3,13.5" fill="#fc77be"/><circle cx="36.7" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`gate | gate, vec` → `—`

Consumes a non-drifing gate iota, or a drifting gate iota and a position. Transports all the entities marked with the passed gate to the stored or given position, as relevant. Costs half an [[Amethyst|Amethyst Shard]], plus an [[Amethyst|Amethyst Dust]] per metre transported per entity, if the gate was drifting.

---
### Patterns on this page

- [[Gate's Reflection]] · Great Spell · Reflection
- [[Gate's Opening]] · Spell · Spell
- [[Gate's Dismissal]] · Spell · Spell
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Marked Distillation]] · Pattern · Distillation
- [[Gate's Closing]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/great_spells/gates) · 🧩 [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
