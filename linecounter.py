import os

lines = 0

def count(dir):
    global lines
    for i in os.listdir(dir):
        print(f"{dir}/{i}")
        if os.path.isfile(f"{dir}/{i}") and i.endswith(".rs"):
            text = open(f"{dir}/{i}").read()
            lines += len(text.splitlines())
        elif os.path.isdir(f"{dir}/{i}"):
            count(f"{dir}/{i}")

if __name__ == '__main__':
    count("src")
    print(f"Total lines: {lines}")
    input()
