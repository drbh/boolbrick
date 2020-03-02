use boolblock::*;

fn main() {
    // let _y = process();
    // let _y = example();

    let data = r#"
	{
	    "title": "Lesson 2",
	    "cards": [
	        [
	            [
	            {
	                "value": "MAT 1",
	                "values": null,
	                "kind": "Exact"
	            }],
	            [
	            {
	                "value": "MAT 2",
	                "values": null,
	                "kind": "Exact"
	            },
	            {
	                "value": "MAT 3",
	                "values": null,
	                "kind": "Exact"
	            }]
	        ],
	        [
	            [
	            {
	                "value": null,
	                "values": ["ART 1", "ART 2", "ART 3", "DES 1", "DES 2", "DES 3"],
	                "kind": "OneOf"
	            }]
	        ]
	    ]
	}"#;

    let v: Brick = serde_json::from_str(data).unwrap();

    println!("{:#?}", v);
}
