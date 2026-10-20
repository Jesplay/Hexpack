---
aliases: []
tags: [hexbook/entry, mod/ephemera, category/spells, topic/visuals]
mod: "Ephemera"
category: "Spells"
entry_id: "hexcasting:patterns/spells/particle_burst"
---

# Particle Burst

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Ephemera]]

## [[Particle Burst|Particle Burst]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="77" height="90" viewBox="0 0 77.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.0" cy="11.0" r="1.6"/><circle cx="26.7" cy="11.0" r="1.6"/><circle cx="42.4" cy="11.0" r="1.6"/><circle cx="58.1" cy="11.0" r="1.6"/><circle cx="73.8" cy="11.0" r="1.6"/><circle cx="3.1" cy="24.6" r="1.6"/><circle cx="18.8" cy="24.6" r="1.6"/><circle cx="34.6" cy="24.6" r="1.6"/><circle cx="50.3" cy="24.6" r="1.6"/><circle cx="66.0" cy="24.6" r="1.6"/><circle cx="11.0" cy="38.2" r="1.6"/><circle cx="26.7" cy="38.2" r="1.6"/><circle cx="42.4" cy="38.2" r="1.6"/><circle cx="58.1" cy="38.2" r="1.6"/><circle cx="73.8" cy="38.2" r="1.6"/><circle cx="3.1" cy="51.8" r="1.6"/><circle cx="18.8" cy="51.8" r="1.6"/><circle cx="34.6" cy="51.8" r="1.6"/><circle cx="50.3" cy="51.8" r="1.6"/><circle cx="66.0" cy="51.8" r="1.6"/><circle cx="11.0" cy="65.4" r="1.6"/><circle cx="26.7" cy="65.4" r="1.6"/><circle cx="42.4" cy="65.4" r="1.6"/><circle cx="58.1" cy="65.4" r="1.6"/><circle cx="73.8" cy="65.4" r="1.6"/><circle cx="3.1" cy="79.0" r="1.6"/><circle cx="18.8" cy="79.0" r="1.6"/><circle cx="34.6" cy="79.0" r="1.6"/><circle cx="50.3" cy="79.0" r="1.6"/><circle cx="66.0" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="26.7,65.4 42.4,65.4 34.6,79.0 18.8,79.0 11.0,65.4 18.8,51.8 34.6,51.8 50.3,51.8 42.4,38.2 34.6,51.8 26.7,38.2 42.4,38.2 58.1,38.2 66.0,24.6 58.1,11.0 42.4,11.0 34.6,24.6 50.3,24.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:267;--dur:4.8s" points="26.7,65.4 42.4,65.4 34.6,79.0 18.8,79.0 11.0,65.4 18.8,51.8 34.6,51.8 50.3,51.8 42.4,38.2 34.6,51.8 26.7,38.2 42.4,38.2 58.1,38.2 66.0,24.6 58.1,11.0 42.4,11.0 34.6,24.6 50.3,24.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="26.7" y1="65.4" x2="31.7" y2="65.4" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.3,65.4 31.7,67.1 31.7,63.7" fill="#fc77be"/><circle cx="26.7" cy="65.4" r="2.6" fill="#fc77be"/></svg>

`vec, vec, num, num` → `—`

Accepts two vectors and two numbers, and uses them to define a burst of particles. Costs a negligible amount of [[Media|media]].

The first vector determines the burst's origin position, and the second vector determines its direction and speed. The first number determines the scale of random numbers added to each position component, and the second does the same for the direction vector.

Mod author's note for Forge users: This spell does not work on Forge. I do not know why. I've set the cost on Forge to zero so at least it won't waste media. Sorry.

I'll try to get it working eventually.

---
### Patterns on this page

- [[Particle Burst]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/particle_burst) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
