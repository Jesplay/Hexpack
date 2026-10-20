---
aliases: ["Links"]
tags: [hexbook/entry, mod/hexal, category/spells, topic/wisps]
mod: "Hexal"
category: "Spells"
entry_id: "hexcasting:patterns/spells/links"
---

# Links

> **Category:** Patterns › [[Spells (Category)|Spells]]  
> **Source mod:** [[Hexal]]

These patterns outline a system that I have found for communicating information from one [[Wisps|wisp]] to another, among other things. There are a number of patterns related to creating these links, and a number of patterns related to sending and receiving [[Iotas|iotas]] over a link once it is created. A wisp can have any number of links active, with each costing a negligible amount of [[Media|media]] to maintain each twentieth of a second.

The links of a given linkable are indexed by the order they are created, with this number used by a number of patterns. "Linkable" is not its own type, but a shorthand for any type that can be coerced into a linkable (e.g. Entity -&gt; Linkable entities, Vec -&gt; Linkable blocks, ...). Links can span up to double the sum of the two linkable's ranges. To connect linkables over truly vast distances I would do well to look into building some [[Relay|Relays]].

## [[Link|Link]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="8.5,19.0 20.7,19.0 26.8,29.6 32.8,19.0 26.8,8.5 20.7,19.0 32.8,19.0 38.9,29.6 51.1,29.6 57.2,19.0 51.1,8.5 38.9,8.5 32.8,19.0 45.0,19.0 57.2,19.0 63.2,29.6 69.3,19.0 63.2,8.5 57.2,19.0 69.3,19.0 81.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:243;--dur:5.6s" points="8.5,19.0 20.7,19.0 26.8,29.6 32.8,19.0 26.8,8.5 20.7,19.0 32.8,19.0 38.9,29.6 51.1,29.6 57.2,19.0 51.1,8.5 38.9,8.5 32.8,19.0 45.0,19.0 57.2,19.0 63.2,29.6 69.3,19.0 63.2,8.5 57.2,19.0 69.3,19.0 81.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="8.5" y1="19.0" x2="12.4" y2="19.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="15.2,19.0 12.4,20.4 12.4,17.7" fill="#fc77be"/><circle cx="8.5" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`linkable` → `—`

Links the caster to the entity removed from the [[The Stack|stack]], which must be linkable. This costs one [[Amethyst|Amethyst Shard]].

## [[Link Others|Link Others]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="8.5,19.0 20.7,19.0 26.8,29.6 38.9,29.6 45.0,19.0 38.9,8.5 26.8,8.5 20.7,19.0 32.8,19.0 45.0,19.0 51.1,8.5 63.2,8.5 69.3,19.0 63.2,29.6 51.1,29.6 45.0,19.0 57.2,19.0 69.3,19.0 81.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:219;--dur:5.0s" points="8.5,19.0 20.7,19.0 26.8,29.6 38.9,29.6 45.0,19.0 38.9,8.5 26.8,8.5 20.7,19.0 32.8,19.0 45.0,19.0 51.1,8.5 63.2,8.5 69.3,19.0 63.2,29.6 51.1,29.6 45.0,19.0 57.2,19.0 69.3,19.0 81.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="8.5" y1="19.0" x2="12.4" y2="19.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="15.2,19.0 12.4,20.4 12.4,17.7" fill="#fc77be"/><circle cx="8.5" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`linkable, linkable` → `—`

May be evaluated by any casting instrument. Links two entities, which must be linkable. This costs one [[Amethyst|Amethyst Shard]].

## [[Unlink|Unlink]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="81.5,19.0 69.3,19.0 63.2,29.6 57.2,19.0 63.2,8.5 69.3,19.0 57.2,19.0 51.1,29.6 38.9,29.6 32.8,19.0 38.9,8.5 51.1,8.5 57.2,19.0 45.0,19.0 32.8,19.0 26.8,29.6 20.7,19.0 26.8,8.5 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:243;--dur:5.6s" points="81.5,19.0 69.3,19.0 63.2,29.6 57.2,19.0 63.2,8.5 69.3,19.0 57.2,19.0 51.1,29.6 38.9,29.6 32.8,19.0 38.9,8.5 51.1,8.5 57.2,19.0 45.0,19.0 32.8,19.0 26.8,29.6 20.7,19.0 26.8,8.5 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="81.5" y1="19.0" x2="77.6" y2="19.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="74.8,19.0 77.6,17.7 77.6,20.4" fill="#fc77be"/><circle cx="81.5" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

