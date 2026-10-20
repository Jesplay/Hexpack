---
aliases: ["Strings"]
tags: [hexbook/entry, mod/moreiotas, category/patterns, topic/text]
mod: "MoreIotas"
category: "Patterns"
entry_id: "hexcasting:patterns/strings"
---

# Strings

> **Category:** [[Patterns (Category)|Patterns]]  
> **Source mod:** [[MoreIotas]]

## [[Additive Distillation|Additive Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="69" height="90" viewBox="0 0 69.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:144;--dur:1.6s" points="20.1,69.9 34.5,45.0 48.8,20.1 20.1,20.1 34.5,45.0 48.8,69.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.1" y1="69.9" x2="24.7" y2="61.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.0,56.2 27.4,63.5 22.0,60.3" fill="#fc77be"/><circle cx="20.1" cy="69.9" r="2.6" fill="#fc77be"/></svg>

`str, str` → `str`

Removes the top two strings from the [[The Stack|stack]] and combines them together.

## [[Separation Distillation|Separation Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1 39.2,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="50.8,36.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1 39.2,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="58.2" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="63.5,36.1 58.2,38.7 58.2,33.6" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`str, str` → `[str]`

Removes the top two strings from the stack and splits the first string, using the second string as the delimiter and returning as a list.

## [[Input Purification|Input Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:138;--dur:1.7s" points="50.8,36.1 73.8,36.1 62.3,16.2 39.2,16.2 16.2,16.2 27.7,36.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="58.2" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="63.5,36.1 58.2,38.7 58.2,33.6" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`str` → `num | null`

