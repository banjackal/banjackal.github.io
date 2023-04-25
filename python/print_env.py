import os
from typing import Optional

def getEnv():
    foo = os.environ.get('foo','foobar')
    return foo


def dontReturn() -> Optional[str]:
    print("not returning anything")

if __name__ == '__main__':
    foo = getEnv()
    print(foo)
    if foo is not None:
        foo = dontReturn()
    print(foo)

    
