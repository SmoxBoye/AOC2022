class directory:
    def __init__(self, name, parent=None):
        self.name = name
        self.subdirs = []
        self.files = []
        self.parent = parent
    
    def get_size(self):
        size = 0
        for file in self.files:
            size += file.size
        for subdir in self.subdirs:
            size += subdir.get_size()
        return size
    
    def add_file(self, file):
        self.files.append(file)
    
    def add_subdir(self, name):
        self.subdirs.append(directory(name=name, parent=self))
    
    def step_up(self):
        if self.parent == None:
            return self
        return self.parent
    
    def step_in(self, name):
        for subdir in self.subdirs:
            if subdir.name == name:
                return subdir
        self.add_subdir(name)
        return self.step_in(name)
    
    def go_home(self):
        if self.parent == None:
            return self
        else:
            return self.step_up().go_home()
    
    def solution_1(self):
        size = 0
        for subdir in self.subdirs:
            if subdir.get_size() <= 100000:
                size += subdir.get_size()
            size += subdir.solution_1()
        return size
    
    def solution_2(self, required_size):
        size = 70_000_000
        for subdir in self.subdirs:
            if subdir.get_size() >= required_size:
                size = min(subdir.get_size(), size) 
            size = min(subdir.solution_2(required_size), size)
        return size

class file: 
    def __init__(self, name, size):
        self.name = name
        self.size = size


filesystem = directory("/")

with open("input\input07.txt") as f:
    data = f.readlines()

for instruction in data:
    p_instruction = instruction.strip().split(" ")
    if p_instruction[0] == "$":
        if p_instruction[1] == "cd":
            if p_instruction[2] == "..":
                filesystem = filesystem.step_up() # cd .. moves out one level
            elif p_instruction[2] == "/": # cd / switches the current directory to the outermost directory
                filesystem = filesystem.go_home()
            else:
                filesystem = filesystem.step_in(p_instruction[2]) # cd x moves in one level
        elif p_instruction[1] == "ls":
            continue
    elif p_instruction[0] == "dir":
        filesystem.add_subdir(p_instruction[1])
    else: 
        filesystem.add_file(file(p_instruction[1], int(p_instruction[0])))

filesystem = filesystem.go_home()

print(f"Total filesystem size: {filesystem.get_size()}")

print(f"Solution 1: {filesystem.solution_1()}")
filesystem = filesystem.go_home()
required_size = (filesystem.get_size() - (70_000_000 - 30_000_000))
print(f"Solution 2: {filesystem.solution_2(required_size)}")