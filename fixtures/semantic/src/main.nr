struct Pair {
    left: Field,
    right: Field,
}

fn add_pair(pair: Pair) -> Field {
    pair.left + pair.right
}

fn main() {
    let pair = Pair { left: 1, right: 2 };
    let total = add_pair(pair);
    let _copy = total;
}
