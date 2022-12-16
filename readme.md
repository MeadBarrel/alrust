# Alrust

Alrust is a set of tools for an alchemist in Mortal Online 2.

## Getting Started

### Creating your grimoire

Grimoire is a collection of your knowledge about the alchemical-related knowlede
in MO2, as well as your characters' skills. Alrust uses sqlite dabase files to
store that knowledge. Let's first create your first empty grimoire

```powershell
alrust2.exe db --filename db.sqlite
```

This creates a new grimoire database at db.sqlite. For now, it contains nothing
except information about existing lore skills in MO2, and even then, this
information is not full (for example lore "effectiveness" that is used to
calculate a lore multiplier when creating a potion, is not set - alrust will
assume that it's 0.66666 which is true most of the time).

Now let's create our character. There are two ways to modify your grimoire - 
you can use some 3rd party tools to modify your database (like 
`DB Browser for SQLite`), or you can use alrust's `update` command. Let's try
it.

First, let's create a file called `mygrimoire.yaml`

```yaml
db: db.sqlite

characters:

  # A list of characters goes here

  Tashka:
    advanced_potion_making: 100
    alvarin_clade: true

    # now let's define lores known to Tashka
    lores:
      Material Lore: 100
      Botany: 100
      Herbology: 100
      Steel Lore: 100
```

So, we want to create a character called Tashka with advanced potion making
skill of 100, she also has an alvarin alchemist gift. We've added some lore
values to her. Notice how Steel lore is set to 100, but it's parent skill, 
Iron-Based Alloys is unset. Alrust will consider Tashka's Steel Lore to be 0
since unset lores are assumed to be 0.

Let's tell alrust to update our database:

```powershell
alrust2.exe update --from mygrimoire.yaml --to db.sqlite
```

There, you've added Tashka to the database. Now let's fix our mistake above and
set values for Steel Lore parent skills:

```yaml
db: db.sqlite

characters:

  Tashka:
    lores:
      Iron-Based Alloys: 100
      Metallurgy: 100
```

```powershell
alrust2.exe update --from mygrimoire.yaml --to db.sqlite
```

You don't need to keep the information that's already in the database in your
config file. 

Our database still lacks ingredients, so let's add some and also give Tashka
a few more lores:

```yaml
db: db.sqlite

Tashka:
  lores:
    Botanical Oils Lore: 100

ingredients:
  Salvia Oil:
    weight: 1 # Alchemical weight for salvia oil
    lore: Botanical Oils Lore
    dh: 2.4
  Sea Dew Leaves:
    lore: Herbology
    weight: 1
    dh: 1.2
  Purified Water:
    weight: 1
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
alrust2.exe update --from mygrimoire.yaml --to db.sqlite
```

Now our grimoire contains purified water, salvia oil, and sea dew leaves. 
Enough for a potion! Let's create one! Create a file mix.yaml (or any name
that you want):

```yaml
grimoire:
  db: db.sqlite

character:
  Tashka

mix:
  Salvia Oil: 11
  Sea Dew Leaves: 11
```

Now run

```powershell
alrust2.exe experiment --config mix.yaml
```

We get this result:

```yaml
volume: 2.1
effects:
  dh: !Unknown 2.6399952
  dp: !Unknown 0.0
  hot: !Unknown 0.0
  hl: !Unknown 0.0
  pot: !Unknown 0.0
  pl: !Unknown 0.0
  a: !Unknown 0.0
healing:
  direct: !Unknown 2.6399952
  over_time: !Unknown 0.0
  per_second: !Unknown 0.0
  length: !Unknown 0.0
poison:
  direct: !Unknown 0.0
  over_time: !Unknown 0.0
  per_second: !Unknown 0.0
  length: !Unknown 0.0
ingredients:
- - Salvia Oil
  - 11
- - Sea Dew Leaves
  - 11
```

What is this? Why it says !Unknown everywhere? This is because we did not
set direct healing multiplier values for Salvia Oil and Sea Dew Leaves. 
!Unknown <value> warns you that this value may not be correct.

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
volume: 2.1
effects:
  dh: !Known 2.6399952
  dp: !Unknown 0.0
  hot: !Unknown 0.0
  hl: !Unknown 0.0
  pot: !Unknown 0.0
  pl: !Unknown 0.0
  a: !Unknown 0.0
healing:
  direct: !Known 2.6399952
  over_time: !Unknown 0.0
  per_second: !Unknown 0.0
  length: !Unknown 0.0
poison:
  direct: !Unknown 0.0
  over_time: !Unknown 0.0
  per_second: !Unknown 0.0
  length: !Unknown 0.0
ingredients:
- - Salvia Oil
  - 11
- - Sea Dew Leaves
  - 11
```

Now we see direct healing value as !Known. Other values are still unknown - 
ingredients in our database only have known values for direct healing and its
multiplier.