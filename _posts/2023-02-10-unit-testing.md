---
layout: post
title:  "Unit Testing, a Practical Approach"
categories: unit testing, theory
---
I'm operating on the assumption that you are working in a language you are familiar with and already have an understanding of how to write unit tests and what differentiates a unit test from another type of test, like an integration or a UI test. If you're not familiar with these concepts, fear not. There's a lot of great resources out there to get you up to speed.

## Overview
In an enterprise environment, writing automated tests is not optional. You have to write and maintain a test suite to ensure your code continues to work as the codebase evolves with new features and bug fixes. 

Many times, I've witnessed agile teams write tests for the wrong reasons, namely to increase a code coverage percentage metric without thinking about what actual benefit to the software development lifecycle they provide.

### Why We Should Not Write Tests
Do not write tests to show off how much code coverage you have. The code coverage metric is not an accurate representation of how good your tests are. In enterprise technology, there's a lot of pressure to use quantitative metrics to measure just about everything that a devloper does, so there's usually pressure to get the code coverage report up above 80% or 90%, because it's something that _can_ be measured.

Do not write tests to 'prove' that your code works.

In practice, you can cover a lot of code without actually testing anything. When the pressure is to meet test coverage, it's very easy to find the most uncovered lines that can be covered with the least amount of effort. While sometimes these test might not necessarily be bad to have, they waste effort that could be used writing a much more practical test.

### Why We Should Write Tests
As a developer, you should be focusing your testing effort on the critical pieces of your code. Think about where decisions are being made, and what situations are going to impact your output. Essentially, focus on testing your business logic first.

Writing the important tests first, without worrying about achieving a particular coverage percentage, will help get you in the right state of mind to write effective tests. More often than not, writing more tests to cover different behaviors will cover the same lines of code you covered in previous tests, but in a dynamic application with a lot of dependencies, these test will prove to be more valuable to maintaining a robust system. Your code is going to change and it's going to change often.

I'm a firm believer that 

Especially when working in an active, collaborative repository, you have to ensure that your work isn't being undone accidentally by other developers. We are generally not a malicious bunch, but when I'm in the weeds working on a feature, it's very difficult to see past the task at hand to ensure that I'm not inadvertently breaking the feature we shipped in the last sprint. If I have a robust suite of tests that cover the _behavior_ of that feature, not the lines of code added in that feature, I can confidently rely on my test suite to ensure I'm not breaking that functionality.

### Bug Fixes
Yes, there's going to be bugs. Yes, they will be your fault. Yes, you will have to fix them.

This is where I'm a fan of Test Driven Development (TDD). Sometimes you will work on features that are simple enough to write the tests before the code, but in a fast paced Agile environment, there's usually a decent amount of discovery that has to happen during development that makes true TDD practical. When a nasty bug rears its head, it's usually the ideal time to first write a test that reproduces the bug.

Spend the time to write the test. Understand the bug and how to reproduce it. After writing the test (or tests) that cover the bug, you'll have a better understanding of the problem when it comes time to write the code.

### Conclusion
Unit tests are one of the best tools in a devleoper's toolbox to write and refactor code effectively. It ensures our hard work continues to do what it's supposed to do. Your boss will want a percentage of lines covered, but your colleagues and your future self will want effective tests.

Take the time to thing about what you're testing and why you're testing it.

#### Some Unsolicited related posts
 [Martin Fowler](https://martinfowler.com/bliki/UnitTest.html) provides a great notion of solitary vs sociable tests. I'm strongly of the opinion that both have their place in your test suite.

You can also find a much more in-depth article from [Ham Vocke](https://martinfowler.com/articles/practical-test-pyramid.html#UnitTests) on Martin's site.
