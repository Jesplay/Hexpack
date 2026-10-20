---
aliases: ["Amalgamations", "Amalgamation Container"]
tags: [hexbook/entry, mod/lapisworks, category/chariot, topic/enchanting, topic/minds]
mod: "Lapisworks"
category: "Chariot"
entry_id: "hexcasting:interop/lapisphantics/amalgamations"
---

# Amalgamations

> **Category:** Lapisworks › Interop › [[Chariot (Category)|Chariot]]  
> **Source mod:** [[Lapisworks]]

While most villager minds can be used somehow in my [[Spell Circles]], there are some types of villagers which have little to no place in them. For these I have devised an alternative: **Amalgamations**. By [[Flay Mind (Book Entry)|flaying]] a villager that is at least near it's maximum into a full simple mind, I can create these amalgamations, perversions of nature to the utmost degree, a meld of minds in tiny, unfit containers.

**Amalgamation Container**: [[Flay Mind]] a **fisherman** villager, level 5+ into **Simple Mind Container**

The result isn't two minds, but *one singular mind in complete agony.* Memories twisting and turning, two identities or one? I cannot imagine the suffering incurred, but don't need to. **Their feeble lives are for my genius after all.**

**Amalgamation Container**: [[Flay Mind]] a **fisherman** villager, level 3+ into **Simple Mind Container**

A lesser villager to make a simpler amalgamation. Differences explained on the next page.

Embedded amalgamations cast when they detect my entity reference be added to a [[The Stack|stack]] within range. Complex ones (due to being formed by more experienced villagers) always detect this event and start the cast with a "guess" of where the event came from, guaranteed to be within a certain error range of the real position. Simple Amalgamations have a chance to cast (which depends on their error value) and have less maximum range but always start with the exact position of the offender.

Before [[Imbue Mind (Book Entry)|imbuing]] the amalgamation into the **Imbuement Bed** I'm on (which may waste the amalgamation if a player is not on the bed) I must configure the amalgamation. To do this, I have been granted **Amalgamation** [[Iotas|iotas]], which are linked to their block of origin. All patterns concerning these are documented in the next pages.

## [[Amalgamation Purification|Amalgamation Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,64.4 58.2,41.5 45.0,18.5 31.8,41.5 45.0,64.4 71.5,64.4 58.2,41.5 31.8,41.5 18.5,64.4 45.0,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="45.0,64.4 58.2,41.5 45.0,18.5 31.8,41.5 45.0,64.4 71.5,64.4 58.2,41.5 31.8,41.5 18.5,64.4 45.0,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="64.4" x2="49.2" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="52.3,51.8 51.8,58.5 46.7,55.6" fill="#fc77be"/><circle cx="45.0" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`vec` → `amalgam`

Returns the **Amalgamation** in the container at the specified position (within range). Any changes to it are reflected in the block it came from.

## [[The Mind's Decomp|The Mind's Decomp.]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="23.2,80.4 30.0,68.6 23.2,56.8 16.4,45.0 23.2,33.2 30.0,21.4 23.2,9.6 16.4,21.4 23.2,33.2 30.0,45.0 23.2,56.8 16.4,68.6 23.2,80.4 36.8,80.4 30.0,68.6 16.4,68.6 9.6,80.4 23.2,80.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:232;--dur:4.8s" points="23.2,80.4 30.0,68.6 23.2,56.8 16.4,45.0 23.2,33.2 30.0,21.4 23.2,9.6 16.4,21.4 23.2,33.2 30.0,45.0 23.2,56.8 16.4,68.6 23.2,80.4 36.8,80.4 30.0,68.6 16.4,68.6 9.6,80.4 23.2,80.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="23.2" y1="80.4" x2="25.4" y2="76.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.9,73.9 26.7,77.4 24.1,75.9" fill="#fc77be"/><circle cx="23.2" cy="80.4" r="2.6" fill="#fc77be"/></svg>

`amalgam` → `amalgam, bool`

Pushes whether the provided amalgamation is **Complex** (true) or **Simple** (false).

### Loudness

**Loudness** determines how much effort an amalgamation would put into alerting the person it's embedded in. 0 means the amalgamation would make no effort, 1 means the amalgamation would [[Reveal|Reveals]] a warning to the person, 2 means the amalgamation would produce a sound as well, and 3 means the amalgamation would make text flash in front of the person's eyes for a few seconds.

