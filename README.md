# korean-rs
![circleci](https://circleci.com/gh/kiwiyou/korean-rs.svg?style=shield&circle-token=:circle-ci-badge-token)
![crates.io](https://img.shields.io/crates/v/korean.svg)
> korean-rs는 Hangul.js의 영향을 받아 만들어진 Rust 한글 라이브러리입니다.

## 기능
- 한글 완성형 문자를 낱자로 분리합니다. (밝다 → ㅂㅏㄹㄱㄷㅏ)
- 한글 낱자를 완성형 문자로 조합합니다. (ㅂㅏㄹㄱㄷㅏ → 밝다)
- 한글 완성형 문자를 한글 호환용 자모로 분리합니다. (밝 → ㅂ, ㅏ, ㄺ)
- 한글 호환용 자모를 한글 완성형 문자로 조합합니다. (ㅂ, ㅏ, ㄺ → 밝)

## 사용법
`Cargo.toml`의 `dependencies`에 `korean`을 추가합니다.
```toml
[dependencies]
# ...
korean = "0.2"
```

## 예제
문장 속 모든 ㅇ을 ㅁ으로 바꾸는 [예제](examples/nemo.rs)입니다.
```rust
use korean::*;

fn main() {
    let input = "당신은 네모네모 멍뭉이의 저주에 걸렸습니다!";

    let nemo = input
        .disassemble()
        .map(|c| if c == 'ㅇ' { 'ㅁ' } else { c })
        .assemble();

    println!("{}", nemo); // 담신믄 네모네모 멈뭄미의 저주메 걸렸습니다!
}
```