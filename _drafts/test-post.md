---
layout: post
title:  "Modular Testing Strategies in AWS"
categories: test
---
## Overview
In an enterprise environment, writing automated tests is not optional. You have to write and maintain a test suite, not to prove that your code works, but to ensure your code continues to work.

Many times, I've witnessed agile teams write tests for the wrong reasons, namely to increase a code coverage percentage metric without thinking about what actual benefit to the software development lifecycle they provide.

### Why We Should Write Tests
Do not write tests to show off how much code coverage you have. The code coverage metric is not an accurate representation of how good your tests are. In enterprise technology, there's a lot of pressure to use quantitative metrics to measure just about everything that a devloper does, so there's usually pressure to get the code coverage report up above 80% or 90%, because it's something that _can_ be measured.

In practice, you can cover a lot of code without actually testing anything. Take, for example, the following python function:
```python
def print_info(input):
    print(input)
    return input
```
Comments aside about how useless the function is. We can easily add code coverage by writing a test function:
```python
def print_info_test():
    result = print_info('some input')
    assert result == 'some input'
```

Now, we've tested a function, and our code coverage will be ever so slightly higher, but what did we really accomplish? What assurances do we have? What use do we actually gain from testing this function?

We don't gain anything from testing this function. It does not give us any confidence to make changes, it only pads numbers.

As a developer, you should be focusing your testing effort on the critical pieces of your code. Think about where decisions are being made, and what situations are going to impact your output.Essentially, focus on testing your business logic first. Writing the important tests first, without worrying about achieving a particular coverage percentage, will help get you in the right state of mind to write effective tests.

>When the user requests a widget, but today is Saturday, they shouldn't get a widget, they should get a warning


