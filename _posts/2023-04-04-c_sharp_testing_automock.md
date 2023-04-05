---
layout: post
title: Leveraging AutoMock and Moq for C# Unit Tests
categories: unit testing, C#, Dotnet
---

AutoMock and Moq are some great .NET libraries that can help streamline your unit testing with Xunit or NUnit, once you understand the common pattern.

In numerous shops, I've found myself using a similar base test class pattern to quickly ramp up my unit testing.

The following example is for Xunit:
```csharp
using Moq;
using Moq.AutoMock;

public class SpecBase<I,T> where T : class, I
{
    protected AutoMocker mocker;
    protected I sut; //"sut" is an abbreviation for "System Under Test"
    
    private void init()
    {
        setup();
        mocker = new AutoMocker();
        sut  = mocker.CreateInstance<T>();
        arrange();
        act();
    }

    public SpecBase()
    {
        init();
    }
        
    protected virtual void setup(){}
    protected virtual void arrange(){}
    protected virtual void act(){}
}
```

Now that we have a base testing class, we can use it to rapidly spin up tests and automocks for any class that implements an interface, like our "Bar" class:
```csharp
public interface IBar 
{
    string GetValue();
}

public class Bar : IBar 
{
    private IFoo _foo {get; set;}

    public Bar(IFoo foo)
    {
        _foo = foo;
    }

    public string GetValue() 
    {
        var fooString = _foo.GetValue();
        return $"Foo string is {fooString}";
    }
}
```

Which has a dependency on our `IFoo` interface:
```csharp
public interface IFoo {
    string GetValue();
}
```

The real magic comes from the automocker, which automatically creates mocks for all of the interfaces that our system under test depends on. Now, in order to mock the behavior, all we have to do is `Setup` the mock, like we would do oon any Mock object:
```csharp
using Xunit;

namespace tests;

public class Testing_Bar{
    public class Using_Bar : SpecBase<IBar, Bar>{}

    public class When_Foo_Returns_Something_Mocked : Using_Bar 
    {
        protected const string VALUE = "Some Mocked Value";
        protected string result;

        protected override void arrange()
        {
            mocker.Setup<IFoo, string>(x=> x.GetValue()).Returns(VALUE);
        }

        protected override void act()
        {
            result = sut.GetValue();
        }

        [Fact]
        public void Should_return_a_string()
        {
            Assert.Equal($"Foo string is {VALUE}", result);
        }
    }

}
```

Note how the `arrange()` method is overridden here. Now we can set up our mocks and call our system under test once, and all of our assertions can run against one block of code to set up our dependency mocks and stubs.

Now, if we're using NUnit instead, the base class is going to look a little different. Most notably, the constructor we used in the Xunit base has been replaced by attributing the `init()` method with NUnit's `SetUp` attribute. This behaves similarly to a constructor in that it runs our virtual methods and creates our AutoMocker and System Under Test for each test method. Also note that our `init()` method can no longer be public so NUnit can run it.

```csharp
using NUnit.Framework;
using Moq;
using Moq.AutoMock;

namespace NUnitSpecBase;

public class SpecBase<I,T> where T : class, I{
    protected AutoMocker mocker;
    protected I sut;

    [SetUp]
    protected void init(){
        setup();
        mocker = new AutoMocker();
        sut  = mocker.CreateInstance<T>();
        arrange();
        act();
    }

    protected virtual void setup(){}
    protected virtual void arrange(){}
    protected virtual void act(){}
}
```

Our implementation again looks similar, the only real change is that NUnit uses the `Test` attribute instead of `Fact` and we need to use `Assert.AreEqual()` instead of `Assert.Equal()`

```csharp
using NUnit.Framework;
using NUnitSpecBase;

namespace NUnitTests;

public class Testing_Bar{
    public class Using_Bar : SpecBase<IBar, Bar>{}

    public class When_Foo_Returns_Something_Mocked : Using_Bar {
        protected const string VALUE = "Some Mocked Value";
        protected string result;
        protected override void setup(){}
        protected override void arrange(){
            mocker.Setup<IFoo, string>(x=> x.GetValue()).Returns(VALUE);
        }
        protected override void act(){
            result = sut.GetValue();
        }

        [Test]
        public void Should_return_a_string(){
            Assert.AreEqual($"Foo string is {VALUE}", result);
        }
    }

}
```
