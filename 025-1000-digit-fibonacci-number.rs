fn main() {
    let mut current = vec![1];
    let mut next = vec![1];
    let mut count = 1;
    loop {
        if current.len() == 1000 {
            current.reverse();
            let number: String = current.iter().map(|digit| format!("{}", digit)).collect();
            println!("{} → {}", count, number);
            return;
        }
        count += 1;
        let sum = add(&current, &next);
        current = next;
        next = sum;
    }
}

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let len = a.len().max(b.len());
    let mut c = vec![];
    c.reserve(len + 1);
    let mut carry = 0;
    for i in 0..len {
        let ai = *a.get(i).unwrap_or(&0);
        let bi = *b.get(i).unwrap_or(&0);
        let sum = ai + bi + carry;
        let ci = sum % 10;
        carry = sum / 10;
        c.push(ci);
    }
    if carry > 0 {
        c.push(carry);
    }
    c
}
