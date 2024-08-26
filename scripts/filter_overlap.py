import os
import pandas
import sys

script, path_name = sys.argv
df = pandas.read_csv(os.path.join("..", "outputs",path_name), sep='\t')

def calculate_overlap(start1, end1, start2, end2):
    overlap_start = max(start1, start2)
    overlap_end = min(end1, end2)
    overlap = max(0, overlap_end - overlap_start)
    
    len1 = end1 - start1
    len2 = end2 - start2
    overlap_percent = overlap / min(len1, len2)
    
    return overlap_percent

start = df["Start"]
end = df["End"]

filtered_indices = []
prev_start = start[0]
prev_end = end[0]
filtered_indices.append(0)

for i in range(1, len(start)):
    current_start = start[i]
    current_end = end[i]
    
    overlap = calculate_overlap(prev_start, prev_end, current_start, current_end)
    
    if overlap > 0.9:
        continue
    else:
        filtered_indices.append(i)
        prev_start = current_start
        prev_end = current_end

filtered_df = df.iloc[filtered_indices]
filtered_df.to_csv(os.path.join("..", "outputs", "filtered_"+path_name), sep='\t', index=False)