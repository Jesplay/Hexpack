---
aliases: ["Jump Manipulation"]
tags: [hexbook/entry, mod/caduceus, category/patterns, topic/control-flow]
mod: "Caduceus"
category: "Patterns"
entry_id: "hexcasting:caduceus/patterns/jump_manipulation"
---

# Jump Manipulation

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Caduceus]]

Just like my **Hexes** interact with the [[The Stack|stack]], Nature uses a [[The Stack|stack]] of its own (made up of "frames") when casting my **Hexes**. Jump [[Iotas|iotas]] contain a snapshot of this stack at a particular moment in time, and I can use the patterns in this section to deconstruct and rearrange that snapshot.
Unfortunately, despite their similarity to Jump iotas, it seems these patterns do not work on Call iotas.

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`jump, jump` → `jump`

Remove the Jump iota at the top of the stack, then add all its frames to the bottom of the Jump iota at the top of the stack.

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`jump` → `num`

Remove the Jump iota at the top of the stack, then push the number of frames within it to the stack.

## [[Flock's Disintegration|Flock's Disintegration]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 62.3,56.1 73.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="36.1" x2="70.2" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="67.5,25.1 72.4,28.5 68.0,31.0" fill="#fc77be"/><circle cx="73.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`jump` → `many`

Remove the Jump iota at the top of the stack, then, for each of its frames, push a new Jump iota containing only that frame to the stack.

## [[Speaker's Distillation|Speaker's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="24.5,14.3 34.8,32.0 14.3,32.0 24.5,14.3 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="24.5,14.3 34.8,32.0 14.3,32.0 24.5,14.3 45.0,14.3 65.5,14.3 75.7,32.0 55.2,32.0 65.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="14.3" x2="27.8" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,24.1 25.9,21.1 29.8,18.9" fill="#fc77be"/><circle cx="24.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`jump, jump` → `jump`

Remove the Jump iota at the top of the stack, then add its top frame to the top of the Jump iota at the top of the stack.

## [[Speaker's Decomposition|Speaker's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="46" viewBox="0 0 90.0 46.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/></g><polyline class="hexpat-base" points="65.5,14.3 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 24.5,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="65.5,14.3 55.2,32.0 75.7,32.0 65.5,14.3 45.0,14.3 24.5,14.3 14.3,32.0 34.8,32.0 24.5,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="14.3" x2="62.2" y2="20.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,24.1 60.2,18.9 64.1,21.1" fill="#fc77be"/><circle cx="65.5" cy="14.3" r="2.6" fill="#fc77be"/></svg>

`jump` → `jump, jump`

Remove the top frame from the Jump iota at the top of the stack, then push a new Jump iota containing only that frame to the stack.

### Jump Tags

The [[Reading and Writing|ravenmind]] is not the only place that I can store information within a **Hex**. In fact, I am able to hide iotas within the very concept of evaluation. I call this technique **jump tags**; some other texts refer to it as "continuation marks".

There is one notable limitation of **jump tags**, however: they can only hold a single iota at a time. Unlike the [[Reading and Writing|ravenmind]], even lists are too much.

## [[Ewer's Reflection|Ewer's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 38.3,74.2 27.0,54.7 38.3,35.3 15.8,35.3 27.0,54.7 49.5,54.7 38.3,35.3 27.0,15.8 49.5,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="15.8,74.2 38.3,74.2 27.0,54.7 38.3,35.3 15.8,35.3 27.0,54.7 49.5,54.7 38.3,35.3 27.0,15.8 49.5,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="23.0" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.1,74.2 23.0,76.7 23.0,71.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`—` → `any`

Copy the iota stored in the **jump tag** of the current evaluation and add it to the stack.

## [[Ewer's Purification|Ewer's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 38.3,74.2 27.0,54.7 49.5,54.7 38.3,35.3 27.0,54.7 15.8,35.3 38.3,35.3 27.0,15.8 49.5,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="15.8,74.2 38.3,74.2 27.0,54.7 49.5,54.7 38.3,35.3 27.0,54.7 15.8,35.3 38.3,35.3 27.0,15.8 49.5,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="23.0" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.1,74.2 23.0,76.7 23.0,71.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`jump` → `any`

Like [[Ewer's Reflection]], but the iota is read out of the **jump tag** of the top frame in a Jump iota.

## [[Ewer's Gambit|Ewer's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="65" height="90" viewBox="0 0 65.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="49.5,74.2 27.0,74.2 38.3,54.7 27.0,35.3 49.5,35.3 38.3,54.7 15.8,54.7 27.0,35.3 38.3,15.8 15.8,15.8" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:203;--dur:2.5s" points="49.5,74.2 27.0,74.2 38.3,54.7 27.0,35.3 49.5,35.3 38.3,54.7 15.8,54.7 27.0,35.3 38.3,15.8 15.8,15.8" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="49.5" y1="74.2" x2="42.3" y2="74.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.1,74.2 42.3,71.8 42.3,76.7" fill="#fc77be"/><circle cx="49.5" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`any` → `—`

Remove the top iota from the stack, and write it to the **jump tag** of the current evaluation.
As with [[Chronicler's Gambit]], I cannot write my own Name using this spell.

I should be careful if using [[Ewer's Reflection]] or [[Ewer's Gambit]] at the very end of an invocation. The tag that it reads or writes may not be the one I expect.

---
### Patterns on this page

- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Flock's Disintegration]] · Pattern · Disintegration *(from Hex Casting)*
- [[Speaker's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Speaker's Decomposition]] · Pattern · Decomposition *(from Hex Casting)*
- [[Ewer's Reflection]] · Pattern · Reflection
- [[Ewer's Purification]] · Pattern · Purification
- [[Ewer's Gambit]] · Pattern · Gambit

**Sources:** 🌐 [Official online docs](https://caduceus.hexxy.media/v/0.3.0+1.20.1/1.0/en_us/#caduceus/patterns/jump_manipulation) · 🧩 [Caduceus on Modrinth](https://modrinth.com/mod/Q6Hj9z4p)
