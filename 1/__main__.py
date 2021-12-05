import sys

count = 0
last = None
for val in sys.stdin:
    val = val.strip()
    if val != "":
        if last is not None and int(val) > last:
            count += 1
        last = int(val)

print(count)
