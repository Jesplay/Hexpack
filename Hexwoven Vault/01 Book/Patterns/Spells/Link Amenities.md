---
aliases: ["Link Amenities", "Network Gateway Probe", "Network Routing Index"]
tags: [hexbook/entry, mod/ephemera, category/spells, topic/wisps]
mod: "Ephemera"
category: "Spells"
entry_id: "hexcasting:patterns/spells/network"
---

# Link Amenities

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Ephemera]]

I've devised a suite of patterns (plus a couple of blocks) to make links a bit more useful for things other than data transmission and [[Media|media]] sharing.

Note that the patterns which fetch information about the network make no promises regarding whether returned linkables are within ambit.

## [[Network Distillation|Network Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="10.7,23.9 25.9,23.9 33.6,37.1 48.8,37.1 56.4,23.9 48.8,10.7 33.6,10.7 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 79.3,37.1 71.7,23.9 79.3,10.7 64.1,10.7 56.4,23.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:229;--dur:4.2s" points="10.7,23.9 25.9,23.9 33.6,37.1 48.8,37.1 56.4,23.9 48.8,10.7 33.6,10.7 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 79.3,37.1 71.7,23.9 79.3,10.7 64.1,10.7 56.4,23.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="23.9" x2="15.6" y2="23.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.1,23.9 15.6,25.6 15.6,22.2" fill="#fc77be"/><circle cx="10.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`linkable, num` → `[linkable]`

Scans the link network connected to the input linkable up to a recursion depth equal to the input number (max 32), and returns a list of all connected linkables. Costs a negligible amount of media.

## [[Network Gateway|Network Gateway]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="32.8,19.0 26.8,8.5 14.6,8.5 8.5,19.0 14.6,29.6 26.8,29.6 32.8,19.0 38.9,8.5 45.0,19.0 38.9,29.6 32.8,19.0 45.0,19.0 57.2,19.0 51.1,29.6 63.2,29.6 75.4,29.6 81.5,19.0 75.4,8.5 63.2,8.5 51.1,8.5 57.2,19.0 69.3,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:255;--dur:5.9s" points="32.8,19.0 26.8,8.5 14.6,8.5 8.5,19.0 14.6,29.6 26.8,29.6 32.8,19.0 38.9,8.5 45.0,19.0 38.9,29.6 32.8,19.0 45.0,19.0 57.2,19.0 51.1,29.6 63.2,29.6 75.4,29.6 81.5,19.0 75.4,8.5 63.2,8.5 51.1,8.5 57.2,19.0 69.3,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="32.8" y1="19.0" x2="30.9" y2="15.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.5,13.3 32.1,15.0 29.7,16.3" fill="#fc77be"/><circle cx="32.8" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`entity, linkable, vec` → `—`

Accepts an entity to teleport, a linkable to use for access to its network, and a destination. If the entity is within 8 blocks of the linkable, and there is a network node within 4 blocks of the destination, the entity will be teleported. Cost varies based on what entity is teleported.

**Network Gateway Probe**: shaped crafting

|   |   |   |
|---|---|---|
|   | `E` |   |
| `S` | `S` | `S` |
| `S` | `C` | `S` |

`E` = Ender Pearl · `S` = Block of Slate · `C` = Charged Amethyst

Outputs a brief redstone signal when an adjacent relay is used as the output for a casting of [[Network Gateway]].

**Network Routing Index**: shaped crafting

|   |   |   |
|---|---|---|
|   | `F` |   |
| `S` | `S` | `S` |
| `S` | `C` | `S` |

`F` = Focus · `S` = Block of Slate · `C` = Charged Amethyst

When rclicked with a [[Focus|focus]] in the main hand, attempts to store its [[Iotas|iota]] in the block. When the focus is in the offhand, writes the stored iota to the focus.

## [[Routing Distillation|Routing Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="48" viewBox="0 0 90.0 47.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="10.7" r="1.6"/><circle cx="18.3" cy="10.7" r="1.6"/><circle cx="33.6" cy="10.7" r="1.6"/><circle cx="48.8" cy="10.7" r="1.6"/><circle cx="64.1" cy="10.7" r="1.6"/><circle cx="79.3" cy="10.7" r="1.6"/><circle cx="10.7" cy="23.9" r="1.6"/><circle cx="25.9" cy="23.9" r="1.6"/><circle cx="41.2" cy="23.9" r="1.6"/><circle cx="56.4" cy="23.9" r="1.6"/><circle cx="71.7" cy="23.9" r="1.6"/><circle cx="86.9" cy="23.9" r="1.6"/><circle cx="3.1" cy="37.1" r="1.6"/><circle cx="18.3" cy="37.1" r="1.6"/><circle cx="33.6" cy="37.1" r="1.6"/><circle cx="48.8" cy="37.1" r="1.6"/><circle cx="64.1" cy="37.1" r="1.6"/><circle cx="79.3" cy="37.1" r="1.6"/></g><polyline class="hexpat-base" points="10.7,23.9 25.9,23.9 33.6,37.1 48.8,37.1 56.4,23.9 48.8,10.7 33.6,10.7 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 79.3,37.1 71.7,23.9 79.3,10.7 64.1,10.7 56.4,23.9 71.7,23.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:244;--dur:4.5s" points="10.7,23.9 25.9,23.9 33.6,37.1 48.8,37.1 56.4,23.9 48.8,10.7 33.6,10.7 25.9,23.9 41.2,23.9 56.4,23.9 64.1,37.1 79.3,37.1 71.7,23.9 79.3,10.7 64.1,10.7 56.4,23.9 71.7,23.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="10.7" y1="23.9" x2="15.6" y2="23.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.1,23.9 15.6,25.6 15.6,22.2" fill="#fc77be"/><circle cx="10.7" cy="23.9" r="2.6" fill="#fc77be"/></svg>

`linkable, iota` → `vec`

Accepts a linkable to use for network access, and any iota. If a relay connected to the network has an adjacent Network Routing Index with the same iota, returns that relay's position. Costs half an amethyst dust.

---
### Patterns on this page

- [[Network Distillation]] · Pattern · Distillation
- [[Network Gateway]] · Spell · Spell
- [[Routing Distillation]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/network) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
