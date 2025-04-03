---
layout: post
title: Hosting a Synapse server on AWS
description: Using Terraform to host a Synapse secure messaging server
categories: Encryption, Messaging, AWS, self-hosted, Terraform
---

# Overview

I use a handful of messaging platforms for daily communication: in-app messaging, Discord, SMS/RCS. I know a lot of people use additional applications,
like Signal, WhatsApp, Telegram, Snapchat

In an age when we send an unbelievable amount of data out to the world, we sometimes want to have more control over what our data is doing. In a lot of those cases (but not always) we can use open source programs to maintain our own data, instead of feeding it to Software-as-a-service (SaaS) providers, or by using a free client
application with servers we don't own.

I have been using Jellyfin to host my own media, so I figured, why not host a messaging platform that I can use to communicate with friends and family securely.
From my initial research, I decided to host a Synapse server, and will try various [Matrix](https://matrix.org/) client applications. The beauty of Matrix is
that users can use various client apps to connect, instead of one proprietary web or mobile application. Before delving into which client application to use,
I want to get my own managed server running.

## Considerations

Originally, I was going to use [Conduit](https://conduit.rs), but the default setup to use RocksDB or SQLite made
it a non-starter for my desired architecture. Both these database systems are bundled with the application, making
it difficult to independently scale the compute server and the persistant data. Since Synapse uses PostgreSQL,
I can easily set up my database in AWS RDS or Aurora, and have separate management for my data layer.

# Architecture

The most straightforward approach will be to run the server in an EC2 instance, behind an application load balancer, using Route 53 for DNS resolution. I'm going to use a domain name that I already own, so I don't need to provision a new one.

//TODO Add an architecture diagram

The main benefit of hosting this server in AWS instead of on my home network is that I do not have to open my own private home network to public internet traffic.

Yes, I could set up a separate subnet, even an airgapped one on a different router, but for the sake of simplicity, I want to isolate this in a cloud VPC, completely separate from my home network to prevent my personal devices and data from being targets for bad actors.

## Autoscaling

For the sake of simplicity, I don't need to run this in containers, but I
definitely could, using ECS. Using an autoscaling group (ASG) of small EC2
instances will still give me scaling flexibility without having to manage
separate scaling of individual tasks as well as task nodes. For my
development, I'll simply use an ASG that keeps 1 EC2 running. This will
give me the flexibility to add scaling later if my server can't handle
the initial load of a small handful of group chats.

## Networking

All of the components will be isolated in a VPC for the application.
This keeps the environment segregated from the rest of my AWS
resources in the account, and will enable me to keep "prod" and
"dev" in the same account with logical segregation of resources
and data.

Each component will live in a separate subnet, for additinal security.

The ALB will be the only component exposed in a public subnet. The
compute and data layers will each have their own separate subnet.
This is usually not necessary, and many times just having a public and
a private subnet is sufficient, but havig a separate subnet for the
data layer is helpful to restrict network traffic that isn't coming
from the application, via NACLs.

## Database

Synapse uses PostgreSQL. For simplicity and autoscaling, I am going to use
AWS Aurora. If I can discern a predictable load and usage patterns, I'll
explore switching over to pure RDS and manage my compute resources myself.

