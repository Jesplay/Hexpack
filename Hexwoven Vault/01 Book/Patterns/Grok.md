---
aliases: ["Grok"]
tags: [hexbook/entry, mod/hexical, category/patterns, topic/storage]
mod: "Hexical"
category: "Patterns"
entry_id: "hexcasting:patterns/grok"
---

# Grok

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hexical]]

## [[Grok Reflection|Grok Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,35.3 49.5,35.3 38.3,15.8 15.8,15.8 27.0,35.3 38.3,54.7 49.5,74.2 72.0,74.2 60.8,54.7 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="27.0,35.3 49.5,35.3 38.3,15.8 15.8,15.8 27.0,35.3 38.3,54.7 49.5,74.2 72.0,74.2 60.8,54.7 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="35.3" x2="34.2" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,35.3 34.2,37.7 34.2,32.8" fill="#fc77be"/><circle cx="27.0" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`—` → `list, list`

Pushes two lists; one of the [[The Stack|stack]] and one of the unfinished list building after an unclosed [[Introspection]]. Entity references to players that are not me become Null.

## [[Grok Gambit|Grok Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,35.3 49.5,35.3 60.8,54.7 72.0,74.2 49.5,74.2 38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="27.0,35.3 49.5,35.3 60.8,54.7 72.0,74.2 49.5,74.2 38.3,54.7 27.0,35.3 15.8,15.8 38.3,15.8 49.5,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="35.3" x2="34.2" y2="35.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.4,35.3 34.2,37.7 34.2,32.8" fill="#fc77be"/><circle cx="27.0" cy="35.3" r="2.6" fill="#fc77be"/></svg>

`list, list` → `—`

Takes two lists of [[Iotas|iotas]] and sets them as my stack and my parenthesized list respectively. Entity references to players that are not me become Null.

---
### Patterns on this page

- [[Grok Reflection]] · Pattern · Reflection
- [[Grok Gambit]] · Spell · Gambit
