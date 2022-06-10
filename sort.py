import random


def insertionSort(arr: list):
    for i in range(1, len(arr)):
        current = arr[i]
        j = i - 1

        while j >= 0 and arr[j] > current:
            arr[j + 1] = arr[j]
            j -= 1

        arr[j + 1] = current


res = [random.randrange(1, 1000, 1) for _ in range(10)]
print(res)
insertionSort(res)
print(res)
