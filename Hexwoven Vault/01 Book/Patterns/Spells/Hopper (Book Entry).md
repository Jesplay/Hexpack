---
aliases: []
tags: [hexbook/entry, mod/hexical, category/spells, topic/items]
mod: "Hexical"
category: "Spells"
entry_id: "hexcasting:patterns/spells/hopper"
---

# Hopper

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexical]]

This spell is explicitly designed for the precise and mass transport of items from place to place, supporting a wide array of blocks and entities to pull and push to.

*allows arbitrary objects to be retrieved... and instantiated in reality... arbitrary information between arbitrary data sources*

## [[Hopper|Hopper]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="24.5,32.0 34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="27.8" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,41.8 25.9,38.8 29.8,36.6" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`source, num?, dest., num?` → `—`

Instantly transports all items from **source** to **destination**. Cost scales based on the amount of mass transferred - a [[The Stack|stack]] of 64 items will cost about three [[Amethyst|Amethyst Dust]]. Numbers are optional; details later.

Entity references to the caster, an **Armor Stand**, a **Minecart with a Chest**, a **Hopper Minecart**, or a **Boat with a Chest** within ambit are valid **sources** that refer to the respective inventory. They also serve as valid **destinations**.

Entity references to **Item Frames** and dropped **Items** are also valid **sources** and **destinations**. For **Items**, pushes will attempt to stack onto it.

Vectors are also possible **sources** and **destinations**. If a block with an inventory does not exist at a location, it is a valid **destination** to push items out into the world.

If a block with an inventory does exist, it serves as both a **source** and a **destination**. If the vector is closest to the center, it pushes and pulls from any inventory slot; if it is instead closer to a face, it will only push to and pull from that face, similar to **Hoppers**.

If I pass in Null, it seems to refer to my **Ender Chest**'s inventory. Naturally, this means it will only work for humans casting this spell.

Because **sources** and **destinations** can never be numbers, it is distinguishable to pass an optional number after a **source** or **destination**. This is the num? in the pattern signature and correlates to *slot*.

Slots are a slightly esoteric way that Nature has chosen to represent specific spaces in the inventory. For example, 0, 1, 2, and 3 corresponds to boots, leggings, chestplate, and helmet spaces of an **Armor Stand source** respectively. Passing in slots allows me to dictate to [[Hopper]] which space to take items from and move items to.

For quick reference, my hotbar is 0-8, my offhand is 40, and my [[Wristpocket (Book Entry)|Wristpocket]] is -1.

## [[Hopper Purification|Hopper Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:245;--dur:3.4s" points="24.5,32.0 14.3,49.7 24.5,67.5 45.0,67.5 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="32.0" x2="21.3" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="18.9,41.8 19.3,36.6 23.2,38.8" fill="#fc77be"/><circle cx="24.5" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`source` → `list`

Applies to **sources** that support slots; inventories. Returns a list of the item stacks contained within them, with their index in the list being their slot number.

While the slot number is generally fairly logical, it can feel arbitrary for storages such as a player's inventory.

This pattern will be useful in ascertaining the slot number of any given location in a **source** or **destination**'s inventory. It can also be useful for processing: scanning a **source** for what items it contains, evaluating them based on some metrics, and from there deciding whether and where to move them.

---
### Patterns on this page

- [[Hopper]] · Pattern · Other
- [[Hopper Purification]] · Pattern · Purification
