
# eleven elves
> text based simulation game
## abstract
- a game based on managing a colony of elves as they fight off goblins, human attacks and starvation
- it has a solely tui-based interface, allowing you to manage elves, control your stocks, combat invasions, manage production of food, control industry, and master your kingdom
- each colony starts from one launch area (the first stronghold) , but it can branch out into multiple strongholds
- multiple colonies can exist on the same map
- server and multiplayer functionality can be made such that colonies can exist on one map and interact (i.e. trade and fight)
## game components
### map
- one map can hold a pre-determined (or infinite) number of strongholds and colonies
- maps can be networked and shared between players
- strongholds in a map are always a set difference of travel time apart - in order to combat another stronghold, one must "stronghold-hop", as the elven colony can only jump one distance in a go without time to restock and recharge. the strongholds taken over are added to the colony.
- a map is filled with strongholds - most will be empty, but some will contain humans or goblins

### colony
- colonies are the base unit of eleven elves - each "game" is a colony, which can have unlimited strongholds
- a colony is the combination of each stronghold controlled by the player
- colonies begin at the start of each game with eleven initial elves

### stronghold
- strongholds are the units of control of eleven
- strongholds are either empty, goblin controlled, human controlled or player controlled
- the player begins with a single level one strongholds
- various aspects of each stronghold can be levelled up (e.g. size, movement efficiency, various defence stats)

### elf
- the elf is the most basic unit
- each elf has various traits (ex. easily displeased, eats lots) that influence its resource production and consumption
- elves will do any tasks either
	- specifically assigned to them
	- assigned for all elves or a group (either a work group or a profession group)
- elves require food and drink to live - each food and drink item gives different pleasure or displeasure statistics, but all food items will ease hunger, albeit by different amounts, and all drink items will quench thirst
- elves require various other things for continued happiness, for example a job and a bed
- if an elf's happiness, hunger or thirst level decreases below a certain point they will begin taking damage as if in a fight until all 3 stats are above the critical point
### group
- there are two types of group, work groups and profession groups
- all elves belonging to a certain profession will be part of the stronghold's profession group for said profession
- work groups can consist of elves of various roles and are manually assigned by the player
- any tasks started at a workstation (eg carpenters' workshop) will be assigned to the right profession group
- any tasks can be assigned instead to a work group, where any and all eligible members will perform that task
