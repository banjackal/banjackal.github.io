---
layout: post
title:  "Exploration Self-hosting a Security Camera System, Pt 1"
categories: smart home, surveillance, self-hosting
---

## Overview
We recently moved to a 15 acre property, complete with a pole barn which is housing our workshop and an outdoor cat. We wanted to set up a security camera system, and as a technology professional, I wanted to host the camera data myself, especially after reading about just how many of these security camera brands have serious security flaws. My plan was to utilize only IP cameras and to have them feed directly to a server hosted on premises.
## Initial Testing
After reading and reading and reading, I decided to try [ZoneMinder](https://cloud.zoneminder.com/) as my camera system server, and began testing IP cameras with the [Reolink RLC-520A](https://reolink.com/product/rlc-520a/).
ZoneMinder is free and open-source, and easily testable within a Docker container.
The Reolink camera was only about $50 and can be powered via Power Over Ethernet (PoE)
I also acquired a [Netgear GS308EPP 8-Port Gigabit PoE+ Compliant Managed Switch](https://www.netgear.com/business/wired/switches/plus/gs308epp/) to connect the camera and deliver PoE.
### First Impressions
After acquiring all the gear, I plugged the camera in to the PoE switch and attached it to my home network.
I quickly spun up ZoneMinder using a docker container and used the following [guide](https://www.reddit.com/r/reolinkcam/comments/jnurzm/adding_reolink_cameras_to_zoneminder_nvr/) to set up the Reolink camera

___NOTE:___ For initial testing, I specifically set the camera to "monitor" mode so it wouldn't be saving video to disk

After getting a camera up and running, by old 4 core intel i5-2500k was steadily running at about 15% (an overall increase of about 12% from it's standard load)

___NOTE:___ I'm sharing this hardware with my media server, so in order to expand the system, I'll probably need to get another dedicated box to host ZoneMinder, or at least transfer my media server to a cloud solution. I'm anticipating that with 8-12 cameras, I may need more CPU as well
