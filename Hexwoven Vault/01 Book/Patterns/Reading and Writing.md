---
aliases: ["Reading and Writing"]
tags: [hexbook/entry, mod/hex-casting, category/patterns, topic/storage]
mod: "Hex Casting"
category: "Patterns"
entry_id: "hexcasting:patterns/readwrite"
---

# Reading and Writing

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[Hex Casting]]

This section deals with the storage of **[[Iotas|Iotas]]** in a more permanent medium. Nearly any iota can be stored to a suitable item, such as a [[Focus]] or [[Spellbook]], and read back later. Certain items, such as an [[Abacus]], can only be read from.

Iotas are usually read and written from the other hand, but it is also possible to read and write with an item when it is sitting on the ground as an item entity, or when in an item frame.

There may be other entities I can interact with in this way. For example, a [[Scrolls|Scroll]] hung on the wall can have its pattern read off of it.

However, it seems I am unable to save a reference to another player, only me. I suppose an entity reference is similar to the idea of a True Name; perhaps Nature is helping to keep our Names out of the hands of enemies. If I want a friend to have my Name I can make a [[Focus]] for them.

## [[Scribe's Reflection|Scribe's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,18.5 31.8,18.5 18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="45.0,41.5 71.5,41.5 58.2,18.5 31.8,18.5 18.5,41.5 31.8,64.4 58.2,64.4 71.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`—` → `any`

Copy the iota stored in the item in my other hand and add it to the [[The Stack|stack]].

## [[Scribe's Gambit|Scribe's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/><circle cx="5.3" cy="64.4" r="1.6"/><circle cx="31.8" cy="64.4" r="1.6"/><circle cx="58.2" cy="64.4" r="1.6"/><circle cx="84.7" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,41.5 71.5,41.5 58.2,64.4 31.8,64.4 18.5,41.5 31.8,18.5 58.2,18.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.0s" points="45.0,41.5 71.5,41.5 58.2,64.4 31.8,64.4 18.5,41.5 31.8,18.5 58.2,18.5 71.5,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="41.5" x2="53.5" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.6,41.5 53.5,44.4 53.5,38.5" fill="#fc77be"/><circle cx="45.0" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`any` → `—`

Remove the top iota from the stack, and save it into the item in my other hand.

## [[Chronicler's Purification|Chronicler's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="50.3" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.2,40.5 50.3,42.4 50.3,38.7" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`entity` → `any`

