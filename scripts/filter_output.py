import argparse
import pandas

def main():
    args = get_filter_parser()
    
    df = pandas.read_csv(args.input, sep='\t')
    output = args.output
    ratio = args.ratio

    if not (0 <= ratio <= 1):
        print("Ratio must be between 0 and 1")
        return
    
    if args.repeats:
        filter_repeats(df, output, ratio)
    elif args.overlap:
        filter_overlap(df, output, ratio)

    print("Done!")


def get_filter_parser():
    parser = argparse.ArgumentParser(
        description="Filter the output"
    )
    parser.add_argument('-i', '--input', required=True, help="File input path")
    parser.add_argument('-o', '--output', required=True, help="File output path")
    parser.add_argument('-r', '--ratio', required=True, type=float, help="Max tolerance ratio [0-1]")

    filter_options = parser.add_mutually_exclusive_group(required=True)

    filter_options.add_argument('--repeats', help="Filters out palindromes with more than x proportion lowercase letters in arms", action='store_true')
    filter_options.add_argument('--overlap', help="Filters out palindromes that have more than x proportion overlap with another", action='store_true')
    
    return parser.parse_args()



#Filters out sequences with too many lowercase letters
def filter_repeats(df, output, ratio):
    arms = df["Arm-Length(Approx)"]
    seq = df["Sequence"]
    indices = []
    for i in range(len(arms)):
        arm = arms[i]
        if has_more_uppercase(seq[i][:arm], ratio) and has_more_uppercase(seq[i][-arm:], ratio):
            indices.append(i)
    
    filtered_df = df.iloc[indices]
    filtered_df.to_csv(output, sep='\t', index=False)

def has_more_uppercase(sequence, ratio):
    if not isinstance(sequence, str):
        print(sequence)
        raise Exception("Invalid sequence")
    capital_count = sum(1 for char in sequence if char.isupper())
    if len(sequence) == 0:
        return True
    return capital_count / len(sequence) >= ratio


#Filters out overlapping palindromes
def filter_overlap(df, output, ratio):
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
        
        #Skip if too much overlap
        if overlap > ratio:
            continue
        else:
            filtered_indices.append(i)
            prev_start = current_start
            prev_end = current_end

    filtered_df = df.iloc[filtered_indices]
    filtered_df.to_csv(output, sep='\t', index=False)

def calculate_overlap(start1, end1, start2, end2):
    overlap_start = max(start1, start2)
    overlap_end = min(end1, end2)
    overlap = max(0, overlap_end - overlap_start)
    
    len1 = end1 - start1
    len2 = end2 - start2
    overlap_percent = overlap / min(len1, len2)
    
    return overlap_percent


if __name__ == "__main__":
    main()