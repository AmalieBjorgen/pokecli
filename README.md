# pokecli
Welcome to my CLI for getting basic information about Pokémon through the command-line.
This console application uses the publicly available unofficial Pokémon API https://pokeapi.co/ and is written in Rust.

# Basic usage
The commands available for this application are:

_pokecli help_

_pokecli pokedex_

_pokecli pokemon <pokemon_name>_

_pokecli berry <berry_name>_

_pokecli move <move_name>_

_pokecli item <item_name>_

_pokecli ability <ability_name>_


As of now, names with spaces in them, such as Huge Power or Hyper Potion, will require a hyphen in the console.

_pokecli ability huge-power_

_pokecli item hyper-potion_
