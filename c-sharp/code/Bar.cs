public interface IBar {
    string GetValue();
}
public class Bar : IBar {
    private IFoo _foo {get; set;}

    public Bar(IFoo foo){
        _foo = foo;
    }

    public string GetValue() {
        var fooString = _foo.GetValue();
        return $"Foo string is {fooString}";
    }
}
