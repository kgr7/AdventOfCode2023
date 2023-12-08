from pprint import pprint
text_input = None

with open("../../inputs/day3.txt", "r") as f:
    text_input = f.read()


example = """
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
""".strip("\n")

example = text_input.strip("\n")
example = [x for x in example if x != "\n"]

def match_number(start_idx):
    forward_idx = start_idx
    backward_idx = start_idx
    curr = example[start_idx]

    while backward_idx >= 0 and example[backward_idx] in numbers: 
        backward_idx -= 1
        
    while forward_idx <= len(example) and example[forward_idx] in numbers:
        forward_idx += 1

    return backward_idx + 1, forward_idx

def check_for_number(idx):
    if idx < 0:
        return False
    if idx >= len(example):
        return False
    if example[idx] in numbers:
        return idx

def join_to_num(arr):
    res = ""
    for item in arr:
        res += item
    return int(res)

numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
seen = []
total = 0

for i in range(len(example)):
    if example[i] != "." and example[i] not in numbers:
        number_checks = [check_for_number(i - 141), check_for_number(i - 140), check_for_number(i - 139),
                         check_for_number(i - 1), check_for_number(i + 1),
                         check_for_number(i + 139), check_for_number(i + 140), check_for_number(i + 141)]

        number_checks = [item for item in number_checks if item is not None]

        for idx in number_checks:
            start, end = match_number(idx)
            if (start, end) in seen:
                continue
            seen.append((start, end))
            final_num = join_to_num(example[start:end])
            total += final_num

print(total)
