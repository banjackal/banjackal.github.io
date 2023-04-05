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
