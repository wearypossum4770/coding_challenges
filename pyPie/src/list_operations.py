import re


def list_operations(args):
    regex = r"(\w+)\s+(.*)|(\w+)"
    print("in the function")
    commands = []
    regex_groupings = [re.match(regex, operation).groups() for operation in args]

    for item in regex_groupings:

        command = [operation for operation in item if operation is not None]

    return commands


if __name__ == "__main__":
    init = []
    N = int(input())
    for _ in range(N):
        command = input()
        init.append(command)
    print(list_operations(init))
