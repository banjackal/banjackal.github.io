---
layout: post
title: Building an OCR webapp
description: Building An OCR web server to extract text from images
categories: OCR, Go, React
---

As a part of a data collection experiment, I wanted to see how effectively an OCR server could extract text data from images, so that I could build more automated data collection for my application.

# Frontend

I decided to go with React for the frontend, becauese I'm not a frontend developer, and it was all the rage 5 years ago!


# Web Server

I started trying to use a Rust server with Rocket. However, I just about pulled my hair out trying to get the multipart form data to send properly from my React front end. As a result, I pivoted and built my simple web server with Go.

