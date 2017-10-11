extern crate peg;

fn main() {
    peg::cargo_build("src/rules/int.rustpeg");
    peg::cargo_build("src/rules/float.rustpeg");
}
