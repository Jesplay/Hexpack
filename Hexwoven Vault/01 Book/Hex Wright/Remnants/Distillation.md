---
aliases: ["Distillation"]
tags: [hexbook/entry, mod/hexwright, category/remnants, topic/hexwright]
mod: "Hexwright"
category: "Remnants"
entry_id: "hexcasting:hex_wright/remnants/distillation"
---

# Distillation

> **Category:** Hex Wright › [[Remnants (Category)|Remnants]]  
> **Source mod:** [[Hexwright]]

### Distillation

Remnants themselves are useless, they're scattered energy that represent all the traits the creature had when living. To be useful, we need to distil one Remnant. It's best to do this quickly as Remnants quickly degrade, even when stored. The same goes for distilled Remnants. Get them in a [[The Hex Engraved Bottle|bottle]]!

## [[Remnant's Reflection|Remnant's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="85" height="90" viewBox="0 0 84.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="11.0" r="1.6"/><circle cx="18.8" cy="11.0" r="1.6"/><circle cx="34.6" cy="11.0" r="1.6"/><circle cx="50.3" cy="11.0" r="1.6"/><circle cx="66.0" cy="11.0" r="1.6"/><circle cx="81.7" cy="11.0" r="1.6"/><circle cx="11.0" cy="24.6" r="1.6"/><circle cx="26.7" cy="24.6" r="1.6"/><circle cx="42.4" cy="24.6" r="1.6"/><circle cx="58.1" cy="24.6" r="1.6"/><circle cx="73.8" cy="24.6" r="1.6"/><circle cx="3.1" cy="38.2" r="1.6"/><circle cx="18.8" cy="38.2" r="1.6"/><circle cx="34.6" cy="38.2" r="1.6"/><circle cx="50.3" cy="38.2" r="1.6"/><circle cx="66.0" cy="38.2" r="1.6"/><circle cx="81.7" cy="38.2" r="1.6"/><circle cx="11.0" cy="51.8" r="1.6"/><circle cx="26.7" cy="51.8" r="1.6"/><circle cx="42.4" cy="51.8" r="1.6"/><circle cx="58.1" cy="51.8" r="1.6"/><circle cx="73.8" cy="51.8" r="1.6"/><circle cx="3.1" cy="65.4" r="1.6"/><circle cx="18.8" cy="65.4" r="1.6"/><circle cx="34.6" cy="65.4" r="1.6"/><circle cx="50.3" cy="65.4" r="1.6"/><circle cx="66.0" cy="65.4" r="1.6"/><circle cx="81.7" cy="65.4" r="1.6"/><circle cx="11.0" cy="79.0" r="1.6"/><circle cx="26.7" cy="79.0" r="1.6"/><circle cx="42.4" cy="79.0" r="1.6"/><circle cx="58.1" cy="79.0" r="1.6"/><circle cx="73.8" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="26.7,24.6 18.8,38.2 11.0,51.8 18.8,65.4 26.7,79.0 42.4,79.0 58.1,79.0 66.0,65.4 73.8,51.8 66.0,38.2 58.1,24.6 66.0,11.0 50.3,11.0 34.6,11.0 18.8,11.0 26.7,24.6 34.6,38.2 42.4,51.8 50.3,38.2 58.1,24.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:298;--dur:5.3s" points="26.7,24.6 18.8,38.2 11.0,51.8 18.8,65.4 26.7,79.0 42.4,79.0 58.1,79.0 66.0,65.4 73.8,51.8 66.0,38.2 58.1,24.6 66.0,11.0 50.3,11.0 34.6,11.0 18.8,11.0 26.7,24.6 34.6,38.2 42.4,51.8 50.3,38.2 58.1,24.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="26.7" y1="24.6" x2="24.2" y2="28.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.4,32.1 22.7,28.1 25.7,29.8" fill="#fc77be"/><circle cx="26.7" cy="24.6" r="2.6" fill="#fc77be"/></svg>