Removes the link at the given index from the caster's links. This costs two [[Amethyst|Amethyst Dust]].

## [[Unlink Others|Unlink Others]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="38" viewBox="0 0 90.0 38.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="2.4" cy="8.5" r="1.6"/><circle cx="14.6" cy="8.5" r="1.6"/><circle cx="26.8" cy="8.5" r="1.6"/><circle cx="38.9" cy="8.5" r="1.6"/><circle cx="51.1" cy="8.5" r="1.6"/><circle cx="63.2" cy="8.5" r="1.6"/><circle cx="75.4" cy="8.5" r="1.6"/><circle cx="87.6" cy="8.5" r="1.6"/><circle cx="8.5" cy="19.0" r="1.6"/><circle cx="20.7" cy="19.0" r="1.6"/><circle cx="32.8" cy="19.0" r="1.6"/><circle cx="45.0" cy="19.0" r="1.6"/><circle cx="57.2" cy="19.0" r="1.6"/><circle cx="69.3" cy="19.0" r="1.6"/><circle cx="81.5" cy="19.0" r="1.6"/><circle cx="2.4" cy="29.6" r="1.6"/><circle cx="14.6" cy="29.6" r="1.6"/><circle cx="26.8" cy="29.6" r="1.6"/><circle cx="38.9" cy="29.6" r="1.6"/><circle cx="51.1" cy="29.6" r="1.6"/><circle cx="63.2" cy="29.6" r="1.6"/><circle cx="75.4" cy="29.6" r="1.6"/><circle cx="87.6" cy="29.6" r="1.6"/></g><polyline class="hexpat-base" points="81.5,19.0 69.3,19.0 63.2,29.6 51.1,29.6 45.0,19.0 51.1,8.5 63.2,8.5 69.3,19.0 57.2,19.0 45.0,19.0 38.9,8.5 26.8,8.5 20.7,19.0 26.8,29.6 38.9,29.6 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:219;--dur:5.0s" points="81.5,19.0 69.3,19.0 63.2,29.6 51.1,29.6 45.0,19.0 51.1,8.5 63.2,8.5 69.3,19.0 57.2,19.0 45.0,19.0 38.9,8.5 26.8,8.5 20.7,19.0 26.8,29.6 38.9,29.6 45.0,19.0 32.8,19.0 20.7,19.0 8.5,19.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="81.5" y1="19.0" x2="77.6" y2="19.0" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="74.8,19.0 77.6,17.7 77.6,20.4" fill="#fc77be"/><circle cx="81.5" cy="19.0" r="2.6" fill="#fc77be"/></svg>

`linkable, linkable` → `—`

Takes two linkables and unlinks them if they are linked. This costs two [[Amethyst|Amethyst Dust]].

## [[Phonebook Purification|Phonebook Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:167;--dur:2.8s" points="11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="11.7" y1="26.1" x2="17.0" y2="26.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="20.8,26.1 17.0,27.9 17.0,24.3" fill="#fc77be"/><circle cx="11.7" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`num` → `linkable`

Removes an index from the stack, and adds the entity connected to the link at that index to the stack.

## [[Recognition Purification|Recognition Purification]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="20.0,11.7 11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1 70.0,40.5" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:3.4s" points="20.0,11.7 11.7,26.1 28.3,26.1 36.7,40.5 53.3,40.5 61.7,26.1 53.3,11.7 36.7,11.7 28.3,26.1 45.0,26.1 61.7,26.1 78.3,26.1 70.0,40.5" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="20.0" y1="11.7" x2="17.3" y2="16.3" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="15.4,19.6 15.7,15.4 18.9,17.2" fill="#fc77be"/><circle cx="20.0" cy="11.7" r="2.6" fill="#fc77be"/></svg>

