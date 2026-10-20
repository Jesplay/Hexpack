---
aliases: ["Logical Operators"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/logic]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/logic"
---

# Logical Operators

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Augur's Purification|Augur's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="58.2,41.5 71.5,18.5 45.0,18.5 18.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:79;--dur:1.6s" points="58.2,41.5 71.5,18.5 45.0,18.5 18.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="58.2" y1="41.5" x2="62.5" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="65.5,28.8 65.0,35.6 59.9,32.7" fill="#fc77be"/><circle cx="58.2" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any` → `bool`

Convert an argument to a boolean. The number **0**, **Null**, False, the vector (0, 0, 0), and the empty list become False; everything else becomes True.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`bool` → `number`

Convert a boolean to a number; True becomes **1**, and False becomes **0**.

## [[Negation Purification|Negation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="31.8,41.5 18.5,18.5 45.0,18.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:79;--dur:1.6s" points="31.8,41.5 18.5,18.5 45.0,18.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="41.5" x2="27.5" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,28.8 30.1,32.7 25.0,35.6" fill="#fc77be"/><circle cx="31.8" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`bool` → `bool`

If the argument is True, return False; if it is False, return True.

## [[Disjunction Distillation|Disjunction Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="18.5,18.5 31.8,41.5 45.0,64.4 58.2,41.5 71.5,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="18.5" x2="22.8" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,31.1 20.2,27.3 25.3,24.4" fill="#fc77be"/><circle cx="18.5" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`bool, bool` → `bool`

Returns True if at least one of the arguments are True; otherwise returns False.

## [[Conjunction Distillation|Conjunction Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,64.4 31.8,41.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="18.5,64.4 31.8,41.5 45.0,18.5 58.2,41.5 71.5,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="64.4" x2="22.8" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,51.8 25.3,58.5 20.2,55.6" fill="#fc77be"/><circle cx="18.5" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`bool, bool` → `bool`

Returns True if both arguments are true; otherwise returns False.

## [[Exclusion Distillation|Exclusion Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,64.4 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:106;--dur:1.6s" points="31.8,64.4 18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="64.4" x2="27.5" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="24.5,51.8 30.1,55.6 25.0,58.5" fill="#fc77be"/><circle cx="31.8" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`bool, bool` → `bool`

Returns True if exactly one of the arguments is true; otherwise returns False.

## [[Augur's Exaltation|Augur's Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 31.8,64.4 45.0,41.5 58.2,18.5 71.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:132;--dur:1.6s" points="18.5,41.5 31.8,64.4 45.0,41.5 58.2,18.5 71.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="22.8" y2="48.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="25.8,54.1 20.2,50.2 25.3,47.3" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`bool, any, any` → `any`

If the first argument is True, keeps the second and discards the third; otherwise discards the second and keeps the third.

## [[Equality Distillation|Equality Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="70" viewBox="0 0 90.0 70.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.2" cy="21.7" r="1.6"/><circle cx="37.2" cy="21.7" r="1.6"/><circle cx="68.3" cy="21.7" r="1.6"/><circle cx="21.7" cy="48.6" r="1.6"/><circle cx="52.8" cy="48.6" r="1.6"/><circle cx="83.8" cy="48.6" r="1.6"/></g><polyline class="hexpat-base" points="21.7,48.6 52.8,48.6 37.2,21.7 68.3,21.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:93;--dur:1.6s" points="21.7,48.6 52.8,48.6 37.2,21.7 68.3,21.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="21.7" y1="48.6" x2="31.7" y2="48.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.8,48.6 31.7,52.0 31.7,45.2" fill="#fc77be"/><circle cx="21.7" cy="48.6" r="2.6" fill="#fc77be"/></svg>

`any, any` → `bool`

If the first argument equals the second (within a small tolerance), return True. Otherwise, return False.

