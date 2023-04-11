---
layout: post
title: "Learning Rust for Use in AWS - Part 1"
categories: AWS, lambda, Rust, serverless
---

## Overview
Most of my development career has been spent in the .NET runtime, with recent
experience with Python and Go. Recently, I've heard more and more about the
performance benefits of the Rust programming language. I'm always looking
to stay on top of my learning game and to evolve within the software field,
so naturally, I wanted to determine if Rust would be worth learning now, and
how effectively I could implement it in my professional life.

## AWS
Most of my time is spent working within AWS, so natually my first step was to examine
the [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust). The first thing I noticed
is that currently, the SDK is advised to not be ready for critical production workloads, as
it is only in developer preview. Upon examining the [public roadmap](https://github.com/orgs/awslabs/projects/50/views/1),
however, it appears that there's only a handful of features left on the project board, which
makes me optimistic that the SDK will be ready for primetime in the near future.

As such, I would say it's easily safe to use Rust in an AWS environment as long as integrations with the AWS API are
handled by other, more mature and stable, SDKs. (boto3 for Python, etc.)

The next part that stood out for me is that Rust does not have an official supported Lambda runtime.
There does seem to be decent support and documentation for writing Rust lambdas using [Cargo Lambda](https://www.cargo-lambda.info/)
or the [AWS Lambda Rust Runtime](https://github.com/awslabs/aws-lambda-rust-runtime).

All things considered, it seems that Rust is almost ready for full-on AWS Primetime. Based on additional research, the low-level language
without garbage collection (ala C and C++) combined with the zero-cost abstraction of a higher-level language makes Rust seem like
a great addition to the Developer's toolbox, especially for performance-critical workloads.

## Learning
Okay, so now that I've decided that Rust is actually worth learning, what is my next step?
Rust has a great library of [documentation](https://www.rust-lang.org/learn) and ways to get started. My first step
is to roll up my sleeves and start reading [the book](https://doc.rust-lang.org/book/)

