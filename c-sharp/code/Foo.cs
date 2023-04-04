public interface IFoo {
    string GetValue();
}

public class Foo : IFoo {
    public Foo(){}

    public string GetValue(){
        return "Foobar";
    }
}
