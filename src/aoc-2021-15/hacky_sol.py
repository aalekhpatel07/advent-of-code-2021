import sys
from queue import PriorityQueue


def heuristic(a, b):
    (x1, y1) = a
    (x2, y2) = b
    return abs(x1 - x2) + abs(y1 - y2)


# https://github.com/ranjian0/A-star-Pathfinding/blob/master/core/astar.py
# Slightly modified this A* search for my need.
# Will definitely reimplement A* in Rust for even more speed.
def a_star_search(grid, start, goal):
    frontier = PriorityQueue()

    frontier.put((grid[start[0]][start[1]], tuple(start)))

    came_from = {}
    cost_so_far = {}

    came_from[start] = None
    cost_so_far[start] = 0

    while not frontier.empty():
        wt, (cx, cy) = frontier.get()

        if (cx, cy) == goal:
            break

        nbs = {
            (sx + cx, sy + cy) for sx, sy in {(1, 0), (0, 1), (-1, 0), (0, -1)}
        }
        nbs = {
            (nx, ny) for nx, ny in nbs if 0 <= nx < len(grid) and 0 <= ny < len(grid[0])
        }

        for nx, ny in nbs:
            new_cost = cost_so_far[cx, cy] + grid[nx][ny]
            if (nx, ny) not in cost_so_far or new_cost < cost_so_far[nx, ny]:
                cost_so_far[nx, ny] = new_cost
                priority = new_cost + heuristic(goal, (nx, ny))
                frontier.put((priority, (nx, ny)))
                came_from[nx, ny] = (cx, cy)

    return came_from, cost_so_far


def main():
    grid = []
    for row in sys.stdin:
        grid_row = []
        for entry in row.strip():
            grid_row.append(int(entry))
        grid.append(grid_row)

    start = (0, 0)
    end = (len(grid) - 1, len(grid[0]) - 1)
    cf, costs = a_star_search(grid, start, end)

    print("Part 1:", costs[end[0], end[1]])

    big = []
    for row in grid:
        big_row = []
        for i in range(5):
            for x in row:
                if not (x + i) % 9:
                    big_row.append(9)
                else:
                    big_row.append((x + i) % 9)
        big.append(big_row)

    big_empty = []
    for row in big:
        big_empty.append(row)

    for j in range(1, 5):
        for row in big:
            temp_row = []
            for x in row:
                if not (x + j) % 9:
                    temp_row.append(9)
                else:
                    temp_row.append((x + j) % 9)
            big_empty.append(temp_row)

    start = (0, 0)
    end = (len(big_empty) - 1, len(big_empty[0]) - 1)
    cf, costs = a_star_search(big_empty, start, end)

    print("Part 2:", costs[end[0], end[1]])

    # for row in big:
    #     print("".join(map(str, row)))
    # print(big)

if __name__ == '__main__':
    main()


