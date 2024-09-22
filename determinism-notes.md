# yes
    id cumulative_score evolved_via usable_tiles cleaned searched multipliers min_word_length evolved_from tiles words height width
    id width height min_word_length tiles usable_tiles multipliers cumulative_score searched words evolved_via evolved_from cleaned
    min_word_length width multipliers cleaned words tiles evolved_via cumulative_score id usable_tiles height evolved_from searched
    multipliers id cleaned evolved_from width tiles evolved_via height searched words cumulative_score usable_tiles min_word_length
    tiles height multipliers usable_tiles evolved_from min_word_length id searched cumulative_score evolved_via words width cleaned
    tiles width evolved_from searched usable_tiles height cumulative_score min_word_length multipliers id cleaned evolved_via words
    usable_tiles height tiles evolved_from searched cleaned words width id min_word_length multipliers cumulative_score evolved_via
    width id cleaned usable_tiles evolved_from min_word_length multipliers tiles words evolved_via searched height cumulative_score
    tiles cleaned usable_tiles evolved_via height evolved_from multipliers min_word_length searched cumulative_score id words width
    height tiles width min_word_length multipliers cleaned usable_tiles cumulative_score evolved_via words searched id evolved_from
    usable_tiles width multipliers min_word_length cleaned searched id height tiles words cumulative_score evolved_via evolved_from
    height id tiles evolved_from cleaned evolved_via usable_tiles width min_word_length multipliers words searched cumulative_score
    usable_tiles id evolved_via cumulative_score cleaned width height min_word_length searched tiles multipliers evolved_from words


# no
    cleaned cumulative_score evolved_from evolved_via height min_word_length multipliers id usable_tiles searched tiles width words
    cumulative_score id tiles words evolved_from cleaned searched evolved_via width height multipliers min_word_length usable_tiles
    cumulative_score width evolved_from words evolved_via usable_tiles cleaned id min_word_length tiles height searched multipliers
    evolved_via multipliers tiles min_word_length id height width searched evolved_from cumulative_score words usable_tiles cleaned
    height multipliers words cleaned evolved_from usable_tiles width cumulative_score evolved_via searched id tiles min_word_length
    height words width evolved_via cleaned multipliers searched evolved_from tiles id usable_tiles min_word_length cumulative_score
    usable_tiles searched evolved_from evolved_via cumulative_score width words min_word_length id height cleaned multipliers tiles
    words searched cumulative_score multipliers tiles min_word_length evolved_via evolved_from height width usable_tiles id cleaned
    width usable_tiles words searched height evolved_from evolved_via min_word_length id cumulative_score multipliers cleaned tiles
    words cumulative_score id cleaned multipliers tiles width evolved_via searched min_word_length usable_tiles evolved_from height
    words evolved_via min_word_length searched width usable_tiles cleaned cumulative_score height multipliers id evolved_from tiles
    searched cumulative_score evolved_from width tiles words min_word_length height cleaned id multipliers usable_tiles evolved_via
    cumulative_score tiles evolved_via width height min_word_length cleaned searched multipliers words id evolved_from usable_tiles


Hypothesis so far:
* id should appear before cumulative_score.
* cumulative_score should appear before evolved_via.
* evolved_via should appear after evolved_from and cleaned.
* tiles should appear before usable_tiles.
* idth, height, and words should generally appear toward the end of the sequence.

# Bumping up to 4 gens
## Should pass
	Did pass: 1, 3, 4, 5
	Failed: 2

## Should fail
    Passed: 2, 3, 4
    Did fail: 1, 5

Updated hypothesis:
Order rules:
* id must come before cumulative score, but doesn't have to be immediately before it
* evolved_via must come after evolved_from and may need to be near cumulative_score or cleaned.
* Proximity Rule: tiles and usable_tiles might need to appear within a certain number of positions of each other, not just one before the other.

Groupings and Proximity:
* cumulative_score, evolved_via, evolved_from, and cleaned may need to appear in the middle of the list, and potentially within a few positions of each other.
* id, multipliers, and searched might have more flexible positions, but they likely need to maintain an overall flow in relation to the other key groupings.

