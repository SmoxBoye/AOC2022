with open("input\input02.txt") as f:
    data = f.readlines()


translate = {"A": 1, "B":2, "C":3, "X":1, "Y":2, "Z":3}

rock_t = {1:3, 2:1, 3:2}
paper_t = {1:1, 2:2, 3:3}
scissor_t = {1:2, 2:3, 3:1}


pre_processed = []

for line in data:
    alts = line.strip().split(" ")
    pre_processed.append([translate[alts[0]], translate[alts[1]]])

#print(pre_processed)

# for index, line in enumerate(pre_processed):
#     if line[0] == 1:
#         pre_processed[index][1] = rock_t[line[1]]
#     if line[0] == 2:
#         pre_processed[index][1] = paper_t[line[1]]
#     if line[0] == 3:
#         pre_processed[index][1] = scissor_t[line[1]]


def rock(input):
    if input == 1:
        return 3
    if input == 2:
        return 0
    if input == 3:
        return 6

def paper(input):
    if input == 1:
        return 6
    if input == 2:
        return 3
    if input == 3:
        return 0

def scissors(input):
    if input == 1:
        return 0
    if input == 2:
        return 6
    if input == 3:
        return 3

scores = []


for game in pre_processed:
    if game[1] == 1:
        scores.append(1 + rock(game[0]))
    if game[1] == 2:
        scores.append(2 + paper(game[0]))
    if game[1] == 3:
        scores.append(3 + scissors(game[0]))


#print(scores)
print(sum(scores))