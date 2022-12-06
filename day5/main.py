from dataclasses import dataclass
from typing import List

@dataclass
class Move:
    src: int
    dst: int
    number: int

def solve_puzzle(part_two: bool):
    with open("input", "r") as input:
        input_string = input.read()
     
        crane, moves = input_string.split("\n\n")
        splitted_moves = moves.splitlines()

        stacks = read_crane(crane)

        for splitted_move in splitted_moves:
            move = read_move(splitted_move)
            if part_two:
                stacks = do_move_order(move, stacks)
            else:
                stacks = do_move_no_order(move, stacks)

        top_containers = get_top_containers(stacks)

        return top_containers


def read_crane(crane: str):
    stacks = []
    container_length = 4
    crane_lines = crane.splitlines()
    number_of_stack = (len(crane_lines[0]) + 1) // container_length

    for _ in range(number_of_stack):
        stacks.append([])

    position = 0
    for line in crane_lines:
        position = 0
        while position < len(line):
            container: str = line[position:position + container_length]
            if container.startswith("["):
                stacks[position // container_length].append(line[position:position + container_length - 1])

            position += container_length
    
    return stacks

def read_move(move: str) -> Move:
    words = move.split()
    number = int(words[1])
    src =  int(words[3]) - 1
    dst = int(words[5]) - 1

    return Move(src, dst, number)

def do_move_no_order(move: Move, stacks: List[List[str]]) -> List[List[str]]:
    for _ in range(move.number):
        container = stacks[move.src][0]
        stacks[move.dst].insert(0, container) 
        del stacks[move.src][0]

    return stacks


def do_move_order(move: Move, stacks: List[List[str]]) -> List[List[str]]:
    containers = []

    for _ in range(move.number):
        container = stacks[move.src][0]
        containers.insert(0, container)
        del stacks[move.src][0]

    for container in containers:
        stacks[move.dst].insert(0, container)

    return stacks

def get_top_containers(stacks: List[List[str]]) -> List[str]:
    top_containers = []
    for stack in stacks:
        top_containers.append(stack[0])

    return top_containers

if __name__ == "__main__":
    stacks = solve_puzzle(True)
    print(stacks)
