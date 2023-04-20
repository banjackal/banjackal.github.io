def ok_error():
    print('testing')
    try:
        x = 1
        y = "string"
        break
        print("trying to add a string and int")
        z = x + y
        print(z)
    except Exception as x:
        print(x)
        print('this is fine')
    finally:
        print('finally')

    print('still running')


def exit_error():
    print("this will exit")
    x = 1
    y = "string"
    z = x + y

    print("unreachable")

if __name__ == '__main__':
    ok_error()
    exit_error()

