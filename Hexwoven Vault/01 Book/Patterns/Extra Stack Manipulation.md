---
aliases: ["Extra Stack Manipulation"]
tags: [hexbook/entry, mod/overevaluate, category/patterns, topic/stack]
mod: "Overevaluate"
category: "Patterns"
entry_id: "hexcasting:extra_stack"
---

# Extra Stack Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Overevaluate]]

## [[Sekhmet's Gambit|Sekhmet's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="77.6" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,20.1 20.1,45.0 34.5,69.9 48.8,45.0 34.5,20.1 63.2,20.1 48.8,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:172;--dur:1.7s" points="34.5,20.1 20.1,45.0 34.5,69.9 48.8,45.0 34.5,20.1 63.2,20.1 48.8,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="20.1" x2="29.9" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,33.8 27.1,26.5 32.6,29.7" fill="#fc77be"/><circle cx="34.5" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`—` → `varies`

Voids the entire [[The Stack|stack]] except for the top n [[Iotas|iota]], determined by tail length. In the examples above, zero, one, and two iota are left untouched respectively.

This simple gambit serves as a convenient wrapper around a specific combination of [[Flock's Reflection]], [[Flock's Gambit]], [[Selection Exaltation]], and [[Flock's Disintegration]]. It also pairs extremely well with [[Thoth's Gambit]] to leave only the iotas I am interested in on the pseudo-stack.

*Everything is possible, but not everything is necessary.*

## [[Geb's Gambit|Geb's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 18.5,41.5 31.8,64.4 45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="45.0,41.5 18.5,41.5 31.8,64.4 45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="36.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.4,41.5 36.5,38.5 36.5,44.4" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`many` → `many`

Yanks the iota n from the top up to the top, determined by tail length. In the examples above, the iota second, third, and fourth from the top are yanked respectively.

## [[Nut's Gambit|Nut's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`many` → `many`

Sinks the top iota n from the top, determined by tail length. In the examples above, the iota is moved to the second, third, and fourth place from the top respectively.

## [[Reflecting Gambit|Reflecting Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 62.3,16.2 73.8,36.1 50.8,36.1 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="50.8,36.1 62.3,16.2 73.8,36.1 50.8,36.1 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="54.5" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="57.1,25.1 56.7,31.0 52.3,28.5" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`any, any, any` → `any, any, any`

Reflects the order of the top three iota, turning A, B, C into C, B, A. Essentially swaps the top and the third-from-the-top iota.

## [[Bubbling Gambit|Bubbling Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 73.8,36.1 62.3,16.2 50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="50.8,36.1 73.8,36.1 62.3,16.2 50.8,36.1 39.2,56.1 16.2,56.1 27.7,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="58.2" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="63.5,36.1 58.2,38.7 58.2,33.6" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`any, any, any` → `any, any, any`

Causes the third-from-the-top iota to bubble up over the second-from-the-top iota, essentially swapping them.

## [[Dioscuri Gambit II|Dioscuri Gambit II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 55.2,32.0 75.7,32.0 65.5,14.3 55.2,32.0 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="34.8,32.0 55.2,32.0 75.7,32.0 65.5,14.3 55.2,32.0 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="41.3" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,32.0 41.3,34.3 41.3,29.8" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`many, number` → `many`

Takes a number and duplicates that many elements of the stack while preserving order.

---
### Patterns on this page

- [[Sekhmet's Gambit]] · Pattern · Gambit
- [[Geb's Gambit]] · Pattern · Gambit
- [[Nut's Gambit]] · Pattern · Gambit
- [[Reflecting Gambit]] · Pattern · Gambit
- [[Bubbling Gambit]] · Pattern · Gambit
- [[Dioscuri Gambit II]] · Pattern · Gambit
