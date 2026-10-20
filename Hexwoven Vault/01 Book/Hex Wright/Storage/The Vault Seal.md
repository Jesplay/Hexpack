---
aliases: ["The Vault Seal", "Manifold Vault Seal"]
tags: [hexbook/entry, mod/hexwright, category/storage, topic/hexwright, topic/storage]
mod: "Hexwright"
category: "Storage"
entry_id: "hexcasting:hex_wright/storage/seal"
---

# The Vault Seal

> **Category:** Hex Wright › [[Storage (Category)|Storage]]  
> **Source mod:** [[Hexwright]]

### Manifold Vault Seal

*Item:* **Manifold Vault Seal**

Vault Seals provide access to a [[The Manifold Vault|Manifold Vault]] over any distance.

Forged at the [[Essence Forge]] they are blank until used on a Manifold Vault, which then imbues a spatial imprint.

### Using a seal

A seal can be docked in a [[Sealed Satchel|satchel]] or [[The Hexbound Chest|chest]], worn, or held. The order of priority for a seal when being read is. Docked Seal -&gt; Worn Seal -&gt; Carried Seal.

There are 3 patterns to access a Manifold Vault using a seal.

## [[Manifold's Reflection|Manifold's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,56.1 50.8,56.1 27.7,56.1 16.2,36.1 27.7,16.2 50.8,16.2 62.3,36.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="73.8,56.1 50.8,56.1 27.7,56.1 16.2,36.1 27.7,16.2 50.8,16.2 62.3,36.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="56.1" x2="66.5" y2="56.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="61.2,56.1 66.5,53.6 66.5,58.7" fill="#fc77be"/><circle cx="73.8" cy="56.1" r="2.6" fill="#fc77be"/></svg>

`—` → `[itemstack]`

Returns a list of every slot in the Manifold Vault, empty ones included, as Stacks in slot order.

## [[Manifold's Gambit|Manifold's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="58" viewBox="0 0 90.0 57.5"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/></g><polyline class="hexpat-base" points="58.8,44.7 40.4,44.7 22.0,44.7 12.9,28.8 22.0,12.9 40.4,12.9 49.6,28.8 58.8,44.7 77.1,44.7 68.0,28.8 58.8,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.8s" points="58.8,44.7 40.4,44.7 22.0,44.7 12.9,28.8 22.0,12.9 40.4,12.9 49.6,28.8 58.8,44.7 77.1,44.7 68.0,28.8 58.8,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.8" y1="44.7" x2="52.9" y2="44.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.7,44.7 52.9,42.6 52.9,46.7" fill="#fc77be"/><circle cx="58.8" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`item entity, num` → `—`

Put the item entity in the Manifold Vault at the slot given. Player cast displaced items go to inventory.
Costs 1/10 Dust

## [[Manifold's Distillation|Manifold's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="58.8,44.7 40.4,44.7 22.0,44.7 12.9,28.8 22.0,12.9 40.4,12.9 49.6,28.8 58.8,44.7 77.1,44.7 68.0,60.6 58.8,44.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.8s" points="58.8,44.7 40.4,44.7 22.0,44.7 12.9,28.8 22.0,12.9 40.4,12.9 49.6,28.8 58.8,44.7 77.1,44.7 68.0,60.6 58.8,44.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.8" y1="44.7" x2="52.9" y2="44.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="48.7,44.7 52.9,42.6 52.9,46.7" fill="#fc77be"/><circle cx="58.8" cy="44.7" r="2.6" fill="#fc77be"/></svg>

`stack, num` → `item`

Withdraws item at your feet, returning that item entity. Player cast goes to inventory or feet if full. Costs 1 Dust

When this pattern is used in a [[Sealed Satchel]] or [[The Hexbound Chest|Hexbound Chest]] withdraw hex, the item is placed at your feet and then under your cursor in one tick, fast enough that the item never appears on the floor for someone else to pick up. It is placed on the floor so the entity can be captured.

---
### Patterns on this page

- [[Manifold's Reflection]] · Pattern · Reflection
- [[Manifold's Gambit]] · Spell · Gambit
- [[Manifold's Distillation]] · Pattern · Distillation

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
