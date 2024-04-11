what makes a chord [consonant or dissonant](https://en.wikipedia.org/wiki/Consonance_and_dissonance)?
one simple heuristic may be a "spectral complexity", i.e. the amount of spectral information
in the chord should. for each voice $f_i$ in the chord one can calculate a harmonic series
$k f_i \quad \forall k \in [1, N]$. by merging the harmonic series of the chord's voices
and removing duplicates (with some tolerance, since in the
[equal temperament](https://en.wikipedia.org/wiki/Equal_temperament) no two harmonics
will match exactly), we can very easily estimate "spectral complexity" of a given chord as
the number of spectral lines in it's combined harmonic series.

a while ago i [tried building in Matlab](https://github.com/nj-vs-vh/chord-finder/tree/main/prototype)
a chord generator based on this principle. it was generating a bunch of random chords
and picking one with the least "spectral complexity". it was very buggy but somwhat functional
and actually very fun.

recently i re-discovered this project and [rebuilt](https://chord-finder.nj-vs-vh.name/)
it in vanilla JS script using the wonderful
[Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API).
it's rewritten from scratch and seems to be much more predictable. hover over the "?" signs
to see what the parameters do.
