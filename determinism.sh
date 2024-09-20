#!/bin/zsh

cargo build --release
rm -rf run1 run2
final_gen=3
for run_n in {1..2} ; {
    target/release/spell-tower-solver sample-input/board-1.ron -q -c 15 --max-generations ${final_gen} --output-dir run${run_n}
};

diff run1/gen-${final_gen}-to-process-final-sort.txt run2/gen-${final_gen}-to-process-final-sort.txt &> /dev/null
if [[ $? -eq 1 ]] ; then
    echo "😭😭😭😭"
else
    echo "✨they✨are✨the✨same✨"
fi
