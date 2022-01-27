def fix_root(values, parent):
    left = 2 * parent + 1
    right = 2 * parent + 2

    if left >= len(values):
        return

    max_child = right if right < len(
        values) and values[right] > values[left] else left

    if values[parent] < values[max_child]:
        values[parent], values[max_child] = values[max_child], values[parent]
        fix_root(values, max_child)


def fix_parent(values, child):
    if child == 0:
        return
    parent = (child-1) // 2
    if values[parent] < values[child]:
        values[parent], values[child] = values[child], values[parent]
        fix_parent(values, parent)


def heapify(values, n=0):
    start = (1 << n) - 1
    end = (1 << (n+1)) - 1
    if end < len(values):
        heapify(values, n+1)
        for i in range(start, end):
            fix_root(values, i)


def push(values, element):
    values.append(element)
    fix_parent(values, len(values) - 1)


def popmax(values):
    if len(values) == 0:
        raise IndexError()
    elif len(values) == 1:
        return values.pop()

    max = values[0]
    values[0] = values.pop()
    fix_root(values, 0)
    return max
