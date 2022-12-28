# Alrust

Alrust is a set of tools for an alchemist in Mortal Online 2.

## Getting Started

### Creating your grimoire

Grimoire is a collection of your knowledge about the alchemical-related knowlede
in MO2, as well as your characters' skills. Alrust uses json files to
store that knowledge. 

The initial grimoire contains nothing except information about existing lore 
skills in MO2, and even then, this information is not full (for example lore 
"effectiveness" that is used to calculate a lore multiplier when creating a 
potion, is not set - alrust will assume that it's 0.66666 which is true most of 
the time).

Now let's create our character. You can use alrust's `update` command to do 
this. Let's try it.

First, let's create a file called `mygrimoire.yaml`

```yaml
characters:

  # A list of characters goes here

  Tashka:
    skills:
      Alchemy: 100
      Potion Making: 100
      Advanced Potion Making: 100
      Material Lore: 100
      Botany: 100
      Herbology: 100
      Steel Lore: 100

    add_clades:
      - Alchemist
```

So, we want to create a character called Tashka with advanced potion making
skill of 100, she also has an alvarin alchemist gift. We've added some lore
values to her. Notice how Steel lore is set to 100, but it's parent skill, 
Iron-Based Alloys is unset. Alrust will consider Tashka's Steel Lore to be 0
since unset lores are assumed to be 0.

Let's tell alrust to update our database:

```powershell
alrust2.exe grimoire.json update --from mygrimoire.yaml --to grimoire.json
```

There, you've added Tashka to the grimoire. Now let's fix our mistake above and
set values for Steel Lore parent skills:

```yaml
characters:

  Tashka:
    skills:
      Iron-Based Alloys: 100
      Metallurgy: 100
```

```powershell
alrust2.exe grimoire.json update --from mygrimoire.yaml --to grimoire.json
```

You don't need to keep the information that's already in the grimoire in your
config file. 

Our database still lacks ingredients, so let's add some and also give Tashka
a few more lores:

```yaml
characters:
  Tashka:
    skills:
      Botanical Oils Lore: 100

ingredients:
  Salvia Oil:
    weight: true # Alchemical weight for salvia oil
    skill: Botanical Oils Lore
    dh: 2.4
  Sea Dew Leaves:
    skill: Herbology
    weight: true
    dh: 1.2
  Purified Water:
    skill: 1
    weight: true
    dh: 0    # Direct Healing
    dp: 0    # Direct Poison
    mdh: 0   # Direct Healing Multiplier
    mdp: 0   # Direct Poison Multiplier
    hot: 0   # Healing Over Time
    pot: 0   # Poison Over Time
    mhot: 0  # Healing Over Time Multiplier
    mpot: 0  # Poison Over Time Multiplier
    hl: 0    # Healing Length
    pl: 0    # Poison Length
    mhl: 0   # Healing Length Multiplier
    mpl: 0   # Poison Length Multiplier
    a: 0     # Alcohol
    ma: 0    # Alcohol Multiplier
```

```powershell
alrust2.exe grimoire.json update --from mygrimoire.yaml --to grimoire.json
```

Now our grimoire contains purified water, salvia oil, and sea dew leaves. 
Enough for a potion! Let's create one! Create a file mix.yaml (or any name
that you want):

```yaml
mix:
  Salvia Oil: 11
  Sea Dew Leaves: 11
```

Now run

```powershell
alrust2.exe grimoire.json mix --character Tashka mix.yaml
```

We get this result:

```yaml
volume: 2.3100000000000005
effects:
  dh: !? 3.5999856
  dp: !? 0.0
  hot: !? 0.0
  pot: !? 0.0
  hl: !? 0.0
  pl: !? 0.0
  a: !? 0.0
ingredients:
  Salvia Oil: 11
  Sea Dew Leaves: 11
```

What is this? Why it says !? everywhere? This is because we did not
set direct healing multiplier values for Salvia Oil and Sea Dew Leaves. 
!? <value> warns you that this value is only theoretical and may not be correct.

Let's fix our mix.yaml and add some multiplier values

```yaml
grimoire:
  db: db.sqlite
  lores:
    Botanical Oils Lore:
      effectiveness: 0.66666
    Herbology:
      effectiveness: 0.66666

  ingredients:
    Salvia Oil:
      mdh: 0
    Sea Dew Leaves:
      mdh: 0

character: Tashka  

mix:
  Salvia Oil: 11
  Sea Dew Leaves: 11
```

Changes we make in `grimoire` section will only be used for this potion, they
will not be saved to the database. Notice that in addition to adding multiplier
values for the ingredients, we're also implicitly setting effectiveness values
for their lores. If we didn't, the value we'd receive would still be !Unknown,
because we didn't set true values for Botanical Oils Lore's and Herbology's 
effectiveness, and alrust can only assume that they're equal to 0.66666. Later 
you'll probably want to update your grimoire with these, but for now, let's
try our potion:

```powershell
alrust2.exe experiment --config mix.yaml
```

```yaml
volume: 2.3100000000000005
effects:
  dh: 3.5999856
  dp: !? 0.0
  hot: !? 0.0
  pot: !? 0.0
  hl: !? 0.0
  pl: !? 0.0
  a: !? 0.0
ingredients:
  Salvia Oil: 11
  Sea Dew Leaves: 11
```

Now we see direct healing value as !Known. Other values are still theoretical - 
ingredients in our database only have known values for direct healing and its
multiplier.