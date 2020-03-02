# boolblock


This library aims make it easy to build boolean logic maps. These are useful for modeling many real world situations where statements are true based on a collection of contained statements.


### Bricks in tree format
```
.
└── Brick
	├── Card 1
	│	├── Choice 1
	│	│   └── Unit 1
	│	└── Choice 2
	│		└── Unit 1
	│		└── Unit 2
	└── Card 2
		└── Choice 1
			└── Unit 1
```

### Bricks

Bricks have cards. Cards all have `OR` relationships. So only 1 card must be complete to satisfy a `Brick`

### Cards

Cards have choices. Choices all have `AND` relationships. So all choices must be complete to satisfy a `Card`

### Choices

Choices have units. Units all have `OR` relationships. So only 1 unit must be complete to satisfy a `Choice`

### Unit

Units have identifying information. Units represent expressions that reduce to a boolean variable. This means that we should be able to use a Unit to reach a True/False conclusion when we evaluate a list of `Values` against `Bricks`

### Values

Values are just another name for strings that are used in Units.

