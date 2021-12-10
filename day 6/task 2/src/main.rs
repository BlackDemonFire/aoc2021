use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let fishstrings = contents.split(",");
    let mut fish = vec![u128::MIN; 9];
    fishstrings.for_each(|s| fish[s.parse::<usize>().unwrap()] += 1);
    for index in 1..=256 {
        let mut newfish = fish.iter();
        let mut fishlist = vec![u128::MIN; 9];
        let &zeroes = newfish.next().unwrap();
        let &ones = newfish.next().unwrap();
        let &twos = newfish.next().unwrap();
        let &threes = newfish.next().unwrap();
        let &fours = newfish.next().unwrap();
        let &fives = newfish.next().unwrap();
        let &sixes = newfish.next().unwrap();
        let &sevens = newfish.next().unwrap();
        let &eights = newfish.next().unwrap();
        fishlist[8] = zeroes;
        fishlist[6] = zeroes;
        fishlist[0] = ones;
        fishlist[1] = twos;
        fishlist[2] = threes;
        fishlist[3] = fours;
        fishlist[4] = fives;
        fishlist[5] = sixes;
        fishlist[6] += sevens;
        fishlist[7] = eights;
        println!(
            "There are {} fish after Iteration {}.",
            fishlist.iter().sum::<u128>(),
            index
        );
        fish = fishlist;
    }
}
