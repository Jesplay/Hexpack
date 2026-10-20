---
aliases: ["Locus Spells"]
tags: [hexbook/entry, mod/slate-works, category/spells, topic/circles, topic/items]
mod: "Slate Works"
category: "Spells"
entry_id: "hexcasting:patterns/spells/storage_loci"
---

# Locus Spells

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Slate Works]]

As wonderful as **Vessels** are for storage, they have an... opaque method of interaction. The main way of interaction is with **Hexes**; more specifically, using a **Hex** to describe what I wish to retrieve. Importantly, these all [[Enlightened Mishaps|mishap when not cast in a Spell Circle]].

*“Riddles,” I said.*
*“Always preferable to no answer at all,” he responded.*

To elaborate more, **Vessels** respects Nature's silly rules regarding the "[[The Stack|Stack]] Limit" by working around them. Rather than directly calling an item based on its name, I more-or-less supply a "description;" this is a **Hex** that takes an [[Items|Item Stack Iota]], and needs to return a boolean. Also, all patterns to do with the retrieving of **Storage Vessels** take a copy of the Stack, and puts it onto the 'inner' stack (much like [[Thoth's Gambit]]).

## [[Lay Item|Lay Item]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 78.3,40.5 70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:350;--dur:5.9s" points="45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7 28.3,11.7 11.7,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,55.0 53.3,55.0 61.7,40.5 78.3,40.5 70.0,26.1 78.3,11.7 61.7,11.7 45.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="11.7" x2="42.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,19.6 40.7,15.4 43.9,17.2" fill="#fc77be"/><circle cx="45.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`entity` → `—`

It is much easier to store an item into a **Vessel** than to pull it out. This pattern simply takes an Item Entity, and neatly sorts it into the activated **Storage Vessels**. Costs an 1/8th of a [[Amethyst|dust]] per activated **Vessel**.

## [[Get Vessels|Get Vessels]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/><circle cx="11.7" cy="55.0" r="1.6"/><circle cx="28.3" cy="55.0" r="1.6"/><circle cx="45.0" cy="55.0" r="1.6"/><circle cx="61.7" cy="55.0" r="1.6"/><circle cx="78.3" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 20.0,40.5 11.7,26.1 20.0,11.7 36.7,11.7 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 61.7,26.1 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:317;--dur:5.3s" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 20.0,40.5 11.7,26.1 20.0,11.7 36.7,11.7 53.3,11.7 70.0,11.7 78.3,26.1 70.0,40.5 61.7,26.1 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="42.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,34.0 40.7,29.8 43.9,31.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `[vecs]`

This simply returns what **Vessels** the current [[Spell Circles|Spell Circle]] has activated. I should likely use this to check if the [[Spell Circles|Spell Circle]] has collected any **Vessels**.

