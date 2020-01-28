use korean::*;

fn main() {
    let input = "당신은 네모네모 멍뭉이의 저주에 걸렸습니다!";

    let nemo = input
        .disassemble()
        .map(|c| if c == 'ㅇ' { 'ㅁ' } else { c })
        .assemble();

    println!("{}", nemo); // 담신믄 네모네모 멈뭄미의 저주메 걸렸습니다!
}
