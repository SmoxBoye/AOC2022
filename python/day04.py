with open("input\input04.txt") as f:
    data = f.readlines()
    


# I literally have not slept so im dead



count1 = 0
for line in data:
    elfs = line.split(",")
    
    left = [int(x) for x in elfs[0].split("-")]
    right = [int(x) for x in elfs[1].split("-")]

    if left[0] >= right[0] and left[1] <= right[1]:
        count1 += 1
        #print("left")
        #print(left, right)
    elif left[0] <= right[0] and left[1] >= right[1]:
        count1 += 1
        #print("right")
        #print(left, right)

print(f"PART 1: {count1}\n")

count2 = 0
for index, line in enumerate(data):
    elfs = line.split(",")
    
    left = [int(x) for x in elfs[0].split("-")]
    right = [int(x) for x in elfs[1].split("-")]
    s1 = set(range(left[0], left[1]+1))
    s2 = set(range(right[0], right[1]+1))
    if s1.intersection(s2):
        count2 += 1

print(f"PART 2: {count2}\n")
