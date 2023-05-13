class BaseClass():
    def __init__(self):
        self.name = 'Base class'

    def get_name(self) -> str:
        return self.name

def print_name(obj: BaseClass):
    print(obj.get_name())

class SomeClass(BaseClass):
    def __init__(self):
        self.name = "I'm a child class"

if __name__ == '__main__':
    some_object = SomeClass()
    print_name(some_object)

    some_base_object = BaseClass()
    print_name(some_base_object)