## [[Loudener's Decomposition|Loudener's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="65.5,67.5 55.2,49.7 45.0,32.0 34.8,49.7 45.0,67.5 55.2,49.7 34.8,49.7 24.5,67.5 45.0,67.5 65.5,67.5 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 24.5,32.0 14.3,49.7 24.5,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:327;--dur:4.5s" points="65.5,67.5 55.2,49.7 45.0,32.0 34.8,49.7 45.0,67.5 55.2,49.7 34.8,49.7 24.5,67.5 45.0,67.5 65.5,67.5 75.7,49.7 65.5,32.0 55.2,14.3 34.8,14.3 24.5,32.0 14.3,49.7 24.5,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="65.5" y1="67.5" x2="62.2" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="59.8,57.7 64.1,60.7 60.2,62.9" fill="#fc77be"/><circle cx="65.5" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`amalgam` → `amalgam, int`

Returns the **Amalgamation** in the container at the specified position (within range). Any changes to it are reflected in the block it came from.

## [[Loudener's Distillation|Loudener's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="82" viewBox="0 0 90.0 81.8"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="14.3" cy="14.3" r="1.6"/><circle cx="34.8" cy="14.3" r="1.6"/><circle cx="55.2" cy="14.3" r="1.6"/><circle cx="75.7" cy="14.3" r="1.6"/><circle cx="4.1" cy="32.0" r="1.6"/><circle cx="24.5" cy="32.0" r="1.6"/><circle cx="45.0" cy="32.0" r="1.6"/><circle cx="65.5" cy="32.0" r="1.6"/><circle cx="85.9" cy="32.0" r="1.6"/><circle cx="14.3" cy="49.7" r="1.6"/><circle cx="34.8" cy="49.7" r="1.6"/><circle cx="55.2" cy="49.7" r="1.6"/><circle cx="75.7" cy="49.7" r="1.6"/><circle cx="4.1" cy="67.5" r="1.6"/><circle cx="24.5" cy="67.5" r="1.6"/><circle cx="45.0" cy="67.5" r="1.6"/><circle cx="65.5" cy="67.5" r="1.6"/><circle cx="85.9" cy="67.5" r="1.6"/></g><polyline class="hexpat-base" points="24.5,67.5 34.8,49.7 45.0,32.0 55.2,49.7 45.0,67.5 34.8,49.7 55.2,49.7 65.5,67.5 45.0,67.5 24.5,67.5 14.3,49.7 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 65.5,67.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:327;--dur:4.5s" points="24.5,67.5 34.8,49.7 45.0,32.0 55.2,49.7 45.0,67.5 34.8,49.7 55.2,49.7 65.5,67.5 45.0,67.5 24.5,67.5 14.3,49.7 24.5,32.0 34.8,14.3 55.2,14.3 65.5,32.0 75.7,49.7 65.5,67.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="67.5" x2="27.8" y2="61.8" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.2,57.7 29.8,62.9 25.9,60.7" fill="#fc77be"/><circle cx="24.5" cy="67.5" r="2.6" fill="#fc77be"/></svg>

`amalgam, num` → `amalgam`

Sets the provided amalgamation's loudness to the specified number (which must be positive and below 4).

### Range

**Range** determines the range at which the amalgamation can detect an offender. For Simple Amalgamations the maximum is 64 blocks, while for Complex Amalgamations the maximum is 256 blocks.

## [[Fisherman's Decomposition|Fisherman's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 53.3,26.1 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 53.3,55.0 45.0,40.5 36.7,55.0 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 36.7,26.1 45.0,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,40.5 45.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:417;--dur:6.0s" points="45.0,40.5 53.3,26.1 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 20.0,55.0 28.3,69.4 45.0,69.4 53.3,55.0 45.0,40.5 36.7,55.0 45.0,69.4 61.7,69.4 70.0,55.0 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7 36.7,26.1 45.0,40.5 61.7,40.5 53.3,26.1 36.7,26.1 28.3,40.5 45.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="47.7" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="49.6,32.6 49.3,36.8 46.1,35.0" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`amalgam` → `amalgam, num`

Pushes the range of the provided amalgamation.