## [[Reawaken Item|Reawaken Item]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="81" viewBox="0 0 90.0 80.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="9.8" cy="9.8" r="1.6"/><circle cx="23.9" cy="9.8" r="1.6"/><circle cx="38.0" cy="9.8" r="1.6"/><circle cx="52.0" cy="9.8" r="1.6"/><circle cx="66.1" cy="9.8" r="1.6"/><circle cx="80.2" cy="9.8" r="1.6"/><circle cx="2.8" cy="22.0" r="1.6"/><circle cx="16.9" cy="22.0" r="1.6"/><circle cx="30.9" cy="22.0" r="1.6"/><circle cx="45.0" cy="22.0" r="1.6"/><circle cx="59.1" cy="22.0" r="1.6"/><circle cx="73.1" cy="22.0" r="1.6"/><circle cx="87.2" cy="22.0" r="1.6"/><circle cx="9.8" cy="34.2" r="1.6"/><circle cx="23.9" cy="34.2" r="1.6"/><circle cx="38.0" cy="34.2" r="1.6"/><circle cx="52.0" cy="34.2" r="1.6"/><circle cx="66.1" cy="34.2" r="1.6"/><circle cx="80.2" cy="34.2" r="1.6"/><circle cx="2.8" cy="46.4" r="1.6"/><circle cx="16.9" cy="46.4" r="1.6"/><circle cx="30.9" cy="46.4" r="1.6"/><circle cx="45.0" cy="46.4" r="1.6"/><circle cx="59.1" cy="46.4" r="1.6"/><circle cx="73.1" cy="46.4" r="1.6"/><circle cx="87.2" cy="46.4" r="1.6"/><circle cx="9.8" cy="58.6" r="1.6"/><circle cx="23.9" cy="58.6" r="1.6"/><circle cx="38.0" cy="58.6" r="1.6"/><circle cx="52.0" cy="58.6" r="1.6"/><circle cx="66.1" cy="58.6" r="1.6"/><circle cx="80.2" cy="58.6" r="1.6"/><circle cx="2.8" cy="70.7" r="1.6"/><circle cx="16.9" cy="70.7" r="1.6"/><circle cx="30.9" cy="70.7" r="1.6"/><circle cx="45.0" cy="70.7" r="1.6"/><circle cx="59.1" cy="70.7" r="1.6"/><circle cx="73.1" cy="70.7" r="1.6"/><circle cx="87.2" cy="70.7" r="1.6"/></g><polyline class="hexpat-base" points="45.0,22.0 38.0,34.2 23.9,34.2 30.9,46.4 45.0,46.4 59.1,46.4 66.1,34.2 52.0,34.2 45.0,22.0 30.9,22.0 16.9,22.0 9.8,34.2 16.9,46.4 23.9,58.6 38.0,58.6 52.0,58.6 66.1,58.6 73.1,46.4 80.2,34.2 73.1,22.0 59.1,22.0 45.0,22.0 38.0,9.8 23.9,9.8 9.8,9.8 16.9,22.0 23.9,34.2 16.9,46.4 30.9,46.4 23.9,58.6 16.9,70.7 30.9,70.7 45.0,70.7 59.1,70.7 73.1,70.7 66.1,58.6 59.1,46.4 73.1,46.4 66.1,34.2 73.1,22.0 80.2,9.8 66.1,9.8 52.0,9.8 45.0,22.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:605;--dur:6.0s" points="45.0,22.0 38.0,34.2 23.9,34.2 30.9,46.4 45.0,46.4 59.1,46.4 66.1,34.2 52.0,34.2 45.0,22.0 30.9,22.0 16.9,22.0 9.8,34.2 16.9,46.4 23.9,58.6 38.0,58.6 52.0,58.6 66.1,58.6 73.1,46.4 80.2,34.2 73.1,22.0 59.1,22.0 45.0,22.0 38.0,9.8 23.9,9.8 9.8,9.8 16.9,22.0 23.9,34.2 16.9,46.4 30.9,46.4 23.9,58.6 16.9,70.7 30.9,70.7 45.0,70.7 59.1,70.7 73.1,70.7 66.1,58.6 59.1,46.4 73.1,46.4 66.1,34.2 73.1,22.0 80.2,9.8 66.1,9.8 52.0,9.8 45.0,22.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="22.0" x2="42.8" y2="25.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="41.1,28.7 41.4,25.1 44.1,26.7" fill="#fc77be"/><circle cx="45.0" cy="22.0" r="2.6" fill="#fc77be"/></svg>

`[patterns]` → `—`

This takes a **Hex** and runs it on every [[Items|Item Stack]] stored (with it on top of the stack). Once the **Hex** is executed, it requires a **Integer**, **Vector**, and **Boolean**, left on the stack, in that order.

Further explained: this runs the given **Hex** on every stored Item in the **Vessels**. The **Hex** must return (top to bottom) either a False (omits the Vector, and Number) or True, Vector, and Number. The Vector corresponds to where to output the items into the world (respecting ambit), and Number relates to how many of that type of item to retrieve.

Costs a 1/4th of a [[Amethyst|dust]] per activated **Vessel**.

## [[Check Item|Check Item]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/><circle cx="11.7" cy="55.0" r="1.6"/><circle cx="28.3" cy="55.0" r="1.6"/><circle cx="45.0" cy="55.0" r="1.6"/><circle cx="61.7" cy="55.0" r="1.6"/><circle cx="78.3" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 20.0,40.5 11.7,26.1 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 78.3,26.1 70.0,40.5 61.7,26.1 45.0,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:317;--dur:5.3s" points="45.0,26.1 36.7,40.5 20.0,40.5 28.3,55.0 45.0,55.0 61.7,55.0 70.0,40.5 53.3,40.5 45.0,26.1 28.3,26.1 20.0,40.5 11.7,26.1 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 78.3,26.1 70.0,40.5 61.7,26.1 45.0,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="42.3" y2="30.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,34.0 40.7,29.8 43.9,31.6" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`[patterns]` → `bool`

