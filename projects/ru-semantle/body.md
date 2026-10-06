[semantle](https://semantle.com/) (**semant**ic word**le**) is a wonderful
[wordle](https://www.nytimes.com/games/wordle/index.html)-inspired word-guessing
game. it utilizes [word embeddings](https://en.wikipedia.org/wiki/Word_embedding)
(in their case, `word2vec`) to calculate how "semantically close" a guess is to
the secret word.

it is fun to play, but language intuition always works better in your native language.
in my case, it's russian. so, using an open-source embeddings 
[`navec`](https://github.com/natasha/navec) and a couple of other NLP tools,
i hacked together a clone. also, i had a lot of fun writing frontend in svelte and fell
in love with this framework!
