python_students = [
    ["Harry", 37.21],
    ["Berry", 37.21],
    ["Tina", 37.2],
    ["Akriti", 41],
    ["Harsh", 39],
]


def summa_cum_laude(table):
    obj = list(
        {key: value for key, value in sorted(table, key=lambda item: item[1])}.items()
    )
    lowest = obj[0][1]
    lowest_removed = [
        student
        for student in sorted(obj, key=lambda score: score[1])
        if student[1] > lowest
    ]
    for student in sorted(lowest_removed, key=lambda name: name[0]):
        if lowest_removed[0][1] == student[1]:
            print(student[0])


if __name__ == "__main__":
    init = []
    for _ in range(int(input())):
        name = input()
        score = float(input())
        init.append([name, score])
    print(summa_cum_laude(init))
