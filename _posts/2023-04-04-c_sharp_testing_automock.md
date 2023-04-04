---
layout: post
title: Leveraging Automock and Moq for C# Unit Tests
categories: unit testing, C#, Dotnet
---

Automock and Moq are some great .NET libraries that can help streamline your unit testing with XUnit or NUnit, once you understand the common pattern.

In numerous shops, I've found myself using a similar base test class pattern to quickly ramp up my unit testing.

The following example is for XUnit:
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
            Assert.Equal(result, $"Foo string is {VALUE}");
        }
    }

}
```

Note how the `arrange()` method is overridden here. Now we can set up our mocks and call our system under test once, and all of our assertions can run against one block of code to set up our dependency mocks and stubs.