`linkable` → `num`

Removes an entity from the stack, and adds the index to the link to that entity to the stack (or -1 if the caster isn't linked to that entity).

## [[Popularity Reflection|Popularity Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="78.3,26.1 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 45.0,26.1 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:167;--dur:2.8s" points="78.3,26.1 61.7,26.1 53.3,40.5 36.7,40.5 28.3,26.1 36.7,11.7 53.3,11.7 61.7,26.1 45.0,26.1 28.3,26.1 11.7,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="78.3" y1="26.1" x2="73.0" y2="26.1" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="69.2,26.1 73.0,24.3 73.0,27.9" fill="#fc77be"/><circle cx="78.3" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Adds the number of links to the wisp to the stack.

## [[Send Iota|Send Iota]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="52" viewBox="0 0 90.0 52.2"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.3" cy="11.7" r="1.6"/><circle cx="20.0" cy="11.7" r="1.6"/><circle cx="36.7" cy="11.7" r="1.6"/><circle cx="53.3" cy="11.7" r="1.6"/><circle cx="70.0" cy="11.7" r="1.6"/><circle cx="86.7" cy="11.7" r="1.6"/><circle cx="11.7" cy="26.1" r="1.6"/><circle cx="28.3" cy="26.1" r="1.6"/><circle cx="45.0" cy="26.1" r="1.6"/><circle cx="61.7" cy="26.1" r="1.6"/><circle cx="78.3" cy="26.1" r="1.6"/><circle cx="3.3" cy="40.5" r="1.6"/><circle cx="20.0" cy="40.5" r="1.6"/><circle cx="36.7" cy="40.5" r="1.6"/><circle cx="53.3" cy="40.5" r="1.6"/><circle cx="70.0" cy="40.5" r="1.6"/><circle cx="86.7" cy="40.5" r="1.6"/></g><polyline class="hexpat-base" points="45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 53.3,11.7 61.7,26.1 53.3,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:200;--dur:3.4s" points="45.0,26.1 36.7,11.7 20.0,11.7 11.7,26.1 20.0,40.5 36.7,40.5 45.0,26.1 53.3,11.7 61.7,26.1 53.3,40.5 45.0,26.1 61.7,26.1 78.3,26.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="45.0" y1="26.1" x2="42.3" y2="21.5" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="40.4,18.2 43.9,20.6 40.7,22.4" fill="#fc77be"/><circle cx="45.0" cy="26.1" r="2.6" fill="#fc77be"/></svg>

`num, any` → `—`

Removes an iota from the stack, then removes an index from the stack and sends the iota along the link at the given index.  This costs one hundredth of an [[Amethyst|Amethyst Dust]].

## [[Recitation Reflection|Recitation Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,56.1 27.7,36.1 39.2,16.2 62.3,16.2 73.8,36.1 62.3,56.1 39.2,56.1 27.7,36.1 16.2,16.2" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="16.2,56.1 27.7,36.1 39.2,16.2 62.3,16.2 73.8,36.1 62.3,56.1 39.2,56.1 27.7,36.1 16.2,16.2" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="56.1" x2="19.8" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="22.5,45.1 22.0,51.0 17.6,48.5" fill="#fc77be"/><circle cx="16.2" cy="56.1" r="2.6" fill="#fc77be"/></svg>

`—` → `any`

When something receives an iota along a link it is added to a queue. This pattern adds the first iota in the queue to the stack, removing it from the queue.

## [[Postmaster's Reflection|Postmaster's Reflection]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 24.5,14.3 14.3,32.0 34.8,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:225;--dur:3.1s" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 24.5,14.3 14.3,32.0 34.8,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="17.6" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,41.8 15.6,38.8 19.5,36.6" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `num`

Adds to the stack the number of unread iota in the caster's received iota queue.

## [[Unclogging Gambit|Unclogging Gambit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 24.5,14.3 14.3,32.0" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:205;--dur:2.8s" points="14.3,32.0 24.5,49.7 34.8,32.0 45.0,14.3 65.5,14.3 75.7,32.0 65.5,49.7 45.0,49.7 34.8,32.0 24.5,14.3 14.3,32.0" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="14.3" y1="32.0" x2="17.6" y2="37.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="19.9,41.8 15.6,38.8 19.5,36.6" fill="#fc77be"/><circle cx="14.3" cy="32.0" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

Removes all unread iota from the caster's received iota queue.

## [[Open Transmit|Open Transmit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.6" cy="16.2" r="1.6"/><circle cx="27.7" cy="16.2" r="1.6"/><circle cx="50.8" cy="16.2" r="1.6"/><circle cx="73.8" cy="16.2" r="1.6"/><circle cx="16.2" cy="36.1" r="1.6"/><circle cx="39.2" cy="36.1" r="1.6"/><circle cx="62.3" cy="36.1" r="1.6"/><circle cx="85.4" cy="36.1" r="1.6"/><circle cx="4.6" cy="56.1" r="1.6"/><circle cx="27.7" cy="56.1" r="1.6"/><circle cx="50.8" cy="56.1" r="1.6"/><circle cx="73.8" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="73.8,16.2 50.8,16.2 39.2,36.1 27.7,56.1 16.2,36.1 27.7,16.2 39.2,36.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="73.8,16.2 50.8,16.2 39.2,36.1 27.7,56.1 16.2,36.1 27.7,16.2 39.2,36.1 50.8,56.1 73.8,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="73.8" y1="16.2" x2="66.5" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="61.2,16.2 66.5,13.6 66.5,18.7" fill="#fc77be"/><circle cx="73.8" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`num` → `—`

Similar to [[Introspection]], until drawing a [[Close Transmit]] all patterns I draw are sent along the link whose index is at the top of the stack.

## [[Close Transmit|Close Transmit]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="72" viewBox="0 0 90.0 72.3"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="16.2" cy="16.2" r="1.6"/><circle cx="39.2" cy="16.2" r="1.6"/><circle cx="62.3" cy="16.2" r="1.6"/><circle cx="85.4" cy="16.2" r="1.6"/><circle cx="4.6" cy="36.1" r="1.6"/><circle cx="27.7" cy="36.1" r="1.6"/><circle cx="50.8" cy="36.1" r="1.6"/><circle cx="73.8" cy="36.1" r="1.6"/><circle cx="16.2" cy="56.1" r="1.6"/><circle cx="39.2" cy="56.1" r="1.6"/><circle cx="62.3" cy="56.1" r="1.6"/><circle cx="85.4" cy="56.1" r="1.6"/></g><polyline class="hexpat-base" points="16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1 39.2,56.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:185;--dur:2.2s" points="16.2,16.2 39.2,16.2 50.8,36.1 62.3,56.1 73.8,36.1 62.3,16.2 50.8,36.1 39.2,56.1 16.2,56.1" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="16.2" y1="16.2" x2="23.5" y2="16.2" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="28.8,16.2 23.5,18.7 23.5,13.6" fill="#fc77be"/><circle cx="16.2" cy="16.2" r="2.6" fill="#fc77be"/></svg>

`—` → `—`

When drawn after [[Open Transmit]] I stop transmitting drawn patterns. If either [[Open Transmit]] or [[Close Transmit]] are executed by an intermediary caster such as a wisp, they will [[Mishaps|mishap]].

---
### Patterns on this page

- [[Link]] · Spell · Spell
- [[Link Others]] · Spell · Spell
- [[Unlink]] · Spell · Spell
- [[Unlink Others]] · Spell · Spell
- [[Phonebook Purification]] · Pattern · Purification
- [[Recognition Purification]] · Pattern · Purification
- [[Popularity Reflection]] · Pattern · Reflection
- [[Send Iota]] · Spell · Spell
- [[Recitation Reflection]] · Pattern · Reflection
- [[Postmaster's Reflection]] · Pattern · Reflection
- [[Unclogging Gambit]] · Pattern · Gambit
- [[Open Transmit]] · Pattern · Other
- [[Close Transmit]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://hexal.hexxy.media/v/0.3.1/1.0/en_us/#patterns/spells/links) · 🧩 [Hexal on Modrinth](https://modrinth.com/mod/aBVJ6Q36)
