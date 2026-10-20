---
aliases: ["Wisp Handling"]
tags: [hexbook/entry, mod/lapisworks, category/lapse, topic/wisps]
mod: "Lapisworks"
category: "Lapse"
entry_id: "hexcasting:interop/lapisal/wispfeatures"
---

# Wisp Handling

> **Category:** Lapisworks › Interop › [[Lapse (Category)|Lapse]]  
> **Source mod:** [[Lapisworks]]

Long has wisps being unable to hold items irritated me. Fortunately I have discovered this **Enchanted Spell** which allows me to let wisps handle items.

## [[Allocate Item|Allocate Item]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="68" viewBox="0 0 90.0 68.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/><circle cx="9.8" cy="46.4" r="1.6"/><circle cx="23.9" cy="46.4" r="1.6"/><circle cx="38.0" cy="46.4" r="1.6"/><circle cx="52.0" cy="46.4" r="1.6"/><circle cx="66.1" cy="46.4" r="1.6"/><circle cx="80.2" cy="46.4" r="1.6"/><circle cx="2.8" cy="58.6" r="1.6"/><circle cx="16.9" cy="58.6" r="1.6"/><circle cx="30.9" cy="58.6" r="1.6"/><circle cx="45.0" cy="58.6" r="1.6"/><circle cx="59.1" cy="58.6" r="1.6"/><circle cx="73.1" cy="58.6" r="1.6"/><circle cx="87.2" cy="58.6" r="1.6"/></g><polyline class="hexpat-base" points="23.9,22.0 16.9,9.8 9.8,22.0 16.9,34.2 23.9,22.0 30.9,9.8 45.0,9.8 59.1,9.8 66.1,22.0 73.1,34.2 80.2,22.0 73.1,9.8 66.1,22.0 59.1,34.2 45.0,34.2 38.0,46.4 45.0,58.6 52.0,46.4 45.0,34.2 30.9,34.2 23.9,22.0 38.0,22.0 45.0,34.2 52.0,22.0 66.1,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:338;--dur:6.0s" points="23.9,22.0 16.9,9.8 9.8,22.0 16.9,34.2 23.9,22.0 30.9,9.8 45.0,9.8 59.1,9.8 66.1,22.0 73.1,34.2 80.2,22.0 73.1,9.8 66.1,22.0 59.1,34.2 45.0,34.2 38.0,46.4 45.0,58.6 52.0,46.4 45.0,34.2 30.9,34.2 23.9,22.0 38.0,22.0 45.0,34.2 52.0,22.0 66.1,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.9" y1="22.0" x2="21.7" y2="18.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.0,15.3 23.0,17.4 20.3,18.9" fill="#fc77be"/><circle cx="23.9" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`wisp, item entity` → `—`

Swaps the item the provided wisp (that I must own) is holding and the provided item. Costs two and a half dust.

## [[Command Drop|Command Drop]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="68" viewBox="0 0 90.0 68.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.8" cy="9.8" r="1.6"/><circle cx="16.9" cy="9.8" r="1.6"/><circle cx="30.9" cy="9.8" r="1.6"/><circle cx="45.0" cy="9.8" r="1.6"/><circle cx="59.1" cy="9.8" r="1.6"/><circle cx="73.1" cy="9.8" r="1.6"/><circle cx="87.2" cy="9.8" r="1.6"/><circle cx="9.8" cy="22.0" r="1.6"/><circle cx="23.9" cy="22.0" r="1.6"/><circle cx="38.0" cy="22.0" r="1.6"/><circle cx="52.0" cy="22.0" r="1.6"/><circle cx="66.1" cy="22.0" r="1.6"/><circle cx="80.2" cy="22.0" r="1.6"/><circle cx="2.8" cy="34.2" r="1.6"/><circle cx="16.9" cy="34.2" r="1.6"/><circle cx="30.9" cy="34.2" r="1.6"/><circle cx="45.0" cy="34.2" r="1.6"/><circle cx="59.1" cy="34.2" r="1.6"/><circle cx="73.1" cy="34.2" r="1.6"/><circle cx="87.2" cy="34.2" r="1.6"/><circle cx="9.8" cy="46.4" r="1.6"/><circle cx="23.9" cy="46.4" r="1.6"/><circle cx="38.0" cy="46.4" r="1.6"/><circle cx="52.0" cy="46.4" r="1.6"/><circle cx="66.1" cy="46.4" r="1.6"/><circle cx="80.2" cy="46.4" r="1.6"/><circle cx="2.8" cy="58.6" r="1.6"/><circle cx="16.9" cy="58.6" r="1.6"/><circle cx="30.9" cy="58.6" r="1.6"/><circle cx="45.0" cy="58.6" r="1.6"/><circle cx="59.1" cy="58.6" r="1.6"/><circle cx="73.1" cy="58.6" r="1.6"/><circle cx="87.2" cy="58.6" r="1.6"/></g><polyline class="hexpat-base" points="23.9,46.4 16.9,58.6 9.8,46.4 16.9,34.2 23.9,46.4 30.9,58.6 45.0,58.6 59.1,58.6 66.1,46.4 73.1,34.2 80.2,46.4 73.1,58.6 66.1,46.4 59.1,34.2 45.0,34.2 38.0,22.0 45.0,9.8 52.0,22.0 45.0,34.2 30.9,34.2 23.9,46.4 38.0,46.4 45.0,34.2 52.0,46.4 66.1,46.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:338;--dur:6.0s" points="23.9,46.4 16.9,58.6 9.8,46.4 16.9,34.2 23.9,46.4 30.9,58.6 45.0,58.6 59.1,58.6 66.1,46.4 73.1,34.2 80.2,46.4 73.1,58.6 66.1,46.4 59.1,34.2 45.0,34.2 38.0,22.0 45.0,9.8 52.0,22.0 45.0,34.2 30.9,34.2 23.9,46.4 38.0,46.4 45.0,34.2 52.0,46.4 66.1,46.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.9" y1="46.4" x2="21.7" y2="50.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.0,53.1 20.3,49.5 23.0,51.0" fill="#fc77be"/><circle cx="23.9" cy="46.4" r="2.6" fill="#fc77be"/></svg>

`wisp` → `—`

Takes the item in the provided wisp (that I must own)'s offhand and lets it go into the world at the wisp's position. Costs two and a half dust.

While I was working on extensions to wisp functionality, I found that the [[Media Manipulation]] spells function the same for a wisp. Particularly strangely, [[Deposit Media]] considers wisps a valid target for depositing and [[Withdraw Media]] recharges the wisp instead of any [[Media|media]] holder the wisp is holding, and if targeting a wisp, the caster must own that wisp or suffer a [[Mishaps|mishap]].

---
### Patterns on this page

- [[Allocate Item]] · Spell · Spell
- [[Command Drop]] · Spell · Spell

**Sources:** 🧩 [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
