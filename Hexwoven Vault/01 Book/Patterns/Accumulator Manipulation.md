---
aliases: ["Accumulator Manipulation"]
tags: [hexbook/entry, mod/ephemera, category/patterns, topic/control-flow]
mod: "Ephemera"
category: "Patterns"
entry_id: "hexcasting:patterns/metathoth"
---

# Accumulator Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Ephemera]]

It seems that the list which [[Thoth's Gambit]] forms as it iterates over a data set functions as a [[The Stack|stack]], simply one that is a bit out of the way. As such, I have devised a couple of patterns for interacting with this hidden stack, called the accumulator.

## [[Seshat's Gambit|Seshat's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="36.7,26.1 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,26.1 45.0,40.5 53.3,26.1 61.7,40.5 70.0,26.1 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:183;--dur:3.1s" points="36.7,26.1 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,26.1 45.0,40.5 53.3,26.1 61.7,40.5 70.0,26.1 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="36.7" y1="26.1" x2="34.0" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="32.1,18.2 35.6,20.6 32.4,22.4" fill="#fc77be"/><circle cx="36.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `any | null`

Pops an [[Iotas|iota]] from the accumulator and pushes it to the current stack. [[Mishaps|Mishaps]] if used outside of [[Thoth's Gambit]].

## [[Seshat's Reflection|Seshat's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="14.3,67.5 24.5,49.7 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,67.5 45.0,49.7 55.2,67.5 65.5,49.7 75.7,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="14.3,67.5 24.5,49.7 34.8,32.0 24.5,14.3 14.3,32.0 24.5,49.7 34.8,67.5 45.0,49.7 55.2,67.5 65.5,49.7 75.7,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="67.5" x2="17.6" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,57.7 19.5,62.9 15.6,60.7" fill="#fc77be"/><circle cx="14.3" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Returns how many iotas are currently accessible via [[Seshat's Gambit]]. Returns -1 if used outside of [[Thoth's Gambit]].

---
### Patterns on this page

- [[Seshat's Gambit]] · Pattern · Gambit
- [[Seshat's Reflection]] · Pattern · Reflection

**Sources:** 🌐 [Official online docs](https://ephemera.hexxy.media/v/0.3.1/1.0.dev0/en_us/#patterns/metathoth) · 🧩 [Ephemera on Modrinth](https://modrinth.com/mod/yMW33ga1)