## [[Fisherman's Distillation|Fisherman's Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 81.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/><circle cx="11.7" cy="69.4" r="1.6"/><circle cx="28.3" cy="69.4" r="1.6"/><circle cx="45.0" cy="69.4" r="1.6"/><circle cx="61.7" cy="69.4" r="1.6"/><circle cx="78.3" cy="69.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,40.5 36.7,26.1 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 36.7,55.0 45.0,40.5 53.3,55.0 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 53.3,26.1 45.0,40.5 28.3,40.5 36.7,26.1 53.3,26.1 61.7,40.5 45.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:417;--dur:6.0s" points="45.0,40.5 36.7,26.1 45.0,11.7 61.7,11.7 70.0,26.1 78.3,40.5 70.0,55.0 61.7,69.4 45.0,69.4 36.7,55.0 45.0,40.5 53.3,55.0 45.0,69.4 28.3,69.4 20.0,55.0 11.7,40.5 20.0,26.1 28.3,11.7 45.0,11.7 53.3,26.1 45.0,40.5 28.3,40.5 36.7,26.1 53.3,26.1 61.7,40.5 45.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="40.5" x2="42.3" y2="35.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,32.6 43.9,35.0 40.7,36.8" fill="#fc77be"/><circle cx="45.0" cy="40.5" r="2.6" fill="#fc77be"/></svg>

`amalgam, num` → `amalgam`

Sets the provided amalgamation's range to the specified number, mishapping if negative or out of bounds.

### Inside Me

Embedded amalgamations are sorted by order of adding (so the first amalgamation to be added would be at the first index). When anyone in range adds my reference to their stack these amalgamations are cycled through and the first to successfully trigger's hex is activated. There can only be one execution attempt of an amalgamation per twentieth of a second, and I may only have three amalgamations at maximum.

## [[My Mind's Decomposition|My Mind's Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="63" height="90" viewBox="0 0 62.9 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,77.0 40.7,61.0 31.5,45.0 22.2,29.0 31.5,13.0 40.7,29.0 31.5,45.0 22.2,61.0 31.5,77.0 50.0,77.0 40.7,61.0 22.2,61.0 13.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:241;--dur:3.6s" points="31.5,77.0 40.7,61.0 31.5,45.0 22.2,29.0 31.5,13.0 40.7,29.0 31.5,45.0 22.2,61.0 31.5,77.0 50.0,77.0 40.7,61.0 22.2,61.0 13.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="77.0" x2="34.4" y2="71.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,68.2 36.2,72.9 32.7,70.9" fill="#fc77be"/><circle cx="31.5" cy="77.0" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Pushes the number of amalgamations on my person.

## [[Amalgamation Gambit|Amalgamation Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="83" viewBox="0 0 90.0 82.9"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="18.5" cy="18.5" r="1.6"/><circle cx="45.0" cy="18.5" r="1.6"/><circle cx="71.5" cy="18.5" r="1.6"/><circle cx="5.3" cy="41.5" r="1.6"/><circle cx="31.8" cy="41.5" r="1.6"/><circle cx="58.2" cy="41.5" r="1.6"/><circle cx="84.7" cy="41.5" r="1.6"/><circle cx="18.5" cy="64.4" r="1.6"/><circle cx="45.0" cy="64.4" r="1.6"/><circle cx="71.5" cy="64.4" r="1.6"/></g><polyline class="hexpat-base" points="45.0,64.4 31.8,41.5 45.0,18.5 58.2,41.5 45.0,64.4 18.5,64.4 31.8,41.5 58.2,41.5 71.5,64.4 45.0,64.4" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:238;--dur:2.5s" points="45.0,64.4 31.8,41.5 45.0,18.5 58.2,41.5 45.0,64.4 18.5,64.4 31.8,41.5 58.2,41.5 71.5,64.4 45.0,64.4" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="64.4" x2="40.8" y2="57.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="37.7,51.8 43.3,55.6 38.2,58.5" fill="#fc77be"/><circle cx="45.0" cy="64.4" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

Removes the amalgamation on my person with that index.

### Error

**Error** is defined as range minus 32 then divided by eight (**(r-32)/8**) for simple and just range divided by four (**r/4**) for complex amalgamations. Error may never exceed 32. A hundred divided by err (**100/err**) is the percentage chance a **Simple Amalgamation** successfully triggers, while error is the range of innaccuracy of the guess a complex one provides.

