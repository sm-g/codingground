enum Answer {
    Higher,
    Lower,
    Bingo,
}
struct GuessState {
    guess: u32,
    answer: Answer,
    low: u32,
    high: u32,
}

fn suggest_guess(prior_guess: u32, answer: Answer) {
    match answer {
        Answer::Higher => println!("maybe try {} next", prior_guess + 10),
        Answer::Lower  => println!("maybe try {} next", prior_guess - 1),
        Answer::Bingo  => println!("we won with {}!", prior_guess),
    }
}

fn demo_suggest_guess() {
    suggest_guess(10, Answer::Higher);
    suggest_guess(20, Answer::Lower);
    suggest_guess(19, Answer::Bingo);
}
fn suggest_guess_smarter(s: GuessState) {
    match s {
        GuessState { answer: Answer::Bingo, guess: p, .. } => {

            println!("we won with {}!", p);
        }
        GuessState { answer: Answer::Higher, low: _, guess: l, high: h } |
        GuessState { answer: Answer::Lower,  low: l, guess: h, high: _ } => {
   
            let mid = l + ((h - l) / 2);
            println!("lets try {} next", mid);
        }
    }
}

fn demo_guess_state() {
    suggest_guess_smarter(GuessState {
        guess: 20, answer: Answer::Lower, low: 0, high: 1000
    });
}
fn main() {
demo_guess_state();
}