# How to write library for worldlang

Prototypes will given as pseudo code.

## I/O

### println
```ts
println(...args :any)->void_type
```
Print arguments on stdout and new line.
If argument is void_type, print `void`.
If argument is not printable, print `(type of object:size of object)` instead.
No space between arguments.

### print
```ts
print(...args :any)->void_type
```
Print arguments on stdout.
If argument is void_type, print `void`.
If argument is not printable, print `(type of object:size of object)` instead.
No space between arguments.

### input
```ts
input(q :string = "")->string
```
Print q as question and get single string in stdin by `" " "\n" "\t" "\r"` and return

### input_line
```ts
input_line(q :string = "")->string
```
Print q as question and get single line from stdin and return

### static_input, static_input_line
```ts
static_input(etag :integer, q :string = "")->string
static_input_line(etag :integer, q :string = "")->string
```
Print q as quistion and return what is input and mark the input in `etag` and return.
If the function called some more, then find matching input by `etag` and return.
`static_input_line` works for line

## collections

### tup
```ts
tup(...args :any)->tuple<type(args)...>
```
Return tuple made of `args`.

### vec
```ts
vec(first :any, ...args :type(first))->vector<type(first)>
```
Return vector made of `args`.

### until
```ts
until(begin :integer, end :integer)->range
```
`range` is iterable class.
`until(1, 5)` means `[1, 2, 3, 4, 5]`

### sum
```ts
sum(c :Container)->type(c).value_type
```
Return sum of the container.

form: `((((a + b) + c) + d) + ...)`

### max
```ts
max(c :Container)->type(c).iterator_type
```
Returns iterator pointing max value of container.
`lang.operator.de_pointer` to get value of iterator.

### map
```ts
map(c :Container, fn :(type(c).value_type)->T)->vector<T>
```
Map fn over container and collect results of `fn` as vector.

### cat
```ts
cat(c1 :Container, c2 :Container)->vector<T>
```
Concat two containers and return as vector.

### each
```ts
map(c :Container, fn :(type(c).value_type)->T)->vector<T>
```
Iterate over container and run `fn` each.

### fold
```ts
fold(c :Container, fn :(T, type(c).value_type)->T)->T
```
Reduce `fn` over container.

form: `fn(fn(fn(a, b), c), ...)`

### bfold
```ts
fold(initial_value :T, c :Container, fn :(T, type(c).value_type)->T)->T
```
Reduce `fn` over container, initial value as `initial_value`.

form: `fn(fn(fn(fn(i, a), b), c), ...)`

### filter
```ts
filter(c :Container, fn :(type(c).value_type)->bool)->vector<type(c).value_type>
```
Select elements in c which satisfy `fn`.

### integrate
```ts
integrate(c :Container, fn :(T, type(c).value_type)->T)->vector<T>
```
It's hard to explain!
It acts like partial sum.

form: `[S(0), S(1), ..., S(len(c)-1)] where S(n) = fn(S(n-1), c[n]), S(0) = c(0), S(n) ∈ O(N)`

## time

### wait
```ts
wait(t :time_type)->type(t)
```
Wait for t ms.

### get_time
```ts
get_time(fn :(...any)->any, ...args :any)->void_type
```
Count time while `fn(...args)` running and show on stdout with format `{time:.6} second(s) spent. \n`.

## type exports

form | description 
-- | -- 
`string` | basic string type
`vector<T>` | basic vector type
`iB` | `B` byte signed integer type where `B` in `1 2 4 8`
`uB` | `B` byte unsigned integer type where `B` in `1 2 4 8`
`f4` | 4 byte floating point
`f8` | 8 byte fixed point
`cuB ciB cf4 cf8` | constant of them

## etc.

### make_string
```ts
make_string(s :any)->string
```
Return stringified `s`. Arguments on `print, println` print by this.

### stoi
```ts
stoi(s :string)->integer
```
Parse string into integer and return.