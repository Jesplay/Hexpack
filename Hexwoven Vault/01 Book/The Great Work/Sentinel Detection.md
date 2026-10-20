---
aliases: ["Sentinel Detection", "Sentinel Sensor", "Seer Impetus"]
tags: [hexbook/entry, mod/oneironaut, category/the-great-work, topic/sentinels]
mod: "Oneironaut"
category: "The Great Work"
entry_id: "hexcasting:greatwork/sentinel_detection"
---

# Sentinel Detection

> **Category:** [[The Great Work (Category)|The Great Work]]  
> **Source mod:** [[Oneironaut]]

By combining the noosphere's pseudoamethyst with the organic sensory apparatus native to the deep dark, I've discovered a way to detect fluctuations in ambient [[Media|media]]. This new form of sensor detects the presence of [[Sentinels]] within approximately 16 meters, and outputs a redstone signal based on its proximity to the closest one. Comparators attached to it will output a signal if the sentinel in question is particularly powerful.

**Sentinel Sensor**: shaped crafting

|   |   |   |
|---|---|---|
| `B` | `B` | `B` |
| `P` | `S` | `P` |
| `B` | `B` | `B` |

`B` = Block of Slate · `P` = Pseudoamethyst Block · `S` = Sculk Sensor

*I'm sure the warden won't mind.*

Additionally, this sensor can be incorporated into a Cleric Impetus, allowing it to activate when a sentinel is placed in the area (albeit with a lower range than the plain sensor). I've dubbed this contraption a Seer Impetus. Though it no longer responds to redstone, the circle's hex will start with a reference to the owner of the triggering sentinel on the [[The Stack|stack]].
Due to the media interference produced by the cast, the impetus cannot detect sentinels it places itself.

**Seer Impetus**: shaped crafting

|   |   |   |
|---|---|---|
|   | `S` |   |
| `B` | `P` | `B` |
| `B` | `C` | `B` |

`S` = Sentinel Sensor · `B` = Block of Slate · `P` = Shifting Pseudoamethyst · `C` = Cleric Impetus

As the sensor does not care whose sentinel it sees, I should take care to avoid allowing my trap circles to target me.

## [[Scrutinize Sentinel|Scrutinize Sentinel]]

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="64" viewBox="0 0 90.0 64.1"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="4.1" cy="14.3" r="1.6"/><circle cx="24.5" cy="14.3" r="1.6"/><circle cx="45.0" cy="14.3" r="1.6"/><circle cx="65.5" cy="14.3" r="1.6"/><circle cx="85.9" cy="14.3" r="1.6"/><circle cx="14.3" cy="32.0" r="1.6"/><circle cx="34.8" cy="32.0" r="1.6"/><circle cx="55.2" cy="32.0" r="1.6"/><circle cx="75.7" cy="32.0" r="1.6"/><circle cx="4.1" cy="49.7" r="1.6"/><circle cx="24.5" cy="49.7" r="1.6"/><circle cx="45.0" cy="49.7" r="1.6"/><circle cx="65.5" cy="49.7" r="1.6"/><circle cx="85.9" cy="49.7" r="1.6"/></g><polyline class="hexpat-base" points="24.5,49.7 45.0,49.7 65.5,49.7 55.2,32.0 65.5,14.3 45.0,14.3 24.5,14.3 34.8,32.0 24.5,49.7 14.3,32.0 34.8,32.0 55.2,32.0 75.7,32.0 65.5,49.7" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:266;--dur:3.6s" points="24.5,49.7 45.0,49.7 65.5,49.7 55.2,32.0 65.5,14.3 45.0,14.3 24.5,14.3 34.8,32.0 24.5,49.7 14.3,32.0 34.8,32.0 55.2,32.0 75.7,32.0 65.5,49.7" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="24.5" y1="49.7" x2="31.1" y2="49.7" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="35.8,49.7 31.1,52.0 31.1,47.5" fill="#fc77be"/><circle cx="24.5" cy="49.7" r="2.6" fill="#fc77be"/></svg>

`vec, player` → `num | NULL`

Accepts a vector and a player, and returns either the distance from the vector to the target player's sentinel or NULL. Costs a negligible amount of media.

Conjures a fleeting sensor at the target coordinates, which "listens" to the patterns in the ambient media to seek out the telltale signature of a sentinel.

---
### Patterns on this page

- [[Scrutinize Sentinel]] · Pattern · Other

**Sources:** 🌐 [Official online docs](https://oneironaut.hexxy.media/v/0.4.0/1.0/en_us/#greatwork/sentinel_detection)
