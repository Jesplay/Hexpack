---
aliases: ["Iota Hashing"]
tags: [hexbook/entry, mod/ephemera, category/patterns, topic/data-structures]
mod: "Ephemera"
category: "Patterns"
entry_id: "hexcasting:patterns/hashing"
---

# Iota Hashing

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Ephemera]]

I've discovered a curious pattern which scrambles an [[Iotas|iota]] into an unreadable mess. However, unlike Garbage iotas, *this* mess still contains data, and running the same iota through the pattern will always produce the same mess (known as a hash). While it is not possible to recover an iota from a hash, it can be quite useful for comparisons in situations where I cannot save the unaltered iota.

## [[Hashing Purification|Hashing Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="27.7,36.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:208;--dur:2.5s" points="27.7,36.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.7" y1="36.1" x2="31.4" y2="42.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="34.0,47.1 29.2,43.8 33.6,41.3" fill="#fc77be"/><circle cx="27.7" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`any` → `hash`

Accepts any iota, and converts it into its hashed form.

---
### Patterns on this page

- [[Hashing Purification]] · Pattern · Purification

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/hashing) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
