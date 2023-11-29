with open("src/input.txt", "r") as f:
    file = f.read()

current_directory = "root"
tree = {}
file_pruned = file.split("\n")[1:]
for line in file_pruned:
    if line.startswith("$ cd"):
        if line[5:] == "..":
            # go back to parent of current dir
            current_directory = "/".join(current_directory.split("/")[:-1])
        else:
            current_directory += "/" + line.split(" ")[-1]
    elif line.split(" ")[0].isnumeric():
        tree[current_directory+ "/" + line.split(" ")[1]] = int(line.split(" ")[0])
    elif line.split(" ")[0] == "dir":
        # hack to also keep a key in the tree for directories only containing directories and not files
        tree[current_directory+ "/" + line.split(" ")[1] + "_dir"] = 0

directories = list(set(["/".join(k.split("/")[:-1]) + "/" for k in tree.keys()]))


sizes = {d:0 for d in directories}
for dir_0 in directories:
    for k,v in tree.items():
        if dir_0 in k:
            sizes[dir_0]+=v

total = 0
for k,value in sizes.items():
    if value <= 100_000:
        total+=value
print("part 1", total)
sorted_values = sorted(sizes.values())
#sorted_sizes = dict(sorted(sizes.items(), key=lambda x:x[1]))
minimum = 30_000_000-(70_000_000-sum(v for k,v in tree.items()))
ans = [x for x in sorted_values if x > minimum][0]
print("part 2", ans)




# root/gwnwqcgq/mdhln/jhmvgjrr/ghv/npqbngg/cfbfmprt/gpnzggqb/pjmc/pjmc is repeated
#     root/qcf/fqld
# 
# # possible issue line 835, pjmc repeated  and not seen in sizes
#-->root/gwnwqcgq/pjmc



