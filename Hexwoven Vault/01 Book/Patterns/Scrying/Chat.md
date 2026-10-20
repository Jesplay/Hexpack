---
aliases: ["Chat"]
tags: [hexbook/entry, mod/hexpose, category/scrying, topic/text]
mod: "Hexpose"
category: "Scrying"
entry_id: "hexcasting:patterns/scrying/chat"
---

# Chat

> **Category:** Patterns › [[Scrying (Category)|Scrying]]  
> **Source mod:** [[Hexpose]]

## [[News Reflection|News Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="27.0,15.8 15.8,35.3 38.3,35.3 49.5,54.7 38.3,74.2 15.8,74.2 27.0,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="27.0,15.8 15.8,35.3 38.3,35.3 49.5,54.7 38.3,74.2 15.8,74.2 27.0,54.7 38.3,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="27.0" y1="15.8" x2="23.4" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.5 21.3,20.8 25.6,23.2" fill="#fc77be"/><circle cx="27.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`—` → `text, text, number`

Pushes a [[Text Iota|text iota]] of the last person to use chat, a [[Text Iota|text iota]] of the message, and a number of twenieths of a second since that message sent.

## [[Newspaper Reflection|Newspaper Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="38.3,15.8 49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 38.3,54.7 27.0,35.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="38.3,15.8 49.5,35.3 27.0,35.3 15.8,54.7 27.0,74.2 49.5,74.2 38.3,54.7 27.0,35.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="38.3" y1="15.8" x2="41.9" y2="22.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.5,26.5 39.7,23.2 44.0,20.8" fill="#fc77be"/><circle cx="38.3" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`—` → `list of lists`

Applies [[News Reflection]] to the last up to 32 messages in chat, wraps each in a list, and pushes a list of those lists.

---
### Patterns on this page

- [[News Reflection]] · Pattern · Reflection
- [[Newspaper Reflection]] · Pattern · Reflection

**Sources:** 🌐 [Official online docs](https://hexpose.hexxy.media/v/3.0.0/1.0.0/en_us/#patterns/scrying/chat) · 🧩 [Hexpose on Modrinth](https://modrinth.com/mod/VWvaiZqR)
