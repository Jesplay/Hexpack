---
aliases: ["Null Coercion"]
tags: [hexbook/entry, mod/hextra-patterns, category/patterns, topic/logic]
mod: "Hextra Patterns"
category: "Patterns"
entry_id: "hexcasting:patterns/hextrapats_nulls"
---

# Null Coercion

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hextra Patterns]]

## [[Coalescing Distillation|Coalescing Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="18.5" x2="22.8" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,31.1 20.2,27.3 25.3,24.4" fill="#fc77be"/><circle cx="18.5" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`any, any` → `any`

If the first argument is not Null, returns it. Otherwise, returns the second argument.

## [[Coalescing Gambit|Coalescing Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5 31.8,41.5 18.5,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5 45.0,18.5 58.2,41.5 31.8,41.5 18.5,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="18.5" x2="22.8" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,31.1 20.2,27.3 25.3,24.4" fill="#fc77be"/><circle cx="18.5" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`any, [pattern] | pattern` → `any`

If the first argument is not Null, returns it. Otherwise, executes the second argument, much like [[Hermes' Gambit]].

---
### Patterns on this page

- [[Coalescing Distillation]] · Pattern · Distillation
- [[Coalescing Gambit]] · Pattern · Gambit

**Sources:** 🧩 [Hextra Patterns on Modrinth](https://modrinth.com/mod/WMchsbC8)
