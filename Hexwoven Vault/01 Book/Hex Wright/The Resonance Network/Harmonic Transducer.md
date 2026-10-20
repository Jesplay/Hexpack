---
aliases: ["Harmonic Transducer"]
tags: [hexbook/entry, mod/hexwright, category/the-resonance-network, topic/hexwright]
mod: "Hexwright"
category: "The Resonance Network"
entry_id: "hexcasting:hex_wright/network/harmonic_transducer"
---

# Harmonic Transducer

> **Category:** Hex Wright › [[The Resonance Network (Category)|The Resonance Network]]  
> **Source mod:** [[Hexwright]]

**Harmonic Transducer** ×2: shaped crafting

|   |   |   |
|---|---|---|
|   | `S` |   |
| `A` | `C` | `A` |
| `D` | `D` | `D` |

`S` = Sculk Sensor · `A` = Amethyst Shard · `C` = Kyanite · `D` = Polished Deepslate

The counterpart to the [[Harmonic Emitter]]. A Harmonic Transducer attaches to block and sends a signal when a monitored property changes. It's a limited caster with an ambit of 4 and a manual [[Media|media]] store.

### Stick It To Something

A Transducer clips to the face of whatever you place it against, like a torch, and monitors for changes.

It takes a [[Resonant Key]] the same way an Emitter does and must be within 48 blocks of a [[Resonance Tower]]

### Configuring

Right click a transducer to configure what property of the block it's attached to to monitor and trigger on change.

There's currently no hex to configure a transducer as the trigger properties are dynamic. Emitter

Two triggers built in triggers:

Any Change fires when the state changes at all, and sends the position where it happened.

Comparator is equivalent to redstone comparator signal. It is the coarse, universal way to watch contents like a chest filling, or a furnace's progress.

### Transmission

What the transducer sends over it's conifgured channel depends on what it is monitoring. It could send a boolean, a number or a string.
Experimentation and reveal are your best friends here.

## [[Tuner's Gambit|Tuner's Gambit]] ✦ *Spell*

<svg class="hexpat small" xmlns="http://www.w3.org/2000/svg" width="90" height="73" viewBox="0 0 90.0 73.4"><g fill="#8c8c99" fill-opacity="0.35"><circle cx="3.7" cy="12.9" r="1.6"/><circle cx="22.0" cy="12.9" r="1.6"/><circle cx="40.4" cy="12.9" r="1.6"/><circle cx="58.8" cy="12.9" r="1.6"/><circle cx="77.1" cy="12.9" r="1.6"/><circle cx="12.9" cy="28.8" r="1.6"/><circle cx="31.2" cy="28.8" r="1.6"/><circle cx="49.6" cy="28.8" r="1.6"/><circle cx="68.0" cy="28.8" r="1.6"/><circle cx="86.3" cy="28.8" r="1.6"/><circle cx="3.7" cy="44.7" r="1.6"/><circle cx="22.0" cy="44.7" r="1.6"/><circle cx="40.4" cy="44.7" r="1.6"/><circle cx="58.8" cy="44.7" r="1.6"/><circle cx="77.1" cy="44.7" r="1.6"/><circle cx="12.9" cy="60.6" r="1.6"/><circle cx="31.2" cy="60.6" r="1.6"/><circle cx="49.6" cy="60.6" r="1.6"/><circle cx="68.0" cy="60.6" r="1.6"/><circle cx="86.3" cy="60.6" r="1.6"/></g><polyline class="hexpat-base" points="31.2,28.8 22.0,44.7 31.2,60.6 49.6,60.6 58.8,44.7 49.6,28.8 31.2,28.8 12.9,28.8 22.0,12.9 31.2,28.8 40.4,12.9 49.6,28.8 58.8,12.9 68.0,28.8 77.1,12.9" fill="none" stroke="#b38ef3" stroke-opacity="0.35" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><polyline class="hexpat-trace" style="--len:257;--dur:3.9s" points="31.2,28.8 22.0,44.7 31.2,60.6 49.6,60.6 58.8,44.7 49.6,28.8 31.2,28.8 12.9,28.8 22.0,12.9 31.2,28.8 40.4,12.9 49.6,28.8 58.8,12.9 68.0,28.8 77.1,12.9" fill="none" stroke="#b38ef3" stroke-width="3.2" stroke-linejoin="round" stroke-linecap="round"/><line x1="31.2" y1="28.8" x2="28.3" y2="33.9" stroke="#fc77be" stroke-width="1.9" stroke-linecap="round"/><polygon points="26.2,37.5 26.5,32.8 30.0,34.9" fill="#fc77be"/><circle cx="31.2" cy="28.8" r="2.6" fill="#fc77be"/></svg>

`vector, number | null` → `—`

Tells the block at that position which harmonic channel to transmit on. Null resets channel.
Costs 1 Dust

---
### Patterns on this page

- [[Tuner's Gambit]] · Spell · Gambit

**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