## [[Inequality Distillation|Inequality Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="70" viewBox="0 0 90.0 70.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="21.7" cy="21.7" r="1.6"/><circle cx="52.8" cy="21.7" r="1.6"/><circle cx="83.8" cy="21.7" r="1.6"/><circle cx="6.2" cy="48.6" r="1.6"/><circle cx="37.2" cy="48.6" r="1.6"/><circle cx="68.3" cy="48.6" r="1.6"/></g><polyline class="hexpat-base" points="21.7,21.7 52.8,21.7 37.2,48.6 68.3,48.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:93;--dur:1.6s" points="21.7,21.7 52.8,21.7 37.2,48.6 68.3,48.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="21.7" y1="21.7" x2="31.7" y2="21.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.8,21.7 31.7,25.1 31.7,18.3" fill="#fc77be"/><circle cx="21.7" cy="21.7" r="2.6" fill="#fc77be"/></svg>

`any, any` → `bool`

If the first argument does not equal the second (outside a small tolerance), return True. Otherwise, return False.

## [[Maximus Distillation|Maximus Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="55" height="90" viewBox="0 0 54.6 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:57;--dur:1.6s" points="20.1,20.1 34.5,45.0 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="20.1" x2="24.7" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,33.8 22.0,29.7 27.4,26.5" fill="#fc77be"/><circle cx="20.1" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`number, number` → `bool`

If the first argument is greater than the second, return True. Otherwise, return False.

## [[Minimus Distillation|Minimus Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="55" height="90" viewBox="0 0 54.6 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,20.1 20.1,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:57;--dur:1.6s" points="34.5,20.1 20.1,45.0 34.5,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="20.1" x2="29.9" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,33.8 27.1,26.5 32.6,29.7" fill="#fc77be"/><circle cx="34.5" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`number, number` → `bool`

If the first argument is less than the second, return True. Otherwise, return False.

## [[Maximus Distillation II|Maximus Distillation II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:86;--dur:1.6s" points="48.8,20.1 63.2,45.0 48.8,69.9 20.1,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="48.8" y1="20.1" x2="53.4" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="56.8,33.8 50.7,29.7 56.2,26.5" fill="#fc77be"/><circle cx="48.8" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`number, number` → `bool`

If the first argument is greater than or equal to the second, return True. Otherwise, return False.

## [[Minimus Distillation II|Minimus Distillation II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.7" cy="20.1" r="1.6"/><circle cx="34.5" cy="20.1" r="1.6"/><circle cx="63.2" cy="20.1" r="1.6"/><circle cx="20.1" cy="45.0" r="1.6"/><circle cx="48.8" cy="45.0" r="1.6"/><circle cx="77.6" cy="45.0" r="1.6"/><circle cx="5.7" cy="69.9" r="1.6"/><circle cx="34.5" cy="69.9" r="1.6"/><circle cx="63.2" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,20.1 20.1,45.0 34.5,69.9 63.2,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:86;--dur:1.6s" points="34.5,20.1 20.1,45.0 34.5,69.9 63.2,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="20.1" x2="29.9" y2="28.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.6,33.8 27.1,26.5 32.6,29.7" fill="#fc77be"/><circle cx="34.5" cy="20.1" r="2.6" fill="#fc77be"/></svg>

`number, number` → `bool`

If the first argument is less than or equal to the second, return True. Otherwise, return False.

---
### Patterns on this page

- [[Augur's Purification]] · Pattern · Purification
- [[Length Purification]] · Pattern · Purification
- [[Negation Purification]] · Pattern · Purification
- [[Disjunction Distillation]] · Pattern · Distillation
- [[Conjunction Distillation]] · Pattern · Distillation
- [[Exclusion Distillation]] · Pattern · Distillation
- [[Augur's Exaltation]] · Pattern · Exaltation
- [[Equality Distillation]] · Pattern · Distillation
- [[Inequality Distillation]] · Pattern · Distillation
- [[Maximus Distillation]] · Pattern · Distillation
- [[Minimus Distillation]] · Pattern · Distillation
- [[Maximus Distillation II]] · Pattern · Distillation
- [[Minimus Distillation II]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/logic) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
