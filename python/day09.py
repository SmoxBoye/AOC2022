with open("input\input09toy.txt") as f:
    data = f.readlines()

class Move:
    def __init__(self, direction, steps):
        self.direction = direction
        self.steps = steps
    
    def __repr__(self):
        return self.direction + ", " + str(self.steps)

data = [line.strip() for line in data]
data = [line.split(" ") for line in data]
data =  [[Move(line[0], int(line[1]))] for line in data]

print(data)