## [[Mistake Decomposition|Mistake Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="46" height="90" viewBox="0 0 46.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.6" cy="9.6" r="1.6"/><circle cx="23.2" cy="9.6" r="1.6"/><circle cx="36.8" cy="9.6" r="1.6"/><circle cx="2.7" cy="21.4" r="1.6"/><circle cx="16.4" cy="21.4" r="1.6"/><circle cx="30.0" cy="21.4" r="1.6"/><circle cx="43.7" cy="21.4" r="1.6"/><circle cx="9.6" cy="33.2" r="1.6"/><circle cx="23.2" cy="33.2" r="1.6"/><circle cx="36.8" cy="33.2" r="1.6"/><circle cx="2.7" cy="45.0" r="1.6"/><circle cx="16.4" cy="45.0" r="1.6"/><circle cx="30.0" cy="45.0" r="1.6"/><circle cx="43.7" cy="45.0" r="1.6"/><circle cx="9.6" cy="56.8" r="1.6"/><circle cx="23.2" cy="56.8" r="1.6"/><circle cx="36.8" cy="56.8" r="1.6"/><circle cx="2.7" cy="68.6" r="1.6"/><circle cx="16.4" cy="68.6" r="1.6"/><circle cx="30.0" cy="68.6" r="1.6"/><circle cx="43.7" cy="68.6" r="1.6"/><circle cx="9.6" cy="80.4" r="1.6"/><circle cx="23.2" cy="80.4" r="1.6"/><circle cx="36.8" cy="80.4" r="1.6"/></g><polyline class="hexpat-base" points="30.0,45.0 23.2,33.2 30.0,21.4 23.2,9.6 16.4,21.4 23.2,33.2 36.8,33.2 30.0,21.4 16.4,21.4 9.6,33.2 23.2,33.2 16.4,45.0 23.2,56.8 16.4,68.6 23.2,80.4 30.0,68.6 23.2,56.8 9.6,56.8 16.4,68.6 30.0,68.6 36.8,56.8 23.2,56.8 30.0,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:300;--dur:6.0s" points="30.0,45.0 23.2,33.2 30.0,21.4 23.2,9.6 16.4,21.4 23.2,33.2 36.8,33.2 30.0,21.4 16.4,21.4 9.6,33.2 23.2,33.2 16.4,45.0 23.2,56.8 16.4,68.6 23.2,80.4 30.0,68.6 23.2,56.8 9.6,56.8 16.4,68.6 30.0,68.6 36.8,56.8 23.2,56.8 30.0,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="30.0" y1="45.0" x2="27.8" y2="41.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.3,38.5 29.1,40.5 26.5,42.0" fill="#fc77be"/><circle cx="30.0" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`amalgam` → `amalgam, num`

Pushes the error of the provided amalgamation.

## [[Etched Decomposition|Etched Decomposition]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="68.5" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="68.5" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/><circle cx="68.5" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 31.5,45.0 22.2,61.0 40.7,61.0 50.0,45.0 59.2,61.0 50.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="31.5,45.0 40.7,29.0 31.5,13.0 22.2,29.0 31.5,45.0 50.0,45.0 40.7,29.0 22.2,29.0 13.0,45.0 31.5,45.0 22.2,61.0 40.7,61.0 50.0,45.0 59.2,61.0 50.0,77.0 31.5,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="45.0" x2="34.4" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="36.5,36.2 36.2,40.9 32.7,38.9" fill="#fc77be"/><circle cx="31.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`amalgam` → `amalgam, [pattern]`

Pushes the hex that the amalgamation at the top of the stack would execute upon a successful trigger.

## [[Etched Distillation|Etched Distillation]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="72" height="90" viewBox="0 0 72.2 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,45.0 31.5,29.0 40.7,13.0 50.0,29.0 40.7,45.0 22.2,45.0 31.5,29.0 50.0,29.0 59.2,45.0 40.7,45.0 50.0,61.0 31.5,61.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:278;--dur:4.2s" points="40.7,45.0 31.5,29.0 40.7,13.0 50.0,29.0 40.7,45.0 22.2,45.0 31.5,29.0 50.0,29.0 59.2,45.0 40.7,45.0 50.0,61.0 31.5,61.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="45.0" x2="37.7" y2="39.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.6,36.2 39.5,38.9 36.0,40.9" fill="#fc77be"/><circle cx="40.7" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`amalgam, [pattern]` → `amalgam`

Sets the hex that the provided amalgamation would execute upon a successful trigger.

---
### Patterns on this page

- [[Amalgamation Purification]] · Pattern · Purification
- [[The Mind's Decomp]] · Pattern · Decomposition
- [[Loudener's Decomposition]] · Pattern · Decomposition
- [[Loudener's Distillation]] · Pattern · Distillation
- [[Fisherman's Decomposition]] · Pattern · Decomposition
- [[Fisherman's Distillation]] · Pattern · Distillation
- [[My Mind's Decomposition]] · Pattern · Decomposition
- [[Amalgamation Gambit]] · Pattern · Gambit
- [[Mistake Decomposition]] · Pattern · Decomposition
- [[Etched Decomposition]] · Pattern · Decomposition
- [[Etched Distillation]] · Pattern · Distillation

**Sources:** 🧩 [Lapisworks on Modrinth](https://modrinth.com/mod/YPwDELmO)
