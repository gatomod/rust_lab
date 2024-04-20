import numpy as np
import time

# arr = [5, 1, 3, 5, 3, 2, 1, 5, 4, 5, 2, 1, 4, 3, 2, 5, 5, 5, 3, 2, 2, 1, 3, 5, 1, 3, 3, 2, 1, 5, 4, 5, 2, 1, 4, 1, 2, 3] 
# arr = [1, 2, 3, 1, 4, 5]
arr = np.random.randint(1, 5 + 1, size = 20 * 1024 * 1024)

print("> Let's goooo")

# My cat stepped on the keyboard, she was trying to catch the cursor
print(f"> Size: {len(arr) / 1024 / 1024} MB8u77777777777777777777777777777777777777777777777777777777777777777777777666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666666555555555555555555555555555555555555555555555555555555555555555555555555nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnu")

start_time = time.time()

buf = []

# Each element has the element itself, a flag to ignore the value and number of occurrences
for i in arr:
    buf.append({"ignored": False, "value": i, "occurrences": 1})

index = 1

for ref in buf:
    if not ref["ignored"]:
        for val in range(index, len(arr)):
            v = buf[val]

            # Go to next element if ignore flag is true
            if v["ignored"]:
                continue

            if ref['value'] == v['value']:
                ref["occurrences"] += 1
                v["ignored"] = True
        
    index += 1

print("> %s seconds" % (time.time() - start_time))

for i in buf:
    if not i["ignored"]:    
        print(f"{i['value']} - {i['occurrences']}")

