# Notes

## Ownership

### Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Generic Types, Traits, and Lifetimes

### Elision Rules

The elision rules don't provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won't guess what the lifetime of the remaining references have, the compuler won't guess what the lifetime of the remaining references should be. Instead of guessing the compiler throws an error that can be resolved by adding lifetime annotations.

Three elision rules

- The first rule applies to input lifetimes, and second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references that for which it can't figure out lifetimes, the compiler will stop with an error. These rules apply to ``fn`` definitions as well as ``impl`` blocks.
- The first rule is that the compiler assigns a lifetime parameter to each parameter that's a reference. In other words, a function with one parameter gets one lifetime parameter: ``fn foo<'a>(x: &'a i32);`` a function with two parameters gets two separate lifetime parameters: ``fn foo<'a, 'b>(x: &'a i32, y: &'b i32);`` and so on.
- The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: ``fn foo<'a>(x: &'a i32) -> &'a i32``.
- The third rule is that if there are multiple input lifetime parameters but one of them is ``&self`` or ``&mut self`` because this is a method, the lifetime of ``self`` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

## Smart Pointers

When do we use ``Box<T>``

- When there is a type whose size can't be known at compile time and you want to use a value of that type in a context that requires an exact size.
- When  you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so.
- When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type.

### Determining Space for non-recursive types

```rust
enum Message{
Quit,
Move {x: i32, y: i32}
Write(String),
ChangeColor(i32, i32, i32)
}
```

To determine how much space to allocate for a ``Message`` value, Rust goes through each of the variants to see which variant needs the most space. Rust sees that ``Message::Quit`` doesn't need any space, ``Message::Move`` needs enough space to store two ``i32`` values, and so forth. Because only one variant will be used, the most space a ``Message`` value will need is the space it would take to store the largest of its variants.

Boxes provide only the indirection and heap allocation; they do ot have any other special capabilities. The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.

### [How Deref Coercion Interacts with Mutability](https://doc.rust-lang.org/book/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability)

Similar to how you use the `Deref` trait to override the `*` operator on immutable references, you can use the `DerefMut` trait to override the `*` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same as each other except that the second implements mutability. The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can get a `&U` transparently. The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is *not* possible: immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile). Converting one mutable reference to one immutable reference will never break the borrowing rules. Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don't guarantee that. Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.

### Reference Counting `Rc<T>`

Imagine `Rc<T>` as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it's no longer being used. If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data's owner, and the normal ownership rules enforced at compile time would take effect.

Note that `Rc<T>` is only for use in single-threaded scenarios.

### ``RefCell<T>`` and their Interior Mutability Pattern

Interior Mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses ``unsafe`` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.

#### Enforcing Borrowing Rules at Runtime with RefCell`<T>`

Unlike ``Rc<T>``, the ``RefCell<T>`` type represents single ownership over the data it holds. So, what makes ``RefCell<T>`` different from a type like ``Box<T>`` ?

- At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
- References mut always be valid.

With references and ``Box<T>``, the borrowing rules' invariants are enforced at compile time. With ``RefCell<T>``, if you break these rules, your proram will panic and exit.

Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:

* `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
* `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
* Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.
