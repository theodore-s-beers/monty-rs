# Monty Hall problem simulation

I've always found
[this problem](https://en.wikipedia.org/wiki/Monty_Hall_problem) fascinating and
counterintuitive. You pick one door of three, and of course you have a one-third
chance of having chosen the correct door. The host then eliminates from
consideration one of the other two doors—which is guaranteed _not_ to be the
correct one. He offers you the chance to switch your choice to the third door.
What should you do? What are your odds at this point if you stay, or if you
change? I would have guessed that it's a wash: there are now two doors, and you
should have a one-half chance of winning regardless of what you do. In reality,
the one-third chance from your initial guess is "locked in," and the alternative
door—the non-eliminated one—has a _two-thirds_ chance of being correct.
Switching is the winning strategy, by a long shot. How crazy is that? I used to
have a hard time understanding it, so I wrote a simulation to convince myself of
the answer. To run: `cargo run`.