Like [[Scribe's Reflection]], but the iota is read out of an entity instead of my other hand.

## [[Chronicler's Gambit|Chronicler's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:233;--dur:3.9s" points="45.0,40.5 61.7,40.5 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="50.3" y2="40.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="54.2,40.5 50.3,42.4 50.3,38.7" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`entity, any` → `—`

Like [[Scribe's Reflection|Scribe's Gambit]], but the iota is written to an entity instead of my other hand.

Interestingly enough, it looks like I cannot write my own Name using this spell. I get a sense that I might be endangered if I could.

## [[Auditor's Reflection|Auditor's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="34.8,32.0 55.2,32.0 45.0,14.3 24.5,14.3 14.3,32.0 24.5,49.7 45.0,49.7 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="41.3" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,32.0 41.3,34.3 41.3,29.8" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `bool`

If the item in my other hand holds an iota I can read, returns True. Otherwise, returns False.

## [[Auditor's Purification|Auditor's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="59" viewBox="0 0 90.0 59.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="8.5" r="1.6"/><circle cx="20.7" cy="8.5" r="1.6"/><circle cx="32.8" cy="8.5" r="1.6"/><circle cx="45.0" cy="8.5" r="1.6"/><circle cx="57.2" cy="8.5" r="1.6"/><circle cx="69.3" cy="8.5" r="1.6"/><circle cx="81.5" cy="8.5" r="1.6"/><circle cx="2.4" cy="19.0" r="1.6"/><circle cx="14.6" cy="19.0" r="1.6"/><circle cx="26.8" cy="19.0" r="1.6"/><circle cx="38.9" cy="19.0" r="1.6"/><circle cx="51.1" cy="19.0" r="1.6"/><circle cx="63.2" cy="19.0" r="1.6"/><circle cx="75.4" cy="19.0" r="1.6"/><circle cx="87.6" cy="19.0" r="1.6"/><circle cx="8.5" cy="29.6" r="1.6"/><circle cx="20.7" cy="29.6" r="1.6"/><circle cx="32.8" cy="29.6" r="1.6"/><circle cx="45.0" cy="29.6" r="1.6"/><circle cx="57.2" cy="29.6" r="1.6"/><circle cx="69.3" cy="29.6" r="1.6"/><circle cx="81.5" cy="29.6" r="1.6"/><circle cx="2.4" cy="40.1" r="1.6"/><circle cx="14.6" cy="40.1" r="1.6"/><circle cx="26.8" cy="40.1" r="1.6"/><circle cx="38.9" cy="40.1" r="1.6"/><circle cx="51.1" cy="40.1" r="1.6"/><circle cx="63.2" cy="40.1" r="1.6"/><circle cx="75.4" cy="40.1" r="1.6"/><circle cx="87.6" cy="40.1" r="1.6"/><circle cx="8.5" cy="50.6" r="1.6"/><circle cx="20.7" cy="50.6" r="1.6"/><circle cx="32.8" cy="50.6" r="1.6"/><circle cx="45.0" cy="50.6" r="1.6"/><circle cx="57.2" cy="50.6" r="1.6"/><circle cx="69.3" cy="50.6" r="1.6"/><circle cx="81.5" cy="50.6" r="1.6"/></g><polyline class="hexpat-base" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,19.0 45.0,8.5 32.8,8.5 20.7,8.5 14.6,19.0 8.5,29.6 14.6,40.1 20.7,50.6 32.8,50.6 45.0,50.6 51.1,40.1 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:195;--dur:4.5s" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,19.0 45.0,8.5 32.8,8.5 20.7,8.5 14.6,19.0 8.5,29.6 14.6,40.1 20.7,50.6 32.8,50.6 45.0,50.6 51.1,40.1 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="32.8" y1="29.6" x2="36.7" y2="29.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.5,29.6 36.7,30.9 36.7,28.2" fill="#fc77be"/><circle cx="32.8" cy="29.6" r="2.6" fill="#fc77be"/></svg>

`entity` → `bool`

Like [[Auditor's Reflection]], but the readability of an entity is checked instead of my other hand.

## [[Assessor's Reflection|Assessor's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,32.0 55.2,32.0 45.0,49.7 24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:164;--dur:2.2s" points="34.8,32.0 55.2,32.0 45.0,49.7 24.5,49.7 14.3,32.0 24.5,14.3 45.0,14.3 55.2,32.0 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="32.0" x2="41.3" y2="32.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,32.0 41.3,34.3 41.3,29.8" fill="#fc77be"/><circle cx="34.8" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `bool`

If I could save an iota into the item in my other hand, returns True. Otherwise, returns False.

## [[Assessor's Purification|Assessor's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="59" viewBox="0 0 90.0 59.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="8.5" cy="8.5" r="1.6"/><circle cx="20.7" cy="8.5" r="1.6"/><circle cx="32.8" cy="8.5" r="1.6"/><circle cx="45.0" cy="8.5" r="1.6"/><circle cx="57.2" cy="8.5" r="1.6"/><circle cx="69.3" cy="8.5" r="1.6"/><circle cx="81.5" cy="8.5" r="1.6"/><circle cx="2.4" cy="19.0" r="1.6"/><circle cx="14.6" cy="19.0" r="1.6"/><circle cx="26.8" cy="19.0" r="1.6"/><circle cx="38.9" cy="19.0" r="1.6"/><circle cx="51.1" cy="19.0" r="1.6"/><circle cx="63.2" cy="19.0" r="1.6"/><circle cx="75.4" cy="19.0" r="1.6"/><circle cx="87.6" cy="19.0" r="1.6"/><circle cx="8.5" cy="29.6" r="1.6"/><circle cx="20.7" cy="29.6" r="1.6"/><circle cx="32.8" cy="29.6" r="1.6"/><circle cx="45.0" cy="29.6" r="1.6"/><circle cx="57.2" cy="29.6" r="1.6"/><circle cx="69.3" cy="29.6" r="1.6"/><circle cx="81.5" cy="29.6" r="1.6"/><circle cx="2.4" cy="40.1" r="1.6"/><circle cx="14.6" cy="40.1" r="1.6"/><circle cx="26.8" cy="40.1" r="1.6"/><circle cx="38.9" cy="40.1" r="1.6"/><circle cx="51.1" cy="40.1" r="1.6"/><circle cx="63.2" cy="40.1" r="1.6"/><circle cx="75.4" cy="40.1" r="1.6"/><circle cx="87.6" cy="40.1" r="1.6"/><circle cx="8.5" cy="50.6" r="1.6"/><circle cx="20.7" cy="50.6" r="1.6"/><circle cx="32.8" cy="50.6" r="1.6"/><circle cx="45.0" cy="50.6" r="1.6"/><circle cx="57.2" cy="50.6" r="1.6"/><circle cx="69.3" cy="50.6" r="1.6"/><circle cx="81.5" cy="50.6" r="1.6"/></g><polyline class="hexpat-base" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,40.1 45.0,50.6 32.8,50.6 20.7,50.6 14.6,40.1 8.5,29.6 14.6,19.0 20.7,8.5 32.8,8.5 45.0,8.5 51.1,19.0 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:195;--dur:4.5s" points="32.8,29.6 45.0,29.6 57.2,29.6 51.1,40.1 45.0,50.6 32.8,50.6 20.7,50.6 14.6,40.1 8.5,29.6 14.6,19.0 20.7,8.5 32.8,8.5 45.0,8.5 51.1,19.0 57.2,29.6 69.3,29.6 81.5,29.6" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="32.8" y1="29.6" x2="36.7" y2="29.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="39.5,29.6 36.7,30.9 36.7,28.2" fill="#fc77be"/><circle cx="32.8" cy="29.6" r="2.6" fill="#fc77be"/></svg>

`entity` → `bool`

Like [[Assessor's Reflection]], but the writability of an entity is checked instead of my other hand.

### The Ravenmind

Items are not the only places I can store information, however. I am also able to store that information in the **[[Media|media]]** of the **Hex** itself, much like the stack, but separate. Texts refer to this as the **ravenmind**. It holds a single iota, much like a [[Focus]], and begins with **Null** like the same. It is preserved between iterations of [[Thoth's Gambit]], but only lasts as long as the **Hex** it's a part of. Once I stop casting, the value will be lost.

## [[Huginn's Gambit|Huginn's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="15.8" cy="15.8" r="1.6"/><circle cx="38.3" cy="15.8" r="1.6"/><circle cx="60.8" cy="15.8" r="1.6"/><circle cx="83.3" cy="15.8" r="1.6"/><circle cx="4.5" cy="35.3" r="1.6"/><circle cx="27.0" cy="35.3" r="1.6"/><circle cx="49.5" cy="35.3" r="1.6"/><circle cx="72.0" cy="35.3" r="1.6"/><circle cx="15.8" cy="54.7" r="1.6"/><circle cx="38.3" cy="54.7" r="1.6"/><circle cx="60.8" cy="54.7" r="1.6"/><circle cx="83.3" cy="54.7" r="1.6"/><circle cx="4.5" cy="74.2" r="1.6"/><circle cx="27.0" cy="74.2" r="1.6"/><circle cx="49.5" cy="74.2" r="1.6"/><circle cx="72.0" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="72.0,74.2 60.8,54.7 72.0,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3 38.3,54.7 60.8,54.7 49.5,35.3 38.3,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="72.0,74.2 60.8,54.7 72.0,35.3 60.8,15.8 38.3,15.8 15.8,15.8 27.0,35.3 38.3,54.7 60.8,54.7 49.5,35.3 38.3,54.7 27.0,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="72.0" y1="74.2" x2="68.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="65.8,63.5 70.6,66.8 66.3,69.2" fill="#fc77be"/><circle cx="72.0" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`any` → `—`

Removes the top iota from the stack, and saves it to my **ravenmind**, storing it there until I stop casting the **Hex**.

## [[Muninn's Reflection|Muninn's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="88" height="90" viewBox="0 0 87.8 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="72.0" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="60.8" cy="35.3" r="1.6"/><circle cx="83.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="72.0" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/><circle cx="60.8" cy="74.2" r="1.6"/><circle cx="83.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 72.0,15.8 60.8,35.3 49.5,54.7 27.0,54.7 38.3,35.3 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:248;--dur:3.1s" points="15.8,74.2 27.0,54.7 15.8,35.3 27.0,15.8 49.5,15.8 72.0,15.8 60.8,35.3 49.5,54.7 27.0,54.7 38.3,35.3 49.5,54.7 60.8,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`—` → `any`

Copy the iota out of my **ravenmind**, which I likely just wrote with [[Huginn's Gambit]].

---
### Patterns on this page

- [[Scribe's Reflection]] · Pattern · Reflection
- [[Scribe's Gambit]] · Spell · Gambit
- [[Chronicler's Purification]] · Pattern · Purification
- [[Chronicler's Gambit]] · Spell · Gambit
- [[Auditor's Reflection]] · Pattern · Reflection
- [[Auditor's Purification]] · Pattern · Purification
- [[Assessor's Reflection]] · Pattern · Reflection
- [[Assessor's Purification]] · Pattern · Purification
- [[Huginn's Gambit]] · Pattern · Gambit
- [[Muninn's Reflection]] · Pattern · Reflection

**Sources:** 🌐 [Official online docs](https://hexcasting.hexxy.media/v/0.11.4/1.0/en_us/#patterns/readwrite) · 🧩 [Hex Casting on Modrinth](https://modrinth.com/mod/nTW3yKrm)
