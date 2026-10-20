---
aliases: ["Exchange Bridge"]
tags: [hexbook/entry, mod/hexwright, category/the-resonance-network]
mod: "Hexwright"
category: "The Resonance Network"
entry_id: "hexcasting:hex_wright/network/exchange_bridge"
---

# Exchange Bridge

> **Category:** Hex Wright › [[The Resonance Network (Category)|The Resonance Network]]  
> **Source mod:** [[Hexwright]]

### Exchange Bridge

*Item:* **Exchange Bridge**

Two networks, one shared channel.

Dock a [[Resonant Key]] from each of two towers, give it a harmonic channel, and they become linked. Messages published to a networks channel will appear on the other networks channel.

**Exchange Bridge**: shaped crafting

|   |   |   |
|---|---|---|
| `E` | `A` | `E` |
| `S` | `C` | `S` |
| `D` | `D` | `D` |

`E` = Ender Pearl · `A` = Amethyst Shard · `S` = Sculk · `C` = Kyanite · `D` = Polished Deepslate

Each bridge can link one channel across networks. It can be placed anywhere, even in another dimension. It's spatial properties mean it doesn't have to exist within the 48 ambit of a [[Resonance Tower]].

### Two Keys, Two Towers

The keys must be attuned to different towers. A bridge from a network to itself gives you no benefit. Luckily right clicking the bridge holding anything but a [[Resonant Key|resonant key]] will show open a summary that highlights these errors. You can also add keys in this menu or right click the bridge with a key to place it.

### Chaining

Bridges can be chained up to 4 hops. That's four different networks that can reached on one channel. Beyond that the messages are dropped quietly.
Bridges are also loop safe. Messages won't repeat on the same network if two or more bridges are connected in a loop.

A bridge doesn't require the chunk its in to be loaded. It will function fine in any location.
Small note on the summary display shown when right clicking a bridge, the summary message count doesn't include any messages recieved whe the bridge was unloaded. It is more a diagnostic tool than an accurate count.

### Tuning

The Harmonic Channel that is bridged can be tuned using [[Harmonic Transducer|Tuner's Gambit]]. Using null with that pattern will disconnect the bridge.


**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
