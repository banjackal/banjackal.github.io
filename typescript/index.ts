class Foo {
    foo: string
    bar: FooBar

    constructor(foo: string, bar: FooBar){
        this.foo = foo
        this.bar = bar
    }

    get_bar(): string{
        try {
            return this.bar.get_error()
        }
        catch(error){
            return this.do_something_else()
        }
    }

    set_bar(bar: string){
        return this.bar.set(bar)
    }

    private do_something(): string {
        return "something"
    }

    private do_something_else(): string {
        try {
            throw Error("error")
        }
        catch {
            return this.do_something()
        }
    }
}

abstract class Bar {
    private bar: string

    set(bar: string) {
        this.bar = bar
    }

    get_error(): string {
        throw new Error("nope")
    }
    get(): string {
        return this.bar
    }

}

class FooBar extends Bar {
    constructor(bar: string){
        super()
        this.set(bar)
    }
}


const bar = new FooBar("foobar")

const foo = new Foo("foo", bar)

console.log({foo})
console.log(foo.get_bar())
foo.set_bar("foo")
console.log({foo})
console.log(foo.get_bar())