Check Item functions much like Reawaken Item; however, it only requires a Boolean from the inputted **Hex**. If the Boolean is ever True, the spell ends, and returns True.

## [[Reorient Items|Reorient Items]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="67" viewBox="0 0 90.0 66.6"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="11.7" cy="11.7" r="1.6"/><circle cx="28.3" cy="11.7" r="1.6"/><circle cx="45.0" cy="11.7" r="1.6"/><circle cx="61.7" cy="11.7" r="1.6"/><circle cx="78.3" cy="11.7" r="1.6"/><circle cx="3.3" cy="26.1" r="1.6"/><circle cx="20.0" cy="26.1" r="1.6"/><circle cx="36.7" cy="26.1" r="1.6"/><circle cx="53.3" cy="26.1" r="1.6"/><circle cx="70.0" cy="26.1" r="1.6"/><circle cx="86.7" cy="26.1" r="1.6"/><circle cx="11.7" cy="40.5" r="1.6"/><circle cx="28.3" cy="40.5" r="1.6"/><circle cx="45.0" cy="40.5" r="1.6"/><circle cx="61.7" cy="40.5" r="1.6"/><circle cx="78.3" cy="40.5" r="1.6"/><circle cx="3.3" cy="55.0" r="1.6"/><circle cx="20.0" cy="55.0" r="1.6"/><circle cx="36.7" cy="55.0" r="1.6"/><circle cx="53.3" cy="55.0" r="1.6"/><circle cx="70.0" cy="55.0" r="1.6"/><circle cx="86.7" cy="55.0" r="1.6"/></g><polyline class="hexpat-base" points="45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,55.0 45.0,40.5 53.3,55.0 61.7,40.5 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:333;--dur:5.6s" points="45.0,11.7 36.7,26.1 20.0,26.1 28.3,40.5 45.0,40.5 61.7,40.5 70.0,26.1 53.3,26.1 45.0,11.7 28.3,11.7 20.0,26.1 11.7,40.5 28.3,40.5 36.7,55.0 45.0,40.5 53.3,55.0 61.7,40.5 78.3,40.5 70.0,26.1 61.7,11.7 45.0,11.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="11.7" x2="42.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,19.6 40.7,15.4 43.9,17.2" fill="#fc77be"/><circle cx="45.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

In some rare scenarios, **Vessels** can get jumbled up and have items of the same type spread across different **Vessels**. This is a neat spell to resort them at a cost. That cost being 5 [[Amethyst|Charged Amethyst]].

## [[Bind Macro|Bind Macro]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="13.0" r="1.6"/><circle cx="22.2" cy="13.0" r="1.6"/><circle cx="40.7" cy="13.0" r="1.6"/><circle cx="59.2" cy="13.0" r="1.6"/><circle cx="77.7" cy="13.0" r="1.6"/><circle cx="13.0" cy="29.0" r="1.6"/><circle cx="31.5" cy="29.0" r="1.6"/><circle cx="50.0" cy="29.0" r="1.6"/><circle cx="68.5" cy="29.0" r="1.6"/><circle cx="3.7" cy="45.0" r="1.6"/><circle cx="22.2" cy="45.0" r="1.6"/><circle cx="40.7" cy="45.0" r="1.6"/><circle cx="59.2" cy="45.0" r="1.6"/><circle cx="77.7" cy="45.0" r="1.6"/><circle cx="13.0" cy="61.0" r="1.6"/><circle cx="31.5" cy="61.0" r="1.6"/><circle cx="50.0" cy="61.0" r="1.6"/><circle cx="68.5" cy="61.0" r="1.6"/><circle cx="3.7" cy="77.0" r="1.6"/><circle cx="22.2" cy="77.0" r="1.6"/><circle cx="40.7" cy="77.0" r="1.6"/><circle cx="59.2" cy="77.0" r="1.6"/><circle cx="77.7" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="40.7,45.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0 59.2,77.0 68.5,61.0 59.2,45.0 40.7,45.0 31.5,61.0 40.7,77.0 50.0,61.0 40.7,45.0 50.0,29.0 68.5,29.0 59.2,13.0 40.7,13.0 22.2,13.0 13.0,29.0 31.5,29.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:370;--dur:5.6s" points="40.7,45.0 22.2,45.0 13.0,61.0 22.2,77.0 40.7,77.0 59.2,77.0 68.5,61.0 59.2,45.0 40.7,45.0 31.5,61.0 40.7,77.0 50.0,61.0 40.7,45.0 50.0,29.0 68.5,29.0 59.2,13.0 40.7,13.0 22.2,13.0 13.0,29.0 31.5,29.0 40.7,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="40.7" y1="45.0" x2="34.8" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="30.5,45.0 34.8,43.0 34.8,47.0" fill="#fc77be"/><circle cx="40.7" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`vec, any iota, pattern` → `—`

