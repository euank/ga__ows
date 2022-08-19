## Ga\_\_ows

Ga\_\_ows (or *Ga*me *o*f *W*ord*s* if you like) is yet another word game.

This is a very basic terminal-based game which was originally inspired by a
combination of Wordle and Hangman.

There's a very high chance that this game has been invented before, but I do
not particularly want to google for it to find out.

So, what's the game?

You are given a hangman-like set of letters and blanks, and a set of letters
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

Not actually fully implemented, but there's something and it works :)

### Why the name?

It's a pun on "hangman" since the only word that fits into "Ga\_\_ows" is
gallows. But I didn't feel like gallows humor was appropriate for the actual
name, so it's left in a footnote.

This is effectively just playing the last turn of a hangman game, so a
derivative name seems appropriate to me.
