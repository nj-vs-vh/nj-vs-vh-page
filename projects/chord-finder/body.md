what makes a sound [consonant or dissonant](https://en.wikipedia.org/wiki/Consonance_and_dissonance)?
one simple heuristic may be a "spectral complexity", i.e. the amount of spectral information
in the sound. the higher the complexity, the more dissonant a sound sounds to a human ear,
because the brain needs more "processing power" to make sense of this signal. of course,
this is a simplistic view of psychoacoustics, but it's easily operationalizable.
consider a chord --- a combination of several voices, each characterized simply by
a frequency $\{ f_i \}$. for each voice in the chord, one can build a harmonic series
$k f_i \quad \forall k \in [1, N]$. the chord's spectrum is simply a sum of individual voices'
spectra. to simplify, we can ignore spectral lines' relative amplitudes and just consider them
either "on" or "off". by merging the harmonic series of the voices and removing duplicates
(with some tolerance, since in the [equal temperament](https://en.wikipedia.org/wiki/Equal_temperament)
no two harmonics frequencies will be equal), one can estimate the "spectral complexity" of a
given chord as the number of distinct spectral lines in it's combined harmonic series.

once we have a way to quantify the chord's consonant-dissonant quality, we can generate
some nice-sounding chords. the simplest way is Monte-Carlo: generate a bunch of random
combinations of pitches and pick the one with the least spectral complexity, hopefully it would
sound the _least dissonant_ and nicest. of course, one could also be interested in the _most_
dissonant chord as well, but this is likely to be a trivial [cluster](https://en.wikipedia.org/wiki/Tone_cluster).
in this setup, the more Monte Carlo sample size is, i.e. the more attempts the algorithm makes,
the more likely it is to stumble on something very consonant, like a bunch of octaves. so, by
limiting this number one can make the algorithm more "adventurous", picking more spicy (or just bad)
chords.

## implementations

a while ago i [tried building in Matlab](https://github.com/nj-vs-vh/chord-finder/tree/main/prototype)
a chord generator based on similar principles. it was very buggy but somewhat functional and 
very fun to play with. recently i re-discovered this project and rebuilt it in vanilla HTML+JavaScript
with the wonderful [Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API).
it's rewritten from scratch and seems to be much more stable and predictable.
[play](https://chord-finder.nj-vs-vh.name/) with it here --- hover over the "?" signs
to see what the parameters do.
