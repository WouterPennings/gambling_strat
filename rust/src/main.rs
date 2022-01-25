use rand::Rng;

fn main() {
    let bet: f32 = 2.0;
    let mut loss: f64 = 0.0;
    let mut money: f64 = bet as f64;

    let mut i = 0;
    loop {
        if random_bool(40) {
            money = (money * 2.0) - loss; 
            break;
        } else {
            loss += money;
            money = loss / 2.0 + bet as f64;
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
