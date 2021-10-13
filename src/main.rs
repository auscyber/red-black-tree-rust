#![feature(box_patterns)]
#![feature(box_syntax)]

use std::fmt::Display;
#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

use Color::*;

#[derive(Clone)]
enum Tree<T> {
    Node(Color, Box<Tree<T>>, T, Box<Tree<T>>),
    Empty,
}
use Tree::*;

fn print_tree<T: Display>(tree: &Tree<T>, n: u32) {
    if let Node(col, left, x, right) = tree {
        print_tree(right, n + 1);
        for _ in 0..=n {
            print!("\t");
        }
        println!("{:?}: {}", col, x);
        print_tree(left, n + 1);
    }
}

fn balance<T: Ord>(tree: Tree<T>) -> Tree<T> {
    match tree {
        Node(
            Black,
            box Node(Red, box Node(Red, a, x, b), y, c)
            | box Node(Red, a, x, box Node(Red, b, y, c)),
            z,
            d,
        )
        | Node(
            Black,
            a,
            x,
            box Node(Red, box Node(Red, b, y, c), z, d)
            | box Node(Red, b, y, box Node(Red, c, z, d)),
        ) => Node(Red, box Node(Black, a, x, b), y, box Node(Black, c, z, d)),
        x => x,
    }
}

fn ins<T: Ord + Clone>(tree: &Tree<T>, element: T) -> Tree<T> {
    use std::cmp::Ordering;
    if let s @ Node(col, left, x, right) = tree {
        balance(match element.cmp(x) {
            Ordering::Greater => Node(*col, left.to_owned(), x.to_owned(), box ins(right, element)),
            Ordering::Less => Node(*col, box ins(left, element), x.to_owned(), right.to_owned()),
            Ordering::Equal => s.to_owned(),
        })
    } else {
        Node(Red, box Empty, element, box Empty)
    }
}

fn insert<T: Clone + Ord>(tree: &mut Tree<T>, element: T) {
    match ins(tree, element) {
        Node(_, left, x, right) => *tree = Node(Black, left, x, right),
        Empty => {}
    };
}

fn main() {
    let mut tree = Empty;
    for i in 0..=1000 {
        insert(&mut tree, i);
    }
    print_tree(&tree, 0);
}
