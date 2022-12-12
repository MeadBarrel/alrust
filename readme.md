# Alrust

Welcome to alrust, a set of tools to help alchemists in Mortal Online 2. 
The alrust tools include two commands: genetic and experiment.

`Genetic` runs a genetic algorithm to find the best potion mix given a set of 
desired effects. `Experiment` allows the user to create a potion from selected 
ingredients and see its properties.

In order to use the alrust tools, you will need to create a configuration file 
in markdown format. Examples of configuration files for the genetic and 
experiment commands are provided below.

## Genetic alrogithm configuration

The genetic command uses a configuration file in markdown format to run a 
genetic algorithm to find the best potion mix. The configuration file allows 
the user to specify the population size, the frequency of output, and the name 
of the database. It also allows the user to specify parameters for mutation, 
such as the ratio of amount growth and the minimum amount growth. Additionally, 
the user can specify the desired effects to maximize in the resulting potion mix. 
An example configuration file is shown below:

```yaml
population_size: 100
output_every: 10000
output_folder: "output"
db_name: "../db.sqlite"

mutate:
   amount_grow_ratio: 0.25
   min_amount_grow: 1
   num_mutations_amt: 8
   num_mutations_ing: 2

desired_effects:
   - MaximizeHOT
   - MaximizeHL
```

The list of possible objectives are:

    - MaximizeDH
    - MaximizeDP
    - MaximizeHOT
    - MaximizePOT
    - MaximizeHL
    - MaximizePL
    - MaximizeA


## Experiment configuration

The experiment command uses a configuration file in markdown format to specify 
the details of the potion to be created and tested. The configuration file 
allows the user to specify the name of the database to be used, as well as the 
alchemist who will be creating the potion and the mix of ingredients to be used. 
It also allows the user to specify any assumptions about the properties of the 
ingredients in the mix. An example configuration file is shown below:

```yaml
db: ../db.sqlite

character:
  name: Tashka
mix:
  Salvia Oil: 32,
  White Bear Carcass: 16
assume:
  Salvia Oil: # Modify the properties of Salvia oil
    dh: 0
    dp: 2.1
```

In this example configuration file, the experiment command will use the database 
located at ../db.sqlite, and will create a potion by the alchemist named Tashka 
using 32 units of Salvia Oil and 16 units of White Bear Carcass. The 
configuration file also specifies that the experiment command should assume that 
the properties of Salvia Oil in the mix are dh: 0 and dp: 2.1.    