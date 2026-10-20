---
aliases: []
tags: [hexbook/entry, mod/ephemera, category/great-spells, topic/effects]
mod: "Ephemera"
category: "Great Spells"
entry_id: "hexcasting:patterns/spells/great/magearmor"
---

# Instant Aegis

> **Category:** Patterns › [[Great Spells (Category)|Great Spells]]  
> **Source mod:** [[Ephemera]]

I've discovered a rather interesting spell, capable of producing a defensive [[Media|media]] barrier that can match and even exceed the protective abilities of many kinds of conventional armor. It can even apply a wide array of [[Status Iotas|persistent effects]] to the wearer, allowing for incredible versatility.

## [[Instant Aegis|Instant Aegis]] ⭐ *Great Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="36.7,26.1 28.3,11.7 11.7,11.7 20.0,26.1 28.3,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 61.7,40.5 70.0,26.1 78.3,11.7 61.7,11.7 53.3,26.1 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 45.0,40.5 36.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:367;--dur:6.0s" points="36.7,26.1 28.3,11.7 11.7,11.7 20.0,26.1 28.3,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 61.7,40.5 70.0,26.1 78.3,11.7 61.7,11.7 53.3,26.1 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 53.3,26.1 45.0,40.5 36.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="26.1" x2="34.0" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,18.2 35.6,20.6 32.4,22.4" fill="#fc77be"/><circle cx="36.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`player, num, num, status, num` → `—`

Accepts a player to grant the armor to, the durability of the armor, the protection value of the armor (max 10), a status effect to apply, and that effect's level. The latter two can be replaced with any non-status [[Iotas|iota]] to ignore them.

Of course, this barrier is by no means indestructible, and its durability decays steadily in addition to any damage it takes from hits. The base rate of this decay can be described as one "point" per second, plus additional points equal to the squared strength of any effect imbued into it. Additionally, it will disappear instantly if removed from the wearer, so there is no way to repair it, short of recasting the spell to overwrite its current state.

The media cost for this spell is a bit complicated: A base of one amethyst dust per 60 durability, multiplied by the armor's protection value raised to the power of 1.5, times the level+1 of any status effect. Thus, producing no-status strength 10 armor with one minute's worth of durability costs ~31.5 dust. Adding Regeneration 2 increases that media cost to ~94.5 dust, and causes it to only last 36 seconds.

This explanation assumes I am wearing no other armor, in practice the cost can be reduced to as little as 25%% of that formula's output if all but one of my armor slots is already occupied by more conventional armor.

---
### Patterns on this page

- [[Instant Aegis]] · Great Spell · Great Spell

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/spells/great/magearmor) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
