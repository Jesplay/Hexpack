---
aliases: ["Mind Vault"]
tags: [hexbook/entry, mod/hexintent, category/items, topic/storage]
mod: "HexIntent"
category: "Items"
entry_id: "hexcasting:manifestation/mind_vault"
---

# Mind Vault

> **Category:** [[Items (Category)|Items]]  
> **Source mod:** [[HexIntent]]

### Mind Vault

*Item:* **Mind Vault**

The Mind Vault lets you flay villagers body and mind into the vault, storing it for later use. The true innovation is that by consuming the essence of the villager, it allows you to reuse their mind over and over, with a cooldown between uses. The downside is that one vault can only store one type of villager mind, at that level.

It's set by the first villager you store, so choose wisely.

Takes a villager entity and a vector location of the mind vault. Flaying the villager into the vault stores their mind and locks the vault to that type and level of villager.

## [[Mind Vault Counts|Mind Vault Counts]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5 45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5 45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="22.8" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,54.1 20.2,50.2 25.3,47.3" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`vec` → `[num, num]`

Takes a vector of a mind vault location and returns \[ready, total\], where ready is the number of stored minds not on cooldown and total is the number of stored minds in the vault.

---
### Patterns on this page

- [[Mind Vault Counts]] · Pattern · Other

**Sources:** 🧩 [HexIntent on Modrinth](https://modrinth.com/mod/4G64Rm66)
