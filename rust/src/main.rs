use rand::Rng;

fn main() {
    let final_prize = 1;
    let mut loss: i128 = 0;
    let mut money: i128 = 1;

    let mut i = 0;
    loop {
        if random_bool(5) {
            money -= loss;
            break;
        } else {
            loss = money + money;
            money = loss + final_prize;
            i += 1;
        }
    }
    println!(
        "You have made: {} Dollar, in {} lifecycles\nThat means that, you have lost {} Dollar before you won, which has {} amount of zeros!..",
        money, i, loss, loss.to_string().chars().count() - 1
    );
}

fn random_bool(percentage: i32) -> bool {
    return rand::thread_rng().gen_range(1..101) < percentage;
}