`remnants` → `list`

Takes a Remnants [[Iotas|Iota]] and returns a string list of the Remnants present, in order.

Reflection is useful for querying the Remnants, as different mobs have different Remnants. You might prefer to always take a Might Remnant if present, over a Vitality Remnant, so querying them allows you to create logic to extract Remnants under specific circumstances.

## [[Remnant Distillation|Remnant Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="85" height="90" viewBox="0 0 84.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.1" cy="11.0" r="1.6"/><circle cx="18.8" cy="11.0" r="1.6"/><circle cx="34.6" cy="11.0" r="1.6"/><circle cx="50.3" cy="11.0" r="1.6"/><circle cx="66.0" cy="11.0" r="1.6"/><circle cx="81.7" cy="11.0" r="1.6"/><circle cx="11.0" cy="24.6" r="1.6"/><circle cx="26.7" cy="24.6" r="1.6"/><circle cx="42.4" cy="24.6" r="1.6"/><circle cx="58.1" cy="24.6" r="1.6"/><circle cx="73.8" cy="24.6" r="1.6"/><circle cx="3.1" cy="38.2" r="1.6"/><circle cx="18.8" cy="38.2" r="1.6"/><circle cx="34.6" cy="38.2" r="1.6"/><circle cx="50.3" cy="38.2" r="1.6"/><circle cx="66.0" cy="38.2" r="1.6"/><circle cx="81.7" cy="38.2" r="1.6"/><circle cx="11.0" cy="51.8" r="1.6"/><circle cx="26.7" cy="51.8" r="1.6"/><circle cx="42.4" cy="51.8" r="1.6"/><circle cx="58.1" cy="51.8" r="1.6"/><circle cx="73.8" cy="51.8" r="1.6"/><circle cx="3.1" cy="65.4" r="1.6"/><circle cx="18.8" cy="65.4" r="1.6"/><circle cx="34.6" cy="65.4" r="1.6"/><circle cx="50.3" cy="65.4" r="1.6"/><circle cx="66.0" cy="65.4" r="1.6"/><circle cx="81.7" cy="65.4" r="1.6"/><circle cx="11.0" cy="79.0" r="1.6"/><circle cx="26.7" cy="79.0" r="1.6"/><circle cx="42.4" cy="79.0" r="1.6"/><circle cx="58.1" cy="79.0" r="1.6"/><circle cx="73.8" cy="79.0" r="1.6"/></g><polyline class="hexpat-base" points="26.7,24.6 18.8,38.2 11.0,51.8 18.8,65.4 26.7,79.0 42.4,79.0 58.1,79.0 66.0,65.4 73.8,51.8 66.0,38.2 58.1,24.6 66.0,11.0 50.3,11.0 34.6,11.0 18.8,11.0 26.7,24.6 34.6,38.2 26.7,51.8 34.6,65.4 50.3,65.4 58.1,51.8 50.3,38.2 58.1,24.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:346;--dur:6.0s" points="26.7,24.6 18.8,38.2 11.0,51.8 18.8,65.4 26.7,79.0 42.4,79.0 58.1,79.0 66.0,65.4 73.8,51.8 66.0,38.2 58.1,24.6 66.0,11.0 50.3,11.0 34.6,11.0 18.8,11.0 26.7,24.6 34.6,38.2 26.7,51.8 34.6,65.4 50.3,65.4 58.1,51.8 50.3,38.2 58.1,24.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="26.7" y1="24.6" x2="24.2" y2="28.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.4,32.1 22.7,28.1 25.7,29.8" fill="#fc77be"/><circle cx="26.7" cy="24.6" r="2.6" fill="#fc77be"/></svg>

`remnants, num | str` → `remnant`

Distils one remnant from the remnants, by index or name. Empty if not present.
Costs 1/4 Dust.

---
### Patterns on this page

- [[Remnant's Reflection]] · Pattern · Reflection
- [[Remnant Distillation]] · Pattern · Distillation

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
