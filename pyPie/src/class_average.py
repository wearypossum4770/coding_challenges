from functools import reduce


def student_average(kwargs):
    return {
        student: format(reduce(lambda x, y: x + y, scores) / len(scores), ".2f")
        for student, scores in kwargs.items()
    }


if __name__ == "__main__":
    n = int(input())
    student_marks = {}
    for _ in range(n):
        name, *line = input().split()
        scores = list(map(float, line))
        student_marks[name] = scores
    query_name = input()
    print(student_average(student_marks).get(query_name))