This binds both a pattern and an [[Iotas|iota]] to a [[Spell Imprinter]], which is at the targeted vector.

Does not [[Mishaps|mishap]] if casted outside of a [[Spell Circles|Spell Circle]]. Costs a negligible amount of **[[Media|media]]**.

## [[Review Broadcast|Review Broadcast]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="81" height="90" viewBox="0 0 81.4 90.0"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="13.0" cy="13.0" r="1.6"/><circle cx="31.5" cy="13.0" r="1.6"/><circle cx="50.0" cy="13.0" r="1.6"/><circle cx="68.5" cy="13.0" r="1.6"/><circle cx="3.7" cy="29.0" r="1.6"/><circle cx="22.2" cy="29.0" r="1.6"/><circle cx="40.7" cy="29.0" r="1.6"/><circle cx="59.2" cy="29.0" r="1.6"/><circle cx="77.7" cy="29.0" r="1.6"/><circle cx="13.0" cy="45.0" r="1.6"/><circle cx="31.5" cy="45.0" r="1.6"/><circle cx="50.0" cy="45.0" r="1.6"/><circle cx="68.5" cy="45.0" r="1.6"/><circle cx="3.7" cy="61.0" r="1.6"/><circle cx="22.2" cy="61.0" r="1.6"/><circle cx="40.7" cy="61.0" r="1.6"/><circle cx="59.2" cy="61.0" r="1.6"/><circle cx="77.7" cy="61.0" r="1.6"/><circle cx="13.0" cy="77.0" r="1.6"/><circle cx="31.5" cy="77.0" r="1.6"/><circle cx="50.0" cy="77.0" r="1.6"/><circle cx="68.5" cy="77.0" r="1.6"/></g><polyline class="hexpat-base" points="31.5,45.0 13.0,45.0 22.2,61.0 40.7,61.0 59.2,61.0 68.5,45.0 50.0,45.0 31.5,45.0 22.2,29.0 31.5,13.0 50.0,13.0 59.2,29.0 50.0,45.0 40.7,61.0 31.5,77.0 50.0,77.0 40.7,61.0 31.5,45.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:315;--dur:4.8s" points="31.5,45.0 13.0,45.0 22.2,61.0 40.7,61.0 59.2,61.0 68.5,45.0 50.0,45.0 31.5,45.0 22.2,29.0 31.5,13.0 50.0,13.0 59.2,29.0 50.0,45.0 40.7,61.0 31.5,77.0 50.0,77.0 40.7,61.0 31.5,45.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.5" y1="45.0" x2="25.5" y2="45.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="21.3,45.0 25.5,43.0 25.5,47.0" fill="#fc77be"/><circle cx="31.5" cy="45.0" r="2.6" fill="#fc77be"/></svg>

`vec` → `any`

Attempts to read an [[Iotic Door]] at the given vector. If there is not a [[Iotic Door|Door]], returns garbage.
Does not mishap if casted outside of a [[Spell Circles|Spell Circle]], free, and does not require ambit.

---
### Patterns on this page

- [[Lay Item]] · Spell · Spell
- [[Get Vessels]] · Pattern · Other
- [[Reawaken Item]] · Pattern · Other
- [[Check Item]] · Pattern · Other
- [[Reorient Items]] · Pattern · Other
- [[Bind Macro]] · Pattern · Other
- [[Review Broadcast]] · Pattern · Other

**Sources:** 🧩 [Slate Works on Modrinth](https://modrinth.com/mod/3DnSSjl3)
