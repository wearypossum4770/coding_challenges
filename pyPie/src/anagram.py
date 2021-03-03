from collections import Counter


def stringAnagram(dictionary, query):
    # Write your code here
    counterDict = [Counter(word) for word in dictionary]
    found = []

    for word in query:
        count = Counter(word)
        yes = 0
        for j in counterDict:
            if count == j:
                yes += 1
        found.append(yes)
    return found


print(
    stringAnagram(
        [
            "hack",
            "a",
            "rank",
            "khac",
            "ackh",
            "kran",
            "rankhacker",
            "a",
            "ab",
            "ba",
            "stairs",
            "raits",
        ],
        ["a", "nark", "bs", "hack", "stair"],
    )
)
