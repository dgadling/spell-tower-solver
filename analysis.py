from collections import defaultdict, Counter

# Re-define functions since code execution state was reset

# Function to analyze pairwise position relationships
def analyze_pairwise_relationships(sets):
    pairwise_position_dict = defaultdict(list)

    for s in sets:
        words = s.split()
        for i, word1 in enumerate(words):
            for j, word2 in enumerate(words):
                if i < j:  # Avoid duplicates
                    pairwise_position_dict[(word1, word2)].append((i, j))

    return {pair: Counter(positions) for pair, positions in pairwise_position_dict.items()}

# Original passed and failed sets
passed_set = [
    "id width height min_word_length tiles usable_tiles multipliers cumulative_score searched words evolved_via evolved_from cleaned",
    "tiles width evolved_from searched usable_tiles height cumulative_score min_word_length multipliers id cleaned evolved_via words",
    "min_word_length width multipliers cleaned words tiles evolved_via cumulative_score id usable_tiles height evolved_from searched",
    "width id cleaned usable_tiles evolved_from min_word_length multipliers tiles words evolved_via searched height cumulative_score",
    "multipliers id cleaned evolved_from width tiles evolved_via height searched words cumulative_score usable_tiles min_word_length",
    "id cumulative_score evolved_via usable_tiles cleaned searched multipliers min_word_length evolved_from tiles words height width",
    "tiles height multipliers usable_tiles evolved_from min_word_length id searched cumulative_score evolved_via words width cleaned",
    "usable_tiles height tiles evolved_from searched cleaned words width id min_word_length multipliers cumulative_score evolved_via",
]

failed_set = [
    "words searched cumulative_score multipliers tiles min_word_length evolved_via evolved_from height width usable_tiles id cleaned",
    "usable_tiles searched evolved_from evolved_via cumulative_score width words min_word_length id height cleaned multipliers tiles",
    "cumulative_score id tiles words evolved_from cleaned searched evolved_via width height multipliers min_word_length usable_tiles",
    "cumulative_score width evolved_from words evolved_via usable_tiles cleaned id min_word_length tiles height searched multipliers",
    "height multipliers words cleaned evolved_from usable_tiles width cumulative_score evolved_via searched id tiles min_word_length",
    "cleaned cumulative_score evolved_from evolved_via height min_word_length multipliers id usable_tiles searched tiles width words",
    "height words width evolved_via cleaned multipliers searched evolved_from tiles id usable_tiles min_word_length cumulative_score",
    "evolved_via multipliers tiles min_word_length id height width searched evolved_from cumulative_score words usable_tiles cleaned",
]

# New examples provided by the user
new_passed_set = [
    "tiles cleaned usable_tiles evolved_via height evolved_from multipliers min_word_length searched cumulative_score id words width",
    "height tiles width min_word_length multipliers cleaned usable_tiles cumulative_score evolved_via words searched id evolved_from",
    "usable_tiles width multipliers min_word_length cleaned searched id height tiles words cumulative_score evolved_via evolved_from",
    "height id tiles evolved_from cleaned evolved_via usable_tiles width min_word_length multipliers words searched cumulative_score",
    "usable_tiles id evolved_via cumulative_score cleaned width height min_word_length searched tiles multipliers evolved_from words"
]

new_failed_set = [
    "width usable_tiles words searched height evolved_from evolved_via min_word_length id cumulative_score multipliers cleaned tiles",
    "words cumulative_score id cleaned multipliers tiles width evolved_via searched min_word_length usable_tiles evolved_from height",
    "words evolved_via min_word_length searched width usable_tiles cleaned cumulative_score height multipliers id evolved_from tiles",
    "searched cumulative_score evolved_from width tiles words min_word_length height cleaned id multipliers usable_tiles evolved_via",
    "cumulative_score tiles evolved_via width height min_word_length cleaned searched multipliers words id evolved_from usable_tiles"
]

# Extending the original sets with the new examples
extended_passed_set = passed_set + new_passed_set
extended_failed_set = failed_set + new_failed_set

# Re-running the pairwise position analysis with the new extended sets
extended_passed_pairwise_analysis = analyze_pairwise_relationships(extended_passed_set)
extended_failed_pairwise_analysis = analyze_pairwise_relationships(extended_failed_set)

for pair in extended_passed_pairwise_analysis:
    if pair not in extended_failed_pairwise_analysis:
        print("This pair only showed up in passing: {pair}")
#print(extended_passed_pairwise_analysis)
#print(extended_failed_pairwise_analysis)
