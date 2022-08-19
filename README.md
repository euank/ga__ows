## Guillotine

This is a very basic terminal-based game which was originally inspired by a
combination of Wordle and Hangman.

There's a very high chance that this game has been invented before, but I do
not perticularly want to google for it to find out.

So, what's the game?

You are given a hangman like set of letters all filled in, and a set of letters
that are _not_ in the word. It is your goal to figure out the single common
word that can fit.

For a very simple example, suppose the blanks are:

<u>C_A_T</u>

Now, from just these blanks, it's hard to figure out what common word it is. All of "chant", "chart", "coast", and "craft" fit into those blanks correctly.

However, if the "invalid letter" letterbank had:

~~H~~, ~~E~~, ~~R~~

that would be sufficient to narrow it down to "coast" being the correct word.

This game always gives a prompt and letterbank which has exactly one correct
answer, based on a list of common english words.

### Current status

Not actually fully implemented, but something!

### Why the name?

This is effectively just playing the last turn of a hangman game, hence
guillotine. It also feels appropriate to have a hangman derived game have a
similarly awful name.
