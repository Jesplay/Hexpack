---
aliases: ["Iota Patching"]
tags: [hexbook/entry, mod/iotaworks, category/patterns, topic/data-structures]
mod: "Iotaworks"
category: "Patterns"
entry_id: "hexcasting:addon/iotaworks/metatables"
---

# Iota Patching

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Iotaworks]]

I've discovered a way to create a new type of [[Iotas|iota]] — an abhorrence against Nature of my own doing. This pattern weaves a 'sheath' of **[[Media|media]]** out of some iota, letting me customize several attributes of the new iota. Though this can make my spells more expressive, this ability is not to be taken lightly — I cannot imagine the mischief others would do with these 'patchwork' iotas.

To assemble a patchwork iota, I must provide four source iotas:

- The 'userdata' iota, passed to my overload **Hexes**.
- The 'display' iota, visible to my eyes.
- A color for the iota, as an RGB vector in the 0-1 range.
- A pointer to an **overload map.**

Such an 'overload map' is merely a regular [[Maps|Map]] iota (mapping patterns to the **Hexes** they should be replaced with), but stored within a [[Property Iota|Property]]. I assume that Nature's reason for boxing like this is to save space: it's likely I'll have many patched iotas all using the same overloads. This also gives me the convenience of updating every instance with merely [[Schrödinger's Gambit]], rather than finding and updating them all by hand.

## [[Patchwork Exaltation|Patchwork Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="74" height="90" viewBox="0 0 73.7 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="50.5" cy="9.6" r="1.6"/><circle cx="64.1" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="57.3" cy="21.4" r="1.6"/><circle cx="71.0" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="50.5" cy="33.2" r="1.6"/><circle cx="64.1" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="57.3" cy="45.0" r="1.6"/><circle cx="71.0" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="50.5" cy="56.8" r="1.6"/><circle cx="64.1" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="57.3" cy="68.6" r="1.6"/><circle cx="71.0" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/><circle cx="50.5" cy="80.4" r="1.6"/><circle cx="64.1" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="43.7,21.4 50.5,33.2 36.8,33.2 30.0,21.4 23.2,33.2 30.0,45.0 43.7,45.0 57.3,45.0 64.1,33.2 57.3,21.4 43.7,21.4 36.8,9.6 30.0,21.4 16.4,21.4 9.6,33.2 16.4,45.0 30.0,45.0 36.8,56.8 43.7,45.0 50.5,56.8 43.7,68.6 36.8,56.8 30.0,68.6 36.8,80.4 43.7,68.6 30.0,68.6 23.2,56.8 30.0,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:368;--dur:6.0s" points="43.7,21.4 50.5,33.2 36.8,33.2 30.0,21.4 23.2,33.2 30.0,45.0 43.7,45.0 57.3,45.0 64.1,33.2 57.3,21.4 43.7,21.4 36.8,9.6 30.0,21.4 16.4,21.4 9.6,33.2 16.4,45.0 30.0,45.0 36.8,56.8 43.7,45.0 50.5,56.8 43.7,68.6 36.8,56.8 30.0,68.6 36.8,80.4 43.7,68.6 30.0,68.6 23.2,56.8 30.0,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="43.7" y1="21.4" x2="45.8" y2="25.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="47.4,27.9 44.5,25.9 47.1,24.4" fill="#fc77be"/><circle cx="43.7" cy="21.4" r="2.6" fill="#fc77be"/></svg>

`iota, iota, vec, property` → `patchwork`

Constructs a Patchwork iota from the userdata (iota stored inside), display iota (visible to my eyes), color, and an overload map.

These **overload maps** resemble a technique I've seen before — the concept of redefining patterns on a fundamental level resembles the moon cultists' (little time to explain) [metatables](https://www.lua.org/pil/13.html), which allow associating a table with another table to redefine its operations. When my 'metamethods' are invoked, the userdata iota is pushed to the top of the [[The Stack|stack]]. All other information about the patchwork disintegrates: I'll need to recreate it from scratch (maybe I should use a

'constructor' macro?) if I want to return a new iota of the same type.

Important note: unlike these 'metatables' I've heard about, patchwork *replaces* the behavior of the underlying iota. For example, operators such as &lt;ne,waaw&gt; won't work on two patchwork numbers unless I define it myself. However, I feel uneasy relying on this behavior — Nature will be perfectly happy to change this at any moment.

These **overload maps** resemble a technique I've heard of before — the concept of redefining patterns on a fundamental level resembles the moon cultists' (little time to explain) [metatables](https://www.lua.org/pil/13.html), which allow associating a table with another table to redefine its operations. When my 'metamethods' are invoked, the userdata iota is pushed to the top of the stack. All other information about the patchwork disintegrates: I'll need to recreate it from scratch (maybe I should use a 'constructor' macro?) if I want to return a new iota of the same type.

Important note: unlike these 'metatables' I've heard about, patchwork *replaces* the behavior of the underlying iota. For example, operators such as &lt;ne,waaw&gt; won't work on two patchwork numbers unless I define it myself. However, I feel uneasy relying on this behavior — Nature will be perfectly happy to change this at any moment.

---
### Patterns on this page

- [[Patchwork Exaltation]] · Pattern · Exaltation
