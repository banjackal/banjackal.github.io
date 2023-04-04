using Xunit;

namespace tests;

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

        [Fact]
        public void Should_return_a_string(){
            Assert.Equal(result, $"Foo string is {VALUE}");
        }
    }

}
