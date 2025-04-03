var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (Object.prototype.hasOwnProperty.call(b, p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        if (typeof b !== "function" && b !== null)
            throw new TypeError("Class extends value " + String(b) + " is not a constructor or null");
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var Foo = /** @class */ (function () {
    function Foo(foo, bar) {
        this.foo = foo;
        this.bar = bar;
    }
    Foo.prototype.get_bar = function () {
        try {
            return this.bar.get_error();
        }
        catch (error) {
            return this.do_something_else();
        }
    };
    Foo.prototype.set_bar = function (bar) {
        return this.bar.set(bar);
    };
    Foo.prototype.do_something = function () {
        return "something";
    };
    Foo.prototype.do_something_else = function () {
        try {
            throw Error("error");
        }
        catch (_a) {
            return this.do_something();
        }
    };
    return Foo;
}());
var Bar = /** @class */ (function () {
    function Bar() {
    }
    Bar.prototype.set = function (bar) {
        this.bar = bar;
    };
    Bar.prototype.get_error = function () {
        throw new Error("nope");
    };
    Bar.prototype.get = function () {
        return this.bar;
    };
    return Bar;
}());
var FooBar = /** @class */ (function (_super) {
    __extends(FooBar, _super);
    function FooBar(bar) {
        var _this = _super.call(this) || this;
        _this.set(bar);
        return _this;
    }
    return FooBar;
}(Bar));
var bar = new FooBar("foobar");
var foo = new Foo("foo", bar);
console.log({ foo: foo });
console.log(foo.get_bar());
foo.set_bar("foo");
console.log({ foo: foo });
console.log(foo.get_bar());
