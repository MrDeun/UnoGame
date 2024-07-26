use rand::prelude::*;

enum Color {
  universal,
  red,
  blue,
  yellow,
  green
}

enum Type {
  zero,
  one,
  two,
  three,
  four,
  five,
  six,
  seven,
  eight,
  nine,
  reverse,
  plusTwo,
  plusFour,
  blank
}

struct Card {
  color: Color,
  value: Type,
}

impl Card {
  fn new() -> Card {
    return Card{}
  }
}