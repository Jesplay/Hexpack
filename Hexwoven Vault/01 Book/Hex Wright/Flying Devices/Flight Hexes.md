---
aliases: ["Flight Hexes"]
tags: [hexbook/entry, mod/hexwright, category/flying-devices]
mod: "Hexwright"
category: "Flying Devices"
entry_id: "hexcasting:hex_wright/flying_devices/flight_hexes"
---

# Flight Hexes

> **Category:** Hex Wright › [[Flying Devices (Category)|Flying Devices]]  
> **Source mod:** [[Hexwright]]

### Flight Hexes

Write the flight hex with Scribe's or [[Chronicler's Gambit]] before you mount as there's no way to edit it once you're in the air.

You can either a throttle vector or a \[throttle vector, \[pattern for memory\]\] on the [[The Stack|stack]].

### Throttle

That vector is a throttle, not a speed: a direction, with a length between 0 and 1.

1 is all the acceleration you have, 0.5 is half of it. Your vehicle multiplies it by its own max acceleration, so no seperate hexes for different speed brooms or carpets.

Ask for more than 1 and it is simply shortened back to 1. So (1,0,1) and (2,0,2) are the same request: full power, south-east.

Return (0,0,0) to coast. A light drag bleeds speed off slowly so to break efficiently you point the throttle behind you.

The throttle points in world directions, not your own. That is what Facing and Rightwards are for: they are flat, length 1, and already turned the way you are looking.

Returning Facing alone is full throttle straight ahead.

### Cost

Flying costs [[Media|media]].

You pay once each time the hex runs: a little just to hover, more the faster you go, a little more to climb, and most of all for abrupt accelerations

Around half speed covers the most ground per dust.

24 dust, about five shards, gets the following:
Hovering: 3 minutes
Half speed: 90s, 840 blocks
Flat out: 35s, 660 blocks
Climbing hard: 25s, 210 blocks up
Overspeeding: 1 second

Finer brooms fly faster on a bigger tank for the same cost per second.

### Summary

The broom wants a throttle on the stack at the end as the vector value: which way, and how hard?

The vector you return is the which way and the value of each component is how hard, a 0 is nothing and 1 is every bit of acceleration the broom or carpet has.

Think of it like a joystick, you can move it in any direction but can't push past the limit. And it's a circle, so diagonal is no further from centre than straight ahead is. This is how the return vector is capped.
(1,0,1) which is diagonal, is no faster than (1,0,0) which is straight ahead.

The one rule: you can't ask to go faster when you're already at top speed. Full throttle is perfectly safe at lower speeds but will take you over top speed at higher speeds.
The question you should answer with a heix is 'how hard should I push' not 'how fast do I want to go'. You're adding force to push, not setting a speed exactly.

### Simple Hex Example

Take your ground speed. If that plus max acceleration still fits under max horizontal speed, return Facing for full throttle ahead. Otherwise return (0,0,0) and coast.

That is a complete flight hex and it never overspeeds. It does surge, though.

### Overspeed

Push past your limits and the broom drinks media, roughly x30 the usual, and your speed readout turns red.

There's a small grace period of three hex executions and after that the penalties will begin.
Vehicles hold their own media, and start drawing from the rider once that runs out.

### When It Mishaps

Return anything that is neither a vector nor \[vector, list\], a memory that is not a list, or an ordinary Hex [[Mishaps|mishap]] inside your own hex.

Any of those converts the vehicle back to an item and drops you.

### Simple Smoother Flight

The simple hex surges: accelerates flat out, hits its threshold, coasts until drag drops it back then accelerates flat out again.

For smooth flight, build the velocity you want, subtract the velocity you have, and divide by max acceleration.

### Example

want = Facing x (forward input x max horizontal)
+ Rightwards x (strafe input x max horizontal)
+ (0,1,0) x (climb input x max vertical)

Then (want - Velocity) / max acceleration. It steers, brakes, climbs, and can never overspeed.


**Sources:** 🧩 [Hexwright on Modrinth](https://modrinth.com/mod/DXkdjl8n)
