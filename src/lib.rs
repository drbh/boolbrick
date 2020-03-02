use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Brick {
    pub title: String,
    pub cards: Vec<Card>,
}

type Card = Vec<Choice>;
type Choice = Vec<Unit>;

#[derive(Serialize, Deserialize, Debug)]
pub enum MatchType {
    Exact,
    OneOf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub value: Option<String>,
    pub values: Option<Vec<String>>,
    pub kind: MatchType,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// [
//     [
//         [ "MAT 1", "MAT 2", "MAT 3" ],
//         [ "SCI 1" ],
//     ],
//     [
//         [ "BIO 1", "BIO 2" ],
//     ],
// ]
// Take [ [ MAT 1 OR MAT 2 OR MAT 3 ] AND SCI 1 ] OR [ BIO 1 OR BIO 2 ]

// [
//     [
//         [ "MAT 1", "MAT 2", "MAT 3" ],
//         [ "MAT 1", "MAT 2", "MAT 3" ],
//     ]
// ]
// Take 2 of [ "MAT 1", "MAT 2", "MAT 3" ]

// [
//     [
//         [ "MAT 1" ],
//         [ "MAT 2", "MAT 3" ],
//     ]
// ]
// Take MAT 1 AND [ MAT 2 OR MAT 3 ]

// [
//     [
//         [ "MAT 1" ],
//         [ "MAT 2" ],
//         [ "MAT 3" ],
//     ]
// ]
// Take MAT 1 AND MAT 2 AND MAT 3

// DEMO EXAMPLE BUILDING IN RUST

// pub fn example() {
//     let choice1: Choice = vec![
//         // units
//         Unit {
//             value: Some(String::from("MAT 1")),
//             values: None,
//             kind: MatchType::Exact,
//         },
//     ];
//     let choice2: Choice = vec![
//         // units
//         Unit {
//             value: Some(String::from("MAT 2")),
//             values: None,
//             kind: MatchType::Exact,
//         },
//         Unit {
//             value: Some(String::from("MAT 3")),
//             values: None,
//             kind: MatchType::Exact,
//         },
//     ];

//     let choices: Card = vec![choice1, choice2];

//     let choice3: Choice = vec![
//         // units
//         Unit {
//             // title: String::from("Arts and Design"),
//             value: None,
//             values: Some(vec![
//                 String::from("ART 1"),
//                 String::from("ART 2"),
//                 String::from("ART 3"),
//                 String::from("DES 1"),
//                 String::from("DES 2"),
//                 String::from("DES 3"),
//             ]),
//             kind: MatchType::OneOf,
//         },
//     ];

//     let choices2: Card = vec![choice3];

//     let brick: Brick = Brick {
//         title: String::from("Lesson 1"),
//         cards: vec![choices, choices2],
//     };

//     let serialized = serde_json::to_string(&brick).unwrap();

//     println!("{}", serialized);
// }
