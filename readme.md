# Hexgem in Rust

Just a toy project to learn Rust

### personal note:

Hexgem is a typical match 3 game, but with an hexagonal board instead of a rectengular one.
The underlying datastructure is a simple Vec<T>. T here would be a struct representing Gems.
What I would like to do was to add some iterator for this Vec, like a ring or straight lines in x y or z axis. So I could modify T while iterating over my board.
At first, I tried to implement an iter_mut(), but it appear that it's not realy doable without unsafe code.
The fisrt solution was hard to find for me, but quite easy to implement. Just wrap T in RefCell. After that, my Vec can be immutable, and T mutable. There is no need for mut_iter. RefCell have a runtime cost though (it verify ownership at runtime)
The second solution is similar, but use Cell to encapsulate mutable property inside T ()here, a Gem structure). I prefer this one since my property are not big (C like Enum, integer...). Cell don't have runtime cost.
