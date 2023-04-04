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
