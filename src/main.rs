// Slice Type 3 . 25.02.24
// 슬라이스 타입 3

/*
String 에 대한 참조자 대신, 문자열 슬라이스를 매개변수로 하는 함수를 정의하면
기능 면에서 손해보지 않으면서 API를 더 일반적이고 유용하게 만든다.
*/

fn main() {
    // 'first_word'는 'String'의 일부 혹은 전체 슬라이스에 대해 작동한다.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 또한 'first_word'는 'String'의 전체 슬라이스와 동일한 'String'의 참조자에 대해서도 작동한다.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // 'first_word'는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동한다.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 문자열 리터럴은 곧 문자열 슬라이스이므로 아래의 코드도 슬라이스 문법 없이 동작한다.
}