Removes a string from the top of the stack and returns the number that string parses as (or null if it doesn't parse).

## [[Locator's Distillation|Locator's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="83" height="90" viewBox="0 0 83.3 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="20.1" cy="20.1" r="1.6"/><circle cx="48.8" cy="20.1" r="1.6"/><circle cx="77.6" cy="20.1" r="1.6"/><circle cx="5.7" cy="45.0" r="1.6"/><circle cx="34.5" cy="45.0" r="1.6"/><circle cx="63.2" cy="45.0" r="1.6"/><circle cx="20.1" cy="69.9" r="1.6"/><circle cx="48.8" cy="69.9" r="1.6"/><circle cx="77.6" cy="69.9" r="1.6"/></g><polyline class="hexpat-base" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:201;--dur:2.0s" points="34.5,45.0 63.2,45.0 48.8,69.9 20.1,69.9 34.5,45.0 20.1,20.1 48.8,20.1 63.2,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.5" y1="45.0" x2="43.7" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="50.3,45.0 43.7,48.2 43.7,41.8" fill="#fc77be"/><circle cx="34.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`str, str` → `num`

Removes two strings from the top of the stack, and returns the first index of the first string where the second string is a substring.

## [[Selection Exaltation|Selection Exaltation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:231;--dur:2.8s" points="50.8,36.1 39.2,16.2 16.2,16.2 27.7,36.1 16.2,56.1 39.2,56.1 50.8,36.1 62.3,16.2 73.8,36.1 62.3,56.1 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="50.8" y1="36.1" x2="47.1" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="44.4,25.1 49.3,28.5 44.9,31.0" fill="#fc77be"/><circle cx="50.8" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`str, num, num` → `str`

Removes a string and two positive integers from the top of the stack, and returns the substring from the first to the second (inclusive first, exclusive second).

## [[Length Purification|Length Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="54" height="90" viewBox="0 0 54.0 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.5" cy="15.8" r="1.6"/><circle cx="27.0" cy="15.8" r="1.6"/><circle cx="49.5" cy="15.8" r="1.6"/><circle cx="15.8" cy="35.3" r="1.6"/><circle cx="38.3" cy="35.3" r="1.6"/><circle cx="4.5" cy="54.7" r="1.6"/><circle cx="27.0" cy="54.7" r="1.6"/><circle cx="49.5" cy="54.7" r="1.6"/><circle cx="15.8" cy="74.2" r="1.6"/><circle cx="38.3" cy="74.2" r="1.6"/></g><polyline class="hexpat-base" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:135;--dur:1.7s" points="15.8,74.2 27.0,54.7 38.3,35.3 27.0,15.8 15.8,35.3 27.0,54.7 38.3,74.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="15.8" y1="74.2" x2="19.4" y2="68.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.9,63.5 21.5,69.2 17.2,66.8" fill="#fc77be"/><circle cx="15.8" cy="74.2" r="2.6" fill="#fc77be"/></svg>

`str` → `num`

Removes a string and returns its length.

## [[Blank Reflection|Blank Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:123;--dur:1.7s" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="17.6" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,41.8 15.6,38.8 19.5,36.6" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `str`

Add an empty string to the stack, useful for combining with other strings.

## [[Spacing Reflection|Spacing Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,32.0 55.2,32.0 34.8,32.0 14.3,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="17.6" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,41.8 15.6,38.8 19.5,36.6" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `str`

Add a blank space string to the stack, useful for combining with other strings.

## [[Comma Reflection|Comma Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="70" viewBox="0 0 90.0 70.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="6.2" cy="21.7" r="1.6"/><circle cx="37.2" cy="21.7" r="1.6"/><circle cx="68.3" cy="21.7" r="1.6"/><circle cx="21.7" cy="48.6" r="1.6"/><circle cx="52.8" cy="48.6" r="1.6"/><circle cx="83.8" cy="48.6" r="1.6"/></g><polyline class="hexpat-base" points="21.7,48.6 52.8,48.6 68.3,21.7 37.2,21.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:93;--dur:1.6s" points="21.7,48.6 52.8,48.6 68.3,21.7 37.2,21.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="21.7" y1="48.6" x2="31.7" y2="48.6" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="38.8,48.6 31.7,52.0 31.7,45.2" fill="#fc77be"/><circle cx="21.7" cy="48.6" r="2.6" fill="#fc77be"/></svg>

`—` → `str`

Add a comma string to the stack, useful for combining with other strings.

## [[Breaking Reflection|Breaking Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,36.1 39.2,36.1 62.3,36.1 50.8,16.2 39.2,36.1 27.7,56.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:162;--dur:2.0s" points="16.2,36.1 39.2,36.1 62.3,36.1 50.8,16.2 39.2,36.1 27.7,56.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="36.1" x2="23.5" y2="36.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.8,36.1 23.5,38.7 23.5,33.6" fill="#fc77be"/><circle cx="16.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`—` → `str`

Add a string to the stack representing a break between lines, useful for combining with other strings.

## [[Whisper Reflection|Whisper Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="5.3" cy="18.5" r="1.6"/><circle cx="31.8" cy="18.5" r="1.6"/><circle cx="58.2" cy="18.5" r="1.6"/><circle cx="84.7" cy="18.5" r="1.6"/><circle cx="18.5" cy="41.5" r="1.6"/><circle cx="45.0" cy="41.5" r="1.6"/><circle cx="71.5" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:132;--dur:1.6s" points="18.5,41.5 45.0,41.5 71.5,41.5 58.2,18.5 31.8,18.5 45.0,41.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="41.5" x2="27.0" y2="41.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,41.5 27.0,44.4 27.0,38.5" fill="#fc77be"/><circle cx="18.5" cy="41.5" r="2.6" fill="#fc77be"/></svg>

`—` → `str`

Adds the last message the caster sent to the stack as a string.

## [[Listener's Reflection|Listener's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="60" viewBox="0 0 90.0 60.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/></g><polyline class="hexpat-base" points="18.5,18.5 45.0,18.5 71.5,18.5 58.2,41.5 31.8,41.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:132;--dur:1.6s" points="18.5,18.5 45.0,18.5 71.5,18.5 58.2,41.5 31.8,41.5 45.0,18.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="18.5" y1="18.5" x2="27.0" y2="18.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="33.1,18.5 27.0,21.4 27.0,15.6" fill="#fc77be"/><circle cx="18.5" cy="18.5" r="2.6" fill="#fc77be"/></svg>

`—` → `str`

Adds the last message anyone sent to the stack as a string.

## [[Sifter's Gambit|Sifter's Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,16.2 27.7,36.1 50.8,36.1 73.8,36.1 62.3,16.2 39.2,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:138;--dur:1.7s" points="16.2,16.2 27.7,36.1 50.8,36.1 73.8,36.1 62.3,16.2 39.2,16.2 50.8,36.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="16.2" x2="19.8" y2="22.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,27.1 17.6,23.8 22.0,21.3" fill="#fc77be"/><circle cx="16.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`str | null` → `—`

Accepts a string; All future chat messages starting with that string won't be seen by others, and only messages prefixed with that string can be read by [[Whisper Reflection]].

## [[Sifter's Reflection|Sifter's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,36.1 27.7,16.2 50.8,16.2 73.8,16.2 62.3,36.1 39.2,36.1 50.8,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:138;--dur:1.7s" points="16.2,36.1 27.7,16.2 50.8,16.2 73.8,16.2 62.3,36.1 39.2,36.1 50.8,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="36.1" x2="19.8" y2="29.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,25.1 22.0,31.0 17.6,28.5" fill="#fc77be"/><circle cx="16.2" cy="36.1" r="2.6" fill="#fc77be"/></svg>

`—` → `str | null`

Returns the last string you passed to [[Sifter's Gambit]].

## [[Reader's Purification|Reader's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7 65.5,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 55.2,49.7 65.5,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="55.2" y1="49.7" x2="61.8" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="66.5,49.7 61.8,52.0 61.8,47.5" fill="#fc77be"/><circle cx="55.2" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec` → `str | [str]`

Removes a vector from the stack. If that vector is pointing at a sign or lectern, it returns that sign or lectern's text to the stack. Otherwise, it returns null.

## [[Write|Write]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 14.3,49.7 24.5,32.0 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 34.8,49.7 24.5,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:184;--dur:2.5s" points="34.8,49.7 14.3,49.7 24.5,32.0 34.8,14.3 55.2,14.3 75.7,14.3 65.5,32.0 55.2,49.7 34.8,49.7 24.5,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="28.2" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.5,49.7 28.2,47.5 28.2,52.0" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec, str | [str]` → `—`

Removes a vector and a string from the stack. If that vector is pointing at a sign or lectern, it writes that string to that sign or lectern. Costs a hundredth of an [[Amethyst|Amethyst Dust]].

## [[Scrivener's Purification|Scrivener's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,49.7 55.2,49.7 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 14.3,14.3 24.5,32.0 34.8,49.7 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="41.3" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="46.0,49.7 41.3,52.0 41.3,47.5" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`any` → `str`

Converts the [[Iotas|iota]] on top of the stack into a string.

## [[Patternmaster's Purification|Patternmaster's Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="34.8,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="34.8,49.7 24.5,32.0 14.3,14.3 34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 55.2,49.7 34.8,49.7 45.0,32.0 55.2,14.3" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="49.7" x2="31.5" y2="44.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="29.1,40.0 33.4,43.0 29.6,45.2" fill="#fc77be"/><circle cx="34.8" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`pattern` → `str | null`

Converts the pattern on top of the stack into the string for that pattern's name. If there is no action associated with that pattern, or if the associated pattern is a Great Spell, it returns null.

## [[Moniker Purification|Moniker Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="70.0,26.1 78.3,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,40.5 11.7,40.5 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:4.2s" points="70.0,26.1 78.3,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,40.5 11.7,40.5 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 61.7,11.7 45.0,11.7 28.3,11.7 20.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="70.0" y1="26.1" x2="72.7" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="74.6,34.0 71.1,31.6 74.3,29.8" fill="#fc77be"/><circle cx="70.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`entity` → `str`

Takes an entity and returns that entity's name. If it is an item entity, return the item's name.

## [[Name|Name]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="20.0,26.1 11.7,40.5 28.3,40.5 36.7,26.1 53.3,26.1 61.7,40.5 78.3,40.5 70.0,26.1 61.7,40.5 45.0,40.5 28.3,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:250;--dur:4.2s" points="20.0,26.1 11.7,40.5 28.3,40.5 36.7,26.1 53.3,26.1 61.7,40.5 78.3,40.5 70.0,26.1 61.7,40.5 45.0,40.5 28.3,40.5 20.0,26.1 28.3,11.7 45.0,11.7 61.7,11.7 70.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.0" y1="26.1" x2="17.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="15.4,34.0 15.7,29.8 18.9,31.6" fill="#fc77be"/><circle cx="20.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`str, entity` → `—`

Takes a string and an entity, and sets that entities name to the given string. If the entity is an item entity, also set the item's name. Costs a hundredth of an [[Amethyst|Amethyst Dust]].

## [[Case Distillation|Case Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/><circle cx="14.3" cy="67.5" r="1.6"/><circle cx="34.8" cy="67.5" r="1.6"/><circle cx="55.2" cy="67.5" r="1.6"/><circle cx="75.7" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="34.8,67.5 14.3,67.5 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,67.5 55.2,67.5 34.8,67.5 45.0,49.7 55.2,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="34.8,67.5 14.3,67.5 24.5,49.7 34.8,32.0 45.0,14.3 55.2,32.0 65.5,49.7 75.7,67.5 55.2,67.5 34.8,67.5 45.0,49.7 55.2,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="34.8" y1="67.5" x2="28.2" y2="67.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="23.5,67.5 28.2,65.2 28.2,69.7" fill="#fc77be"/><circle cx="34.8" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`str, bool | null` → `str`

Removes a string and a bool or null. If it was true, return the string in upper case. If false, lowercase. If null, toggle each character's case.

---
### Patterns on this page

- [[Additive Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Separation Distillation]] · Pattern · Distillation
- [[Input Purification]] · Pattern · Purification
- [[Locator's Distillation]] · Pattern · Distillation *(from Hex Casting)*
- [[Selection Exaltation]] · Pattern · Exaltation *(from Hex Casting)*
- [[Length Purification]] · Pattern · Purification *(from Hex Casting)*
- [[Blank Reflection]] · Pattern · Reflection
- [[Spacing Reflection]] · Pattern · Reflection
- [[Comma Reflection]] · Pattern · Reflection
- [[Breaking Reflection]] · Pattern · Reflection
- [[Whisper Reflection]] · Pattern · Reflection
- [[Listener's Reflection]] · Pattern · Reflection
- [[Sifter's Gambit]] · Pattern · Gambit
- [[Sifter's Reflection]] · Pattern · Reflection
- [[Reader's Purification]] · Pattern · Purification
- [[Write]] · Spell · Spell
- [[Scrivener's Purification]] · Pattern · Purification
- [[Patternmaster's Purification]] · Pattern · Purification
- [[Moniker Purification]] · Pattern · Purification
- [[Name]] · Spell · Spell
- [[Case Distillation]] · Pattern · Distillation

**Sources:** 🌐 [Official online docs](https://moreiotas.hexxy.media/v/0.1.2/1.0/en_us/#patterns/strings) · 🧩 [MoreIotas on Modrinth](https://modrinth.com/mod/Jmt7p37B)
