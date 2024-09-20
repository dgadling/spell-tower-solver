#!/bin/zsh

cargo build --release
rm -f run1/*.txt run2/*.txt
for run_n in {1..2} ; {
    target/release/spell-tower-solver sample-input/board-1.ron -q -c 15 --max-generations 3 --output-dir run${run_n}
};

diff run1/gen-3-to-process-final-sort.txt run2/gen-3-to-process-final-sort.txt &> /dev/null
if [[ $? -eq 1 ]] ; then
    echo "😭😭😭😭"
else
    echo "✨they✨are✨the✨same✨"
fi
