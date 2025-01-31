use rune::runtime::VmErrorKind::*;
use rune_tests::*;

macro_rules! op_tests {
    ($lhs:literal $op:tt $rhs:literal = $out:expr) => {
        let out: i64 = rune!(pub fn main() { let a = $lhs; let b = $rhs; a $op b});
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"pub fn main() {{ let a = {lhs}; let b = {rhs}; a {op}= b; a }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"struct Foo {{ padding, field }}; pub fn main() {{ let a = Foo{{ padding: 0, field: {lhs} }}; let b = {rhs}; a.field {op}= b; a.field }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"enum Enum {{ Foo {{ padding, field }} }}; pub fn main() {{ let a = Enum::Foo {{ padding: 0, field: {lhs} }}; let b = {rhs}; a.field {op}= b; a.field }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"pub fn main() {{ let a = #{{ padding: 0, field: {lhs} }}; let b = {rhs}; a.field {op}= b; a.field }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"pub fn main() {{ let a = (0, {lhs}); let b = {rhs}; a.1 {op}= b; a.1 }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"struct Foo(padding, a); pub fn main() {{ let a = Foo(0, {lhs}); let b = {rhs}; a.1 {op}= b; a.1 }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"enum Enum {{ Foo(padding, a) }}; pub fn main() {{ let a = Enum::Foo(0, {lhs}); let b = {rhs}; a.1 {op}= b; a.1 }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"pub fn main() {{ let a = Ok({lhs}); let b = {rhs}; a.0 {op}= b; a.0 }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);

        let out: i64 = rune_s!(&format!(
            r#"pub fn main() {{ let a = Some({lhs}); let b = {rhs}; a.0 {op}= b; a.0 }}"#,
            lhs = $lhs, rhs = $rhs, op = stringify!($op),
        ));
        assert_eq!(out, $out);
    }
}

macro_rules! error_test {
    ($lhs:literal $op:tt $rhs:literal = $error:ident) => {
        assert_vm_error!(
            &format!(
                r#"pub fn main() {{ let a = {lhs}; let b = {rhs}; a {op} b; }}"#,
                lhs = $lhs, rhs = $rhs, op = stringify!($op),
            ),
            $error => {}
        );

        assert_vm_error!(
            &format!(
                r#"pub fn main() {{ let a = {lhs}; let b = {rhs}; a {op}= b; }}"#,
                lhs = $lhs, rhs = $rhs, op = stringify!($op),
            ),
            $error => {}
        );

        assert_vm_error!(
            &format!(
                r#"pub fn main() {{ let a = #{{ padding: 0, field: {lhs} }}; let b = {rhs}; a.field {op}= b; }}"#,
                lhs = $lhs, rhs = $rhs, op = stringify!($op),
            ),
            $error => {}
        );

        assert_vm_error!(
            &format!(
                r#"pub fn main() {{ let a = (0, {lhs}); let b = {rhs}; a.1 {op}= b; }}"#,
                lhs = $lhs, rhs = $rhs, op = stringify!($op),
            ),
            $error => {}
        );
    }
}

#[test]
fn test_add() {
    op_tests!(10 + 2 = 12);
    error_test!(9223372036854775807i64 + 2 = Overflow);
}

#[test]
fn test_sub() {
    op_tests!(10 - 2 = 8);
    error_test!(-9223372036854775808i64 - 2 = Underflow);
}

#[test]
fn test_mul() {
    op_tests!(10 * 2 = 20);
    error_test!(9223372036854775807i64 * 2 = Overflow);
}

#[test]
fn test_div() {
    op_tests!(10 / 2 = 5);
    error_test!(10 / 0 = DivideByZero);
}

#[test]
fn test_rem() {
    op_tests!(10 % 3 = 1);
    error_test!(10 % 0 = DivideByZero);
}

#[test]
fn test_bit_ops() {
    op_tests!(0b1100 & 0b0110 = 0b1100 & 0b0110);
    op_tests!(0b1100 ^ 0b0110 = 0b1100 ^ 0b0110);
    op_tests!(0b1100 | 0b0110 = 0b1100 | 0b0110);
    op_tests!(0b1100 << 2 = 0b1100 << 2);
    op_tests!(0b1100 >> 2 = 0b1100 >> 2);
    error_test!(0b1 << 64 = Overflow);
}

#[test]
fn test_bitwise_not() {
    let out: i64 = rune! {
        pub fn main() { let a = 0b10100; !a }
    };
    assert_eq!(out, !0b10100);
}
