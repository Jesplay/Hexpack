---
aliases: []
tags: [hexbook/entry, mod/hexical, category/spells, topic/combat]
mod: "Hexical"
category: "Spells"
entry_id: "hexcasting:patterns/spells/magic_missile"
---

# Magic Missile

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexical]]

## [[Magic Missile|Magic Missile]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,54.7 27.0,54.7 15.8,74.2 38.3,74.2 49.5,54.7 60.8,35.3 72.0,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="49.5,54.7 27.0,54.7 15.8,74.2 38.3,74.2 49.5,54.7 60.8,35.3 72.0,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="54.7" x2="42.3" y2="54.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,54.7 42.3,52.3 42.3,57.2" fill="#fc77be"/><circle cx="49.5" cy="54.7" r="2.6" fill="#fc77be"/></svg>

`vector, vector` → `—`

Conjures a small silver of amethyst that always deals a full heart of damage and forces the target backwards. Costs about one [[Amethyst|Amethyst Dust]].

The projectile has a few interesting properties. It automatically disappears after roughly ten seconds or shatters upon hitting an entity or block. It pierces all forms of armor and protection. It is also unaffected by gravity and water drag.

*A weak but enchanting sparkling projectile.*

The first vector is conjuration location and is quite esoteric: no wonder most **Hexcasters** conjure it in front of them. The basic gist is a vector, where my head's position is the origin, [[Alidade's Purification|my facing]] is the Z+ axis, [[Theodolite's Purification|the vector pointing up from my head]] is the Y+ axis, and the vector from my head pointing rightwards is the X+ axis. For basic purposes, the zero vector conjures it inside my head and a vector resembling (0, 0, n) conjures it n blocks in front of me.

The second vector is velocity and thankfully much less complicated. It is a simple offset vector, similar to the one that [[Impulse]] takes in. Unlike [[Impulse]] though, this vector can be of arbitrary magnitude without incurring additional costs.

Unlike **Arrows** though, **Magic Missiles** will not increase in damage simply by travelling faster. They also apply an odd knockback effect that ignores the entity's mass or the projectile's speed.

---
### Patterns on this page

- [[Magic Missile]] · Spell · Spell

**Sources:** 🌐 [Official online docs](https://hexical.hexxy.media/v/2.0.0/1.0.0/en_us/#patterns/spells/magic_missile)
