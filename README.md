!!!STILL IN DEVELOPMENT!!!

Basically GML with more features

Feature #1 - Type Anotation:
You can add type anotation for functions and variables

```

let life: float = 20;

function is_dead(life) -> bool {
    return life <= 0
}

```

If you do this, the compiler will throw an error:

```

let name: string = "Text";
name += 2;

function get_pi() -> number {
    return "pi";
}

```

You can also create generics:

```
function print<T>(value: T, other_value: T) {
    show_debug_message(value);
    show_debug_message(other_value);
}
print<number>(2, 4); //This is valid
print(2, 5); //This is valid
print<number>(2, "4"); //This isn't valid
print(2, "4") //This isn't valid either
```

This is the list of types:
```
- float 
- int
- number
- string
- char
- T[] //For arrays
- DsList<T>
- Buffer<T>
- DsQueue<T>
- DsStack<T>
- DsMap<K, V>
- DsPriorityQueue<T>
- Matrix
- Tuple(T, G, Q, O...) //For tuples
```

You can use functions as types:
```
alert: Fn = fn() => show_message("HI");
filter_by_evens: Fn(number)<bool> = fn(n) => n%2==0;
show_numb: Fn(number) = fn(n) => show_message(n);
```

And you can also create your own types:
```
type filter = Fn(number)<bool>;
```

Feature #2 - Advanced Enums:
You can make enums like in rust:

```
enum Shape {
    Circle(float),
    Rectangle(float, float),
    Triangle(float, float, float)
}

let my_circle = Shape::Circle(5);
```

OBS: When accessing enums, you can use "::" or "."

Feature #3 - Arrow functions:
You can make arrow functions, but with the "fn" keyword, to make it more clear

```
const sum = fn(a, b) => a + b;
let numb = sum(a, b);
```

Feature #4 - const, let and var:
There are three types of variables:

```
const: Can't be changed in any way
let: normal gamemaker instance variables
var: normal gamemaker event variables
```

Feature #5 - Class keyword:
You can use still use constructors, but classes have additional features

Classes don't have a constructor, instead, you can create a method inside that class that creates a new instance of that class

```
class Person {
    public life: int;
    public name: string;

    public new(life: int, name: string) -> Self {
        return Person {life, name};
    }
}
var james = Person::new(20, "james");
```

If an instance of a class calls one of its methods, the the first argument of the method will be the instance itself.
But if the class calls the method, so the first argument needs to be given manually.

```
class NumberContainer {
    public numbers: Array<number>
    
    public sum_numbers(self: Self) -> number {
        return self.numbers.reduce(fn(a, b) => a + b);
    }
}
var container = NumberContainer {numbers: [2, 5]};
show_debug_message(container.sum_numbers) //7
show_debug_message(NumberContainer::sum_numbers(container)) //7
```

You can set parts of a class as private or public:

```
class Person {
    public name: string;
    public age: int;
    private ip: string;

    public new(name: string, age: int, ip: string) -> Self {
        return Person {name, age, ip};
    }
}
var john = Person::new("John", 23, "123.123.12.22");
var name = john.name; //This is ok
var next_age = john.age + 1 //This is ok too
var ip = john.ip //This throws an error 
```

And you can also set parts of a class as readonly

```
class Animal {
    public readonly name: string;
    public age: int;
}
var bunny = Animal {name: "bunny", age: 3};
bunny.age += 1; //This is ok
bunny.name = "ThePrimeagen" //This throws an error
```

Feature #6 - Option Enum:
Yeah, you can use it:
```
function get_name(index: Option<int>) -> string {
    switch (index) {
    case 1:
        return Some("John");
    case 2:
        return Some("Karen");
    default:
        return None;
    }
}
```


Feature #7 - Data structures as structs:
You can use methods in data structures as you would do in javascript or rust:

```
var array = [2, 5, 6];
var str_array = array.map(fn(n) => string(n));
var sorted_array = array.sort(fn(a,b) => b-a);

var two_str = "2";
var two_num = two_str.parse().unwrap();
```

Feature #8 - Iterators:
You can iterate through some data structures using the for-in syntax

```
my_arr = [2, 5, 6];
for (numb in arr) {
    if (numb == 2) {
        show_message("I FOUND IT");
    }
}

var action_map: DsMap<char, Fn> = ds_map_create();
action_map[? 'a'] = fn() => show_message("GOING TO THE LEFT");
action_map[? 'A'] = fn() => show_message("GOING TO THE LEFT");
action_map[? 'd'] = fn() => show_message("GOING TO THE RIGHT");
action_map[? 'D'] = fn() => show_message("GOING TO THE RIGHT");

for ((key, func) in action_map) {
    if (keyboard_string.last() == key) {
        func();
    }
}
```

Feature #9 - High Order functions:
You can create high order functions like you would do it in javascript:

```
function checker(name) -> Func(string)<bool> {
    return fn(str) => str == name;
}

var fireship_checker = checker("fireship");

for (channel in youtube_channels) {
    if (fireship_checker(channel)) {
        show_message("I FOUND IT!");
    }
}
```

Feature #10 - Pattern Matching:
You can use switch, but you can also use the match statement:

```
match Shape {
    Shape::Circle(area) => show_message($"The radius of the circle is: {area}"),
    Shape::Rectangle(width, height) => show_message($"The width is {width} and the height is {height}"),
}
```

When use match, you MUST handle all the possible classes

```
match Coordinate {
    Coordinate::X() => show_message("IM THE X COORDINATE"),
} // this throws an error, because we aren't handling Y or Z
```

You can use "_" for unexpected cases

```
match Animal {
    Animal::Bunny(name) => show_message($"I'm {name}"),
    Animal::Bee(color) => show_message($"My color is {color}"),
    _ => show_message("????????????????")
}
```

Feature #11 - Tuples:
You can create tuples:

```
var position = (0, 1, 5);
```

And you can access the values inside them using this syntax:

```
var position = (0, 1, 5);
var (x, y, z) = position;
```

OBS: You can also do this with arrays and structs:
```
var array = [8, 24, 84];
var [baby_age, adult_age, emacs_user_age] = array;

var person = {
    age: 32,
    name: "Tom"
};
var {tom_age, tom_name} = person;
```

Feature #12 - Error Handling:
You don't need to have a variable to hold the error when using a try-catch block

```
try {
    *my horrible code*
} catch {
    *idk, show the values of the variables?????????*
}
```
