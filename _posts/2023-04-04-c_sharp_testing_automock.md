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

public class SpecBase<I,T> where T : class, I{
    protected AutoMocker mocker;
    protected I sut;
    private void init(){
        setup();
        mocker = new AutoMocker();
        sut  = mocker.CreateInstance<T>();
        arrange();
        act();
    }

    public SpecBase(){
        init();
    }
        
    protected virtual void setup(){}
    protected virtual void arrange(){}
    protected virtual void act(){}
}
```

I'll have the implementation example up shortly, but in the meantime, here you go, Dave :)
