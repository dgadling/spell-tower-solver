letter_vals = {
    "a": 1,
    "b": 4,
    "c": 4,
    "d": 3,
    "e": 1,
    "f": 5,
    "g": 3,
    "h": 5,
    "i": 1,
    "j": 9,
    "k": 6,
    "l": 2,
    "m": 4,
    "n": 2,
    "o": 1,
    "p": 4,
    "q": 12,
    "r": 2,
    "s": 1,
    "t": 2,
    "u": 1,
    # v = 5 on 3 letter words, 4 otherwise?
    "v": 5,  # solver said 5/2, sometimes 4, sometimes 5
    "w": 4,
    "x": 10,
    "y": 5,
    "z": 11,
}

while True:
    word = input("Word plz: ")
    base_val = sum(letter_vals[l] for l in list(word))
    len_mult = len(word)

    extra = input("Extras plz: ")
    extra_val = 0
    if extra:
        extra_val = sum(letter_vals[l] for l in list(extra))

    extra_len = len(extra)
    # NOTE: This seems to work up to 6 letters long
    score = (base_val * len_mult) + (extra_val * len_mult)

    print(f"  Base = {len_mult} long ; extras = {extra_len} long")
    print(f"  Score = ({base_val} * {len_mult}) + ({extra_val} * {len_mult}) = {score}")
