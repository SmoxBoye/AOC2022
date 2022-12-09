with open("input\input08.txt") as f:
    data = f.readlines()

data = [line.strip() for line in data]
data = [list(line) for line in data]
data =  [[int(x) for x in line] for line in data]

#print(data)
#print(len(data))

def solution1(data):
    count = len(data) * 4 - 4
    for y in range(1, len(data) - 1):
        for x in range(1, len(data) - 1):
            point = data[y][x]
            hidden = 0
            for y2 in range(0, y):
                if data[y2][x] >= point:
                    hidden += 1
                    break
            
            
            for y2 in range(y + 1, len(data)):
                if data[y2][x] >= point:
                    hidden += 1
                    break
            
            
            for x2 in range(0, x):
                if data[y][x2] >= point:
                    hidden += 1
                    break
            
            
            for x2 in range(x+1, len(data)):
                if data[y][x2] >= point:
                    hidden += 1
                    break
            
            if hidden != 4:
                #print(data[y][x], x, y)
                count += 1
    return count

def solution2(data):
    best = 0
    for y in range(1, len(data) - 1):
        for x in range(1, len(data) - 1):
            points = [0, 0, 0, 0]
            point = data[y][x]
            hidden = 0
            for y2 in range(y-1, -1, -1):
                if data[y2][x] < point:
                    points[0] += 1
                else:
                    points[0] += 1
                    break
            
            
            for y2 in range(y + 1, len(data)):
                if data[y2][x] < point:
                    points[1] += 1
                else:
                    points[1] += 1
                    break
            
            
            for x2 in range(x-1, -1, -1):
                if data[y][x2] < point:
                    points[2] += 1
                else:
                    points[2] += 1
                    break
            
            
            for x2 in range(x+1, len(data)):
                if data[y][x2] < point:
                    points[3] += 1
                else:
                    points[3] += 1
                    break
            #print(points[0], points[1], points[2], points[3])
            sum_points = points[0]*points[1]*points[2]*points[3]
            #print(sum_points)
            best = max(best, sum_points)
    return best



print(f"Sol1: {solution1(data)}")
print(f"Sol2: {solution2(data)}")