---
layout: post
title: Polymorphism with Type Safety in Python
categories: python, object-oriented, polymorphism, type-safety
---

## Overview
Polymorphism in object oriented programming is an effective way to reduce code
duplication and isolate behaviors. Without enforcing a type system in Python,
however, attempting to implement polymorphism can lead to a headache of runtime
errors. By instead enforcing static typing, we can rely on our tooling to detect
errors ahead of time.
For our example, we're going to create a new file `polymorphism.py` and add the
following fuction, which will call a method on an input object and print it to
the console
```python
def print_name(obj):
    print(obj.get_name())
```

## Runtime errors
Since static types are not enforced in Python by default, we can define the
following class:

```python
class SomeClass():
    def foo(self):
        return 'bar'
```

Now, we can create an instance of this class and pass it into our function
```python
if __name__ == '__main__':
    some_object = SomeClass()
    print_name(some_object)
```
Our tooling (I'm using pyright as a language server) will not detect any issues
but running the resulting file will cause an `AttributeError`
```bash
$ python polymorphism.py
   
Traceback (most recent call last):
  File "/polymorphism.py", line 11, in <module>
    print_name(some_object)
  File "/polymorphism.py", line 2, in print_name
    print(obj.get_name())
AttributeError: 'SomeClass' object has no attribute 'get_name'
```

We can leverage some static typing to help us catch issues like this before we run
our code by adding some static typing

## Static Typing
Let's first create a new base class at the top of our file which will have a method `get_name`
```python
class BaseClass():
    def __init__(self):
        self.name = 'Base class'

    def get_name(self) -> str:
        return self.name
```

We already see some static typing here, `get_name` is returning a string

Now, we can update the `print_name` function to require an input parameter of type `BaseClass`.
By requiring this type, we now can guarantee that obj will now have the `get_name` method.
```python
def print_name(obj: BaseClass):
    print(obj.get_name())
```

If our language server is working properly, we should now see that our last line `print_name(some_object)` is giving us an error message

```
Argument of type "SomeClass" cannot be assigned to parameter "obj" of type
	"BaseClass" in function "print_name"
		"SomeClass" is incompatible with "BaseClass" (PyrightreportGeneralTypeIssues)
```

## Adding Inheritance
Now, in order to fix our error, we will edit `SomeClass` inherit the default behavior from `BaseClass`
Because `SomeClass` implements `BaseClass`, our language server will no longer complain because the classes are now compatible
```python
class SomeClass(BaseClass):
    def foo(self):
        return 'bar'
```

Running our file will print the value of `get_name` we defined in `BaseClass`
```
$ python polymorphism.py

Base class
```

## Changing Behavior Using Polymorphism
With this static typing in place, we can define different implementations of the `BaseClass` which are all compatible with our `print_name` function. Let's edit `SomeClass` again
```python
class SomeClass(BaseClass):
    def __init__(self):
        self.name = "I'm a child class"
```

Let's also add another invocation of `print_name` to run on an instance of our generic class
```python
if __name__ == '__main__':
    some_object = SomeClass()
    print_name(some_object)

    some_base_object = BaseClass()
    print_name(some_base_object)
```

Running our file once more, we can see we now have different results for each run of `print_name`
```
$ python polymorphism.py

I'm a child class
Base class
```

## Recap
Python's optional type system can be used to enforce good coding practice and help enable good object-oriented concepts, like polymorphism.
Static typing helps detect potential bugs early in the development process.
For more information on when and why to use polymorphism, you can review the following links

<https://stackify.com/oop-concept-polymorphism/>

<https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/object-oriented/polymorphism>

<https://stackoverflow.com/questions/1031273/what-is-polymorphism-what-is-it-for-and-how-is-it-used>
