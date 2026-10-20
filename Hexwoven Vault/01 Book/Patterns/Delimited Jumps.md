---
aliases: ["Delimited Jumps"]
tags: [hexbook/entry, mod/caduceus, category/patterns, topic/control-flow]
mod: "Caduceus"
category: "Patterns"
entry_id: "hexcasting:caduceus/patterns/delimcc"
---

# Delimited Jumps

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Caduceus]]

## [[Thetis' Gambit|Thetis' Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 27.0,35.3 38.3,54.7 60.8,54.7 72.0,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="15.8,15.8 38.3,15.8 60.8,15.8 49.5,35.3 27.0,35.3 38.3,54.7 60.8,54.7 72.0,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="15.8" x2="23.0" y2="15.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.1,15.8 23.0,18.2 23.0,13.3" fill="#fc77be"/><circle cx="15.8" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`[pattern] | pattern` → `many`

Cast a pattern or list of patterns from the [[The Stack|stack]] exactly like [[Hermes' Gambit]], except that anything outside of this cast will not be captured by an [[Arke's Gambit]] within it.

## [[Thetis' Gambit II|Thetis' Gambit II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="72.0,74.2 60.8,54.7 38.3,54.7 27.0,35.3 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="72.0,74.2 60.8,54.7 38.3,54.7 27.0,35.3 49.5,35.3 60.8,15.8 38.3,15.8 15.8,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.0" y1="74.2" x2="68.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="65.8,63.5 70.6,66.8 66.3,69.2" fill="#fc77be"/><circle cx="72.0" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`[pattern] | pattern, any` → `many`

Like [[Thetis' Gambit]], but also sets the [[Jump Manipulation|jump tag]] of the cast to the given [[Iotas|iota]]. If the iota is Null, this is equivalent to [[Thetis' Gambit]].

## [[Arke's Gambit|Arke's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="72.0,15.8 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="72.0,15.8 49.5,15.8 27.0,15.8 38.3,35.3 60.8,35.3 49.5,54.7 27.0,54.7 15.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.0" y1="15.8" x2="64.8" y2="15.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.7,15.8 64.8,13.3 64.8,18.2" fill="#fc77be"/><circle cx="72.0" cy="15.8" r="2.6" fill="#fc77be"/></svg>

`[pattern] | pattern` → `many`

Cast a pattern or list of patterns from the stack similarly to [[Iris' Gambit]]. Must be drawn within [[Thetis' Gambit]].

All of the patterns that would have been drawn between the end of the pattern list and the end of the enclosing [[Thetis' Gambit]] are moved into a Call iota, which is pushed to the stack instead of a Jump iota.

When the Call iota is executed, the captured patterns will be cast as if by [[Hermes' Gambit]]. If the Call iota is not executed, the rest of the patterns within the enclosing [[Thetis' Gambit]] will be skipped.

## [[Arke's Gambit II|Arke's Gambit II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 49.5,54.7 60.8,35.3 38.3,35.3 27.0,15.8 49.5,15.8 72.0,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:158;--dur:2.0s" points="15.8,74.2 27.0,54.7 49.5,54.7 60.8,35.3 38.3,35.3 27.0,15.8 49.5,15.8 72.0,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`[pattern] | pattern, any` → `many`

Like [[Arke's Gambit]], but only patterns within an enclosing [[Thetis' Gambit II]] with a [[Jump Manipulation|jump tag]] matching the given iota are captured. If the iota is Null, this is equivalent to [[Arke's Gambit]].

---
### Patterns on this page

- [[Thetis' Gambit]] · Pattern · Gambit
- [[Thetis' Gambit II]] · Pattern · Gambit
- [[Arke's Gambit]] · Pattern · Gambit
- [[Arke's Gambit II]] · Pattern · Gambit

**Sources:** 🌐 [Official online docs](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/delimcc) · 🧩 [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
