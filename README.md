# Metaheuristics

Find approximate solutions to your optimisation problem using metaheuristics algorithms

The aim of this crate is to host various Metaheuristics algorithms. Patches
implementing useful algorithms most welcome.

The documentation for this crate can be [found
here](https://docs.rs/metaheuristics).

## What are Metaheuristics

Metaheuristics are a class of stochastic optimisation algorithms. These type of
algorithms rely on randomness to jump around the search space, then sample
where they land for possible solutions. In simple terms, **metaheuristics are
structured trial and error**.

If you've got a trial and error problem, and individual trials can be compared
and ranked against each other, Metaheuristics may be your most viable option at
getting good results.

For more information, please see the
[Metaheuristics](https://en.wikipedia.org/wiki/Metaheuristic) Wikipedia
article, and [Essentials of
Metaheuristics](https://www.amazon.com/Essentials-Metaheuristics-Second-Sean-Luke/dp/1300549629).

## How can I use this crate

By implementing the `Metaheuristics` trait, the algorithms within the following modules will be
available to you. To see an example implementation, check out the [Travelling Salesman
Problem](https://docs.rs/travelling_salesman) crate.

# Examples

    let solution = metaheuristics::hill_climbing::solve(&mut problem, runtime);

# Support

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/metaheuristics/issues](https://github.com/alfiedotwtf/metaheuristics/issues)

Watch the repository and keep up with the latest changes:

* [https://github.com/alfiedotwtf/metaheuristics/subscription](https://github.com/alfiedotwtf/metaheuristics/subscription)

Feel free to fork the repository and submit pull requests :)

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Perpetual Copyright (C) to Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