Passing Orderings:
1. id, evolved_from, tiles, cleaned, usable_tiles, multipliers, searched, words, width, cumulative_score, evolved_via, height, min_word_length
2. tiles, id, searched, multipliers, usable_tiles, min_word_length, cleaned, words, evolved_from, width, evolved_via, height, cumulative_score
3. searched, multipliers, evolved_from, id, evolved_via, cleaned, cumulative_score, min_word_length, tiles, words, usable_tiles, width, height

1 = passed
2 = passed
3 = failed

Failing Orderings:
1. min_word_length, searched, width, id, words, multipliers, height, evolved_from, cumulative_score, usable_tiles, tiles, cleaned, evolved_via
2. words, usable_tiles, searched, min_word_length, evolved_from, multipliers, evolved_via, width, height, cleaned, cumulative_score, id, tiles
3. id, cleaned, multipliers, evolved_via, height, min_word_length, cumulative_score, words, width, evolved_from, usable_tiles, searched, tiles

1 = passed
2 = failed
3 = passed

```
impl PartialOrd for Board {
    #[inline]
    fn partial_cmp(&self, other: &Board) -> ::core::option::Option<::core::cmp::Ordering> {
        match ::core::cmp::PartialOrd::partial_cmp(&other.cumulative_score, &self.cumulative_score) {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&other.usable_tiles, &self.usable_tiles) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&other.multipliers.len(), &self.multipliers.len()) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.words.len(), &other.words.len()) {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&other.evolved_via.as_ref().unwrap().word.len(), &self.evolved_via.as_ref().unwrap().word.len()) {
                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.evolved_from, &other.evolved_from) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.id, &other.id) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.tiles, &other.tiles) {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.width, &other.width) {
                                            ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.height, &other.height) {
                                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.min_word_length, &other.min_word_length) {
                                                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => match ::core::cmp::PartialOrd::partial_cmp(&self.searched, &other.searched) {
                                                                                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => ::core::cmp::PartialOrd::partial_cmp(&self.cleaned, &other.cleaned),
                                                                                                    cmp => cmp,
                                                                                                },
                                                                                            cmp => cmp,
                                                                                        },
                                                                                    cmp => cmp,
                                                                                },
                                                                            cmp => cmp,
                                                                        },
                                                                    cmp => cmp,
                                                                },
                                                            cmp => cmp,
                                                        },
                                                    cmp => cmp,
                                                },
                                            cmp => cmp,
                                        },
                                    cmp => cmp,
                                },
                            cmp => cmp,
                        },
                    cmp => cmp,
                },
            cmp => cmp,
        }
    }
}
```

```
impl Ord for Board {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match ::core::cmp::Ord::cmp(&other.cumulative_score, &self.cumulative_score) {
            ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&other.usable_tiles, &self.usable_tiles) {
                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&other.multipliers.len(), &self.multipliers.len()) {
                    ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.words.len(), &other.words.len()) {
                        ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&other.evolved_via.as_ref().unwrap().word.len(), &self.evolved_via.as_ref().unwrap().word.len()) {
                            ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.evolved_from, &other.evolved_from) {
                                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.id, &other.id) {
                                    ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.tiles, &other.tiles) {
                                        ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.width, &other.width) {
                                            ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.height, &other.height) {
                                                ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.min_word_length, &other.min_word_length) {
                                                                    ::core::cmp::Ordering::Equal => match ::core::cmp::Ord::cmp(&self.searched, &other.searched) {
                                                                                                    ::core::cmp::Ordering::Equal => ::core::cmp::Ord::cmp(&self.cleaned, &other.cleaned),
                                                                                                    cmp => cmp,
                                                                                                },
                                                                                            cmp => cmp,
                                                                                        },
                                                                                    cmp => cmp,
                                                                                },
                                                                            cmp => cmp,
                                                                        },
                                                                    cmp => cmp,
                                                                },
                                                            cmp => cmp,
                                                        },
                                                    cmp => cmp,
                                                },
                                            cmp => cmp,
                                        },
                                    cmp => cmp,
                                },
                            cmp => cmp,
                        },
                    cmp => cmp,
                },
            cmp => cmp,
        }
    }
}
```