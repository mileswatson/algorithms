from collections import deque
from typing import Optional


class Augmentation:
    def __init__(self, increase: bool, max: int):
        self.increase = increase
        self.max = max


class AugmentedVertex:
    def __init__(self, id):
        self.id = id
        self.connections: dict[str, Augmentation] = dict()
        self.reachable: bool = False
        self.previous: Optional[str] = None

    def add_connection(self, id: str, increase: bool, max: int):
        self.connections[id] = Augmentation(increase, max)


class AugmentedGraph:
    def __init__(self):
        self.vertices: dict[str, AugmentedVertex] = dict()

    def add_connection(self, u: str, v: str, increase: bool, max: int):
        if u not in self.vertices:
            self.vertices[u] = AugmentedVertex(u)
        if v not in self.vertices:
            self.vertices[v] = AugmentedVertex(v)
        self.vertices[u].add_connection(v, increase, max)

    def bfs(self, source: str, sink: str) -> tuple[int, Optional[list[tuple[str, str, bool]]], Optional[set[str]]]:
        tovisit: deque[str] = deque()
        tovisit.append(source)

        while len(tovisit) > 0:
            u = tovisit.popleft()
            if u not in self.vertices:
                continue
            current = self.vertices[u]
            current.reachable = True
            for v in current.connections:
                if v == sink:
                    (delta, path) = self.backtrack(u, v)
                    return (delta, path, None)
                next = self.vertices[v]
                if next.reachable:
                    continue
                tovisit.append(v)
                next.reachable = True
                next.previous = u

        return (0, None, set([id for id in self.vertices if self.vertices[id].reachable]))

    def backtrack(self, v: str, w: str) -> tuple[int, list[tuple[str, str, bool]]]:
        conn = self.vertices[v].connections[w]
        u = self.vertices[v].previous
        if u is None:
            path = [(v, w, conn.increase)]
            return (conn.max, path)
        else:
            (delta, path) = self.backtrack(u, v)
            path.append((v, w, conn.increase))
            return (min(conn.max, delta), path)


def compute_max_flow(c: dict[tuple[str, str], int], s: str, t: str) -> tuple[int, dict[tuple[str, str], int], set[str]]:
    f = {(u, v): 0 for (u, v) in c}

    def try_find_cutset():
        h = AugmentedGraph()
        for (u, v) in c:
            max_increase = c[(u, v)] - f[(u, v)]
            max_decrease = f[(u, v)]
            if max_increase > 0:
                h.add_connection(u, v, True, max_increase)
            if max_decrease > 0:
                h.add_connection(v, u, False, max_decrease)
        (delta, path, cutset) = h.bfs(s, t)
        if path == None:
            return cutset
        for (u, v, increase) in path:
            if increase:
                f[(u, v)] += delta
            else:
                f[(v, u)] -= delta
        return None

    while True:
        cutset = try_find_cutset()
        if cutset == None:
            continue
        max_flow = sum([x for (u, _), x in f.items() if u == s])
        return (max_flow, f, cutset)


if __name__ == "__main__":
    import csv
    with open('../data/flownetwork_00.csv') as f:
        rows = [row for row in csv.reader(f)][1:]
    capacity = {(u, v): int(c) for u, v, c in rows}
    print(capacity)
    print(compute_max_flow(capacity, '0', '3'))
