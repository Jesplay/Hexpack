---
aliases: ["Reading/Writing Blocks"]
tags: [hexbook/entry, mod/ioticblocks, category/patterns, topic/blocks, topic/storage]
mod: "IoticBlocks"
category: "Patterns"
entry_id: "hexcasting:patterns/readwrite_blocks"
---

# Reading/Writing Blocks

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[IoticBlocks]]

## [[Chronicler's Purification|Chronicler's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="50.3" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.2,40.5 50.3,42.4 50.3,38.7" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`vec` → `any`

Like [[Scribe's Reflection]], but the [[Iotas|iota]] is read out of a block instead of my other hand.

## [[Chronicler's Gambit|Chronicler's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="50.3" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.2,40.5 50.3,42.4 50.3,38.7" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`vec, any` → `—`

Like [[Scribe's Gambit]], but the iota is written to a block instead of my other hand.

Interestingly enough, it looks like I cannot write my own Name using this spell. I get a sense that I might be endangered if I could.

## [[Tachygrapher's Distillation|Tachygrapher's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 36.7,26.1 45.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:5.0s" points="45.0,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 36.7,26.1 45.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="50.3" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.2,40.5 50.3,42.4 50.3,38.7" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`entity | vec, num` → `any`

Read a list iota out of an entity or block, then push the iota at the specified index to the [[The Stack|stack]].

It seems this and the next pattern may be useful for handling large amounts of data without causing a [[Mishaps|Too Many Iotas]] [[Mishaps|mishap]].

## [[Tachygrapher's Gambit|Tachygrapher's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 55.2,49.7 45.0,67.5 24.5,67.5 14.3,49.7 24.5,32.0 45.0,32.0 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:286;--dur:3.9s" points="34.8,49.7 55.2,49.7 45.0,67.5 24.5,67.5 14.3,49.7 24.5,32.0 45.0,32.0 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="41.3" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,49.7 41.3,52.0 41.3,47.5" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`entity | vec, num, any` → `—`

Read a list iota out of an entity or block, then set the element at the specified index of that list to the given iota, then write the resulting list back to the entity or block.

## [[Auditor's Purification|Auditor's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="59" viewBox="0 0 90.0 59.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="8.5" r="1.6"/><circle cx="20.7" cy="8.5" r="1.6"/><circle cx="32.8" cy="8.5" r="1.6"/><circle cx="45.0" cy="8.5" r="1.6"/><circle cx="57.2" cy="8.5" r="1.6"/><circle cx="69.3" cy="8.5" r="1.6"/><circle cx="81.5" cy="8.5" r="1.6"/><circle cx="2.4" cy="19.0" r="1.6"/><circle cx="14.6" cy="19.0" r="1.6"/><circle cx="26.8" cy="19.0" r="1.6"/><circle cx="38.9" cy="19.0" r="1.6"/><circle cx="51.1" cy="19.0" r="1.6"/><circle cx="63.2" cy="19.0" r="1.6"/><circle cx="75.4" cy="19.0" r="1.6"/><circle cx="87.6" cy="19.0" r="1.6"/><circle cx="8.5" cy="29.6" r="1.6"/><circle cx="20.7" cy="29.6" r="1.6"/><circle cx="32.8" cy="29.6" r="1.6"/><circle cx="45.0" cy="29.6" r="1.6"/><circle cx="57.2" cy="29.6" r="1.6"/><circle cx="69.3" cy="29.6" r="1.6"/><circle cx="81.5" cy="29.6" r="1.6"/><circle cx="2.4" cy="40.1" r="1.6"/><circle cx="14.6" cy="40.1" r="1.6"/><circle cx="26.8" cy="40.1" r="1.6"/><circle cx="38.9" cy="40.1" r="1.6"/><circle cx="51.1" cy="40.1" r="1.6"/><circle cx="63.2" cy="40.1" r="1.6"/><circle cx="75.4" cy="40.1" r="1.6"/><circle cx="87.6" cy="40.1" r="1.6"/><circle cx="8.5" cy="50.6" r="1.6"/><circle cx="20.7" cy="50.6" r="1.6"/><circle cx="32.8" cy="50.6" r="1.6"/><circle cx="45.0" cy="50.6" r="1.6"/><circle cx="57.2" cy="50.6" r="1.6"/><circle cx="69.3" cy="50.6" r="1.6"/><circle cx="81.5" cy="50.6" r="1.6"/></g><polyline class="hexpat-base" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,19.0 45.0,8.5 32.8,8.5 20.7,8.5 14.6,19.0 8.5,29.6 14.6,40.1 20.7,50.6 32.8,50.6 45.0,50.6 51.1,40.1 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:195;--dur:4.5s" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,19.0 45.0,8.5 32.8,8.5 20.7,8.5 14.6,19.0 8.5,29.6 14.6,40.1 20.7,50.6 32.8,50.6 45.0,50.6 51.1,40.1 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="32.8" y1="29.6" x2="36.7" y2="29.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.5,29.6 36.7,30.9 36.7,28.2" fill="#fc77be"/><circle cx="32.8" cy="29.6" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Like [[Auditor's Reflection]], but the readability of a block is checked instead of my other hand.

## [[Assessor's Purification|Assessor's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="59" viewBox="0 0 90.0 59.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="8.5" r="1.6"/><circle cx="20.7" cy="8.5" r="1.6"/><circle cx="32.8" cy="8.5" r="1.6"/><circle cx="45.0" cy="8.5" r="1.6"/><circle cx="57.2" cy="8.5" r="1.6"/><circle cx="69.3" cy="8.5" r="1.6"/><circle cx="81.5" cy="8.5" r="1.6"/><circle cx="2.4" cy="19.0" r="1.6"/><circle cx="14.6" cy="19.0" r="1.6"/><circle cx="26.8" cy="19.0" r="1.6"/><circle cx="38.9" cy="19.0" r="1.6"/><circle cx="51.1" cy="19.0" r="1.6"/><circle cx="63.2" cy="19.0" r="1.6"/><circle cx="75.4" cy="19.0" r="1.6"/><circle cx="87.6" cy="19.0" r="1.6"/><circle cx="8.5" cy="29.6" r="1.6"/><circle cx="20.7" cy="29.6" r="1.6"/><circle cx="32.8" cy="29.6" r="1.6"/><circle cx="45.0" cy="29.6" r="1.6"/><circle cx="57.2" cy="29.6" r="1.6"/><circle cx="69.3" cy="29.6" r="1.6"/><circle cx="81.5" cy="29.6" r="1.6"/><circle cx="2.4" cy="40.1" r="1.6"/><circle cx="14.6" cy="40.1" r="1.6"/><circle cx="26.8" cy="40.1" r="1.6"/><circle cx="38.9" cy="40.1" r="1.6"/><circle cx="51.1" cy="40.1" r="1.6"/><circle cx="63.2" cy="40.1" r="1.6"/><circle cx="75.4" cy="40.1" r="1.6"/><circle cx="87.6" cy="40.1" r="1.6"/><circle cx="8.5" cy="50.6" r="1.6"/><circle cx="20.7" cy="50.6" r="1.6"/><circle cx="32.8" cy="50.6" r="1.6"/><circle cx="45.0" cy="50.6" r="1.6"/><circle cx="57.2" cy="50.6" r="1.6"/><circle cx="69.3" cy="50.6" r="1.6"/><circle cx="81.5" cy="50.6" r="1.6"/></g><polyline class="hexpat-base" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,40.1 45.0,50.6 32.8,50.6 20.7,50.6 14.6,40.1 8.5,29.6 14.6,19.0 20.7,8.5 32.8,8.5 45.0,8.5 51.1,19.0 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:195;--dur:4.5s" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,40.1 45.0,50.6 32.8,50.6 20.7,50.6 14.6,40.1 8.5,29.6 14.6,19.0 20.7,8.5 32.8,8.5 45.0,8.5 51.1,19.0 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="32.8" y1="29.6" x2="36.7" y2="29.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.5,29.6 36.7,30.9 36.7,28.2" fill="#fc77be"/><circle cx="32.8" cy="29.6" r="2.6" fill="#fc77be"/></svg>

`vec` → `bool`

Like [[Assessor's Reflection]], but the writability of a block is checked instead of my other hand.

---
### Patterns on this page

- [[Chronicler's Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Chronicler's Gambit]] · Spell · Gambit *(from Hex Casting)*
- [[Tachygrapher's Distillation]] · Pattern · Distillation
- [[Tachygrapher's Gambit]] · Spell · Gambit
- [[Auditor's Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Assessor's Purification]] · Pattern · Purification *(from Hex Casting)*

**Sources:** 🌐 [Official online docs](https://ioticblocks.hexxy.media/v/1.0.2+1.20.1/1.1/en_us/#patterns/readwrite_blocks) · 🧩 [IoticBlocks on Modrinth](https://modrinth.com/mod/u3PPsXzt)
