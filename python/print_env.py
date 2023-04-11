import os
def getEnv():
    foo = os.environ['foo']
    return foo

if __name__ == '__main__':
    foo = getEnv()
    print(foo)
