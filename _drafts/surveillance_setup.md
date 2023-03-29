# Self-hosting a Security Camera System
## Overview
We recently moved to a 15 acre property, complete with a pole barn which is housing our workshop and an outdoor cat. We wanted to set up a security camera system, and as a technology professional, I wanted to host the camera data myself, especially after reading about just how many of these security camera brands have serious security flaws. My plan was to utilize only IP cameras and to have them feed directly to a server hosted on premises.
## Initial Testing
After reading and reading and reading, I decided to try [ZoneMinder](https://cloud.zoneminder.com/) as my camera system server, and began testing IP cameras with the [Reolink RLC-520A](https://reolink.com/product/rlc-520a/).
ZoneMinder is free and open-source, and easily testable within a Docker container.
The Reolink camera was only about $50 and can be powered via Power Over Ethernet (PoE)
I also acquired a [Netgear GS308EPP 8-Port Gigabit PoE+ Compliant Managed Switch](https://www.netgear.com/business/wired/switches/plus/gs308epp/) to connect the camera and deliver PoE.
