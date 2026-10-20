---
aliases: ["Stack Manipulation"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/stack]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/stackmanip"
---

# Stack Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

## [[Novice's Gambit|Novice's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="78" viewBox="0 0 83.1 78.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.9" cy="24.2" r="1.6"/><circle cx="41.5" cy="24.2" r="1.6"/><circle cx="76.2" cy="24.2" r="1.6"/><circle cx="24.2" cy="54.2" r="1.6"/><circle cx="58.8" cy="54.2" r="1.6"/></g><polyline class="hexpat-base" points="24.2,54.2 58.8,54.2 41.5,24.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:69;--dur:1.6s" points="24.2,54.2 58.8,54.2 41.5,24.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.2" y1="54.2" x2="35.3" y2="54.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="43.3,54.2 35.3,58.0 35.3,50.4" fill="#fc77be"/><circle cx="24.2" cy="54.2" r="2.6" fill="#fc77be"/></svg>

`any` → `—`

Removes the first [[Iotas|iota]] from the [[The Stack|stack]].

This seems to be a special case of [[Bookkeeper's Gambit]].

## [[Jester's Gambit|Jester's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any, any` → `any, any`

Swaps the top two iotas of the stack.

## [[Rotation Gambit|Rotation Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 18.5,41.5 31.8,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 18.5,41.5 31.8,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any, any, any` → `any, any, any`

Yanks the iota third from the top of the stack to the top. \[0, 1, 2\] becomes \[1, 2, 0\].

## [[Rotation Gambit II|Rotation Gambit II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 58.2,18.5 71.5,41.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,41.5 58.2,18.5 71.5,41.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="49.2" y2="34.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.3,28.8 51.8,35.6 46.7,32.7" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any, any, any` → `any, any, any`

Yanks the top iota to the third position. \[0, 1, 2\] becomes \[2, 0, 1\].

## [[Gemini Decomposition|Gemini Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any` → `any, any`

Duplicates the top iota of the stack.

## [[Prospector's Gambit|Prospector's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any, any` → `any, any, any`

Copy the second-to-last iota of the stack to the top. \[0, 1\] becomes \[0, 1, 0\].

## [[Undertaker's Gambit|Undertaker's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,18.5 71.5,18.5 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:159;--dur:1.7s" points="45.0,18.5 71.5,18.5 58.2,41.5 45.0,18.5 18.5,18.5 31.8,41.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="18.5" x2="53.5" y2="18.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,18.5 53.5,21.4 53.5,15.6" fill="#fc77be"/><circle cx="45.0" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`any, any` → `any, any, any`

Copy the top iota of the stack, then put it under the second iota. \[0, 1\] becomes \[1, 0, 1\].

## [[Gemini Gambit|Gemini Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5 31.8,64.4 58.2,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5 18.5,41.5 45.0,41.5 31.8,64.4 58.2,64.4 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any, number` → `many`

Removes the number at the top of the stack, then copies the top iota of the stack that number of times. (A count of 2 results in two of the iota on the stack, not three.)

## [[Dioscuri Gambit|Dioscuri Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="55.2,32.0 75.7,32.0 65.5,14.3 55.2,32.0 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 34.8,32.0 55.2,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="55.2,32.0 75.7,32.0 65.5,14.3 55.2,32.0 45.0,14.3 34.8,32.0 24.5,14.3 14.3,32.0 34.8,32.0 55.2,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="32.0" x2="61.8" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="66.5,32.0 61.8,34.3 61.8,29.8" fill="#fc77be"/><circle cx="55.2" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`any, any` → `any, any, any, any`

Copy the top two iotas of the stack. \[0, 1\] becomes \[0, 1, 0, 1\].

## [[Flock's Reflection|Flock's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 39.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:3.6s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1 50.8,36.1 39.2,16.2 27.7,36.1 39.2,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`—` → `number`

Pushes the size of the stack as a number to the top of the stack. (For example, a stack of \[0, 1\] will become \[0, 1, 2\].)

## [[Fisherman's Gambit|Fisherman's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:132;--dur:1.6s" points="45.0,41.5 18.5,41.5 31.8,18.5 45.0,41.5 58.2,18.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="36.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.4,41.5 36.5,38.5 36.5,44.4" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`number` → `any`

Grabs the element in the stack indexed by the number and brings it to the top. If the number is negative, instead moves the top element of the stack down that many elements.

## [[Fisherman's Gambit II|Fisherman's Gambit II]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:132;--dur:1.6s" points="45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,18.5 18.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`number` → `any`

Like [[Fisherman's Gambit]], but instead of moving the iota, copies it.

## [[Bookkeeper's Gambit|Bookkeeper's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="14.3,14.3 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0 75.7,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:102;--dur:1.6s" points="14.3,14.3 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0 75.7,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="14.3" x2="17.6" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,24.1 15.6,21.1 19.5,18.9" fill="#fc77be"/><circle cx="14.3" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`many` → `many`

An infinite family of actions that keep or remove elements at the top of the stack based on the sequence of dips and lines.

Assuming that I draw a [[Bookkeeper's Gambit]] pattern left-to-right, the number of iotas the action will require is determined by the horizontal distance covered by the pattern. From deepest in the stack to shallowest, a flat line will keep the iota, whereas a triangle dipping down will remove it.

If my stack contains 0, 1, 2 from deepest to shallowest, drawing the first pattern opposite will give me 1, the second will give me 0, and the third will give me 0, 2 (the 0 at the bottom is left untouched).

## [[Swindler's Gambit|Swindler's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="31.8,18.5 45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:212;--dur:2.2s" points="31.8,18.5 45.0,41.5 71.5,41.5 58.2,18.5 45.0,41.5 31.8,64.4 18.5,41.5 45.0,41.5 58.2,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.8" y1="18.5" x2="36.0" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.0,31.1 33.5,27.3 38.5,24.4" fill="#fc77be"/><circle cx="31.8" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`many, number` → `many`

Rearranges the top elements of the stack based on the given numerical code, which is the index of the permutation wanted.

Although I can't pretend to know the mathematics behind calculating this permutation code, I have managed to dig up an extensive chart of them, enumerating all permutations of up to six elements.

If I wish to do further study, the key word is "Lehmer Code."

🔗 [Table of Codes](https://github.com/gamma-delta/HexMod/wiki/Table-of-Lehmer-Codes-for-Swindler's-Gambit)

---
### Patterns on this page

- [[Novice's Gambit]] · Pattern · Gambit
- [[Jester's Gambit]] · Pattern · Gambit
- [[Rotation Gambit]] · Pattern · Gambit
- [[Rotation Gambit II]] · Pattern · Gambit
- [[Gemini Decomposition]] · Pattern · Decomposition
- [[Prospector's Gambit]] · Pattern · Gambit
- [[Undertaker's Gambit]] · Pattern · Gambit
- [[Gemini Gambit]] · Pattern · Gambit
- [[Dioscuri Gambit]] · Pattern · Gambit
- [[Flock's Reflection]] · Pattern · Reflection
- [[Fisherman's Gambit]] · Pattern · Gambit
- [[Fisherman's Gambit II]] · Pattern · Gambit
- [[Bookkeeper's Gambit]] · Pattern · Gambit
- [[Swindler's Gambit]] · Pattern · Gambit

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/stackmanip) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
