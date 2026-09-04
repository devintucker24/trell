# Trell

Trell is a language of **named meaning-axes**. You do not prompt a model. You declare the coordinate system that a situation lives in, place grains of meaning on it, and move them. Every move casts a **shadow**: the meaning that did not survive.

That is the whole bet. Foundation models already live in unnamed geometric spaces. Ordinary languages can only talk to those spaces through strings. Trell makes a slice of the space programmable, and makes loss a value.

## Thesis

**What it is.** A Trell program is a small ontology of bipolar axes (`warmth`, `certainty`, `scale`, whatever this situation actually turns on), a handful of *offers* (passages pinned at coordinates — the space's native tongue), and a walk. `feel` projects text onto the axes. `along` moves a grain. `speak` crystallizes by concatenative synthesis: it reads the offers that sit nearest the grain's current coordinate. `shadow of` is the dual walk, the discarded tone.

**Who writes it.** Anyone who currently stuffs adjectives into a prompt and cannot ask, afterward, *what did we lose when we made this warmer?* Editors. Safety reviewers. People designing the voice of a system that must be able to show its work. Writers using generation as a material, not as an oracle.

**Why this cannot be a library.** In Python, an embedding is an anonymous float vector, a prompt is a string, and "what we stripped out to sound kinder" is a comment you forgot to keep. Trell's evaluation model is geometric: identifiers denote positions, control flow branches on resonance (`when scene ~ "abandonment"`), and the type environment *is* the `axes` block. Offers are not a dataset you pass to a function; they are the denotation of `speak`. A library would still be a general-purpose language with a hole in the middle. Trell has no other kind of value.

**What it is not.** Not a nicer calculator. Not Python with `llm_call()`. Not LMQL, Guidance, or Outlines (those constrain *tokens*). Not DSPy (that optimizes *signatures*). Not BAML (that types *prompts*). Not Osgood's questionnaire (that *measures* on three fixed axes). Trell lets you name the axes of *this* program, navigate them, and hold the residue.

## A striking program

```
axes {
  warmth:    "ice, chart, fluorescent, instruments" <-> "ember, hands, held, darling"
  certainty: "perhaps, i wonder, seems"             <-> "always, i swear, must"
  scale:     "one bed, a single night"              <-> "a nation, a century, everyone"
}

offer at warmth=0.12, certainty=0.84, scale=0.16:
  "The ward was closed at 03:10. Lights out. No next of kin present."

offer at warmth=0.22, certainty=0.70, scale=0.78:
  "Hospitals across the region reported empty corridors. The figures will be released."

offer at warmth=0.90, certainty=0.24, scale=0.12:
  "I keep thinking of you in that room. I don't know if you were cold."

offer at warmth=0.86, certainty=0.80, scale=0.14:
  "I was there. I turned the last light off. I did not leave you."

grain scene = feel "a nurse turning off the fluorescent lights in an empty ward at 3am, charts and instruments left out"

grain letter = scene
  along warmth toward 0.88
  along certainty toward 0.28
  keeping scale

speak letter
speak shadow of letter
```

`letter` is the scene walked toward intimacy and doubt, with scale frozen — it remains one night, one room. `speak letter` reads the offers that live there. `speak shadow of letter` is the blotter: the clinical certainty the walk left behind. Same situation. Two positions. A receipt for the move.

## Run it

```bash
cargo run -- examples/letter.trell
```

No LLVM. No model API key. The runtime projects onto your axes with a concept lexicon plus n-gram residue, then speaks by mosaic from the offers you placed. A larger model can replace the substrate later; the language does not change. The unique thing is not the embedder. It is that **loss is first-class**.

## Language

| Form | Meaning |
|---|---|
| `axes { name: "low pole" <-> "high pole" }` | Declare the coordinate system. Poles *are* the axis. |
| `offer at axis=0.2, ...: "passage"` | Pin a voice at a point. This is how the space talks. |
| `grain x = feel "text"` | Project text onto the axes. |
| `x along warmth toward 0.9` | Move. Default `by 1` (go all the way). `by 0.5` goes halfway. |
| `keeping scale` | Freeze axes for every `along` / `without` in this walk. |
| `path name { ... }` then `x via name` | A reusable walk. |
| `x without feel "hospital fluorescent"` | Repel from a meaning. Casts a shadow of what was stripped. |
| `blend a with b by 0.4` | Interpolate two grains. |
| `x with shadow` | Fold the discarded meaning back in. |
| `speak x` | Crystallize at x's coordinate. |
| `speak shadow of x` | Crystallize what the last walks threw away. |
| `echo x` / `echo space` | Print coordinates, or the whole map. |
| `when x ~ "phrase" { ... } else { ... }` | Branch on resonance. |
| `when x.warmth > 0.7 { ... }` | Branch on a named axis. |
| `// comment` | Comment. |

Values are grains: a coordinate, a residual embedding, some source text, and maybe a shadow. There are no integers. There is no `main`. A program is a walk through a space you named.

## Examples

- `examples/letter.trell` — the program above. Scene → letter, then its shadow.
- `examples/voices.trell` — one incident, two destinations: blotter and eulogy.
- `examples/winnow.trell` — `without` strips a meaning and speaks both the cleaned grain and the shadow.
- `examples/fork.trell` — control flow on axis position and on resonance.
- `examples/recipe.trell` — a named `path` applied twice, to different grains.

```bash
cargo test
cargo run -- examples/voices.trell
```
