class SomeBaseClass():
    def __init__(self):
        self.name = 'Base class'

    def get_name(self) -> str:
        return self.name

class SomeChildClass(SomeBaseClass):
    def __init__(self):
        self.name = 'Some child class'

class AnotherChildClass(SomeBaseClass):
    def __init__(self):
        self.name = 'Another child class'

class DoesntImplementBaseClass():
    def get_name(self):
        return 'I do not implement the base class'

class DoesntHaveMethod():
    def some_method(self):
        return 'garbage'

def print_name(obj: SomeBaseClass):
    print(obj.get_name())

def any_print_name(obj):
    print(obj.get_name())

if __name__ == '__main__':
    print('Using the same function with different implementations of the same base class...')
    base = SomeBaseClass()
    print_name(base)
    
    first_child = SomeChildClass()
    print_name(first_child)

    second_child = AnotherChildClass()
    print_name(second_child)

    not_child_class = DoesntImplementBaseClass()
    # print_name(not_child_class)
    any_print_name(not_child_class)

    runtime_error = DoesntHaveMethod()
    any_print_name(runtime_error)



