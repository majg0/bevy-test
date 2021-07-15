# Tasking

## Flow

1. Player creates Designation (farm / chop / mine).
1. Designation creates Tasks (farm / chop / mine specific block).
1. Tasks prioritized by Player.
1. High priority Task picked up by Worker.
1. Worker creates a Program of Instructions and required Items by recursively solving problems posed by the task.
1. Worker makes Reservation for necessary Items.
1. Instructions are executed by Worker.
1. Failed execution creates a new program or aborts.
1. Worker finishes Program and deletes Task.

## Scenario 1: relatively basic

1. Player designates "Mine block x".
1. Designation creates task "Mine block x".
1. There is nothing to prioritize between.
1. Worker picks up the task and inspects its requirements.
1. Worker starts coming up with a solution to the task.
    1. Worker can't mine, but chooses to try to grab a pickaxe.
    1. Worker isn't standing next to a pickaxe, but chooses to try to get to one.
    1. Worker comes up with a path to a pickaxe and adds it as the first instruction, along with remembering the pickaxe for potential reservation.
    1. Worker adds the second instruction to pick up the pickaxe.
    1. Pickaxe requires to stand next to the block to mine, so Worker chooses to try to get next to the block.
    1. Worker comes up with a path to the block and adds this as his third instruction.
    1. Worker adds the fourth and final instruction: mine the block.
1. Worker reserves the pickaxe.
1. Worker executes each program instruction over several seconds.
1. Luckily nothing is obstructed.
1. The block is mined and the task is removed.

## Scenario 2: obstruction

Some instruction can for some reason no longer be executed.

Questions: when is this detected?

Example: terrain changes, a path is obstructed - not necessarily the one being followed - and potentially for just an insignificant moment - or maybe until it's too late?

## Scenario 3: work stealing

A worker which would be better off doing the job becomes available.

Introduces: estimates and work stealing.

## Scenario 4: task may have many workers

Introduces: Task can not simply be claimed, need to count workers instead.

## Scenario 5: task must have many workers

Introduces: Several must become ready before devising a solution.

By introducing completion time/position estimates, we calculate a lower bound on when a full taskforce can become ready. We immediately allocate the required workers via looking at estimates and allow the ones first ready to potentially do jobs in between waiting for the last workers. Once the full taskforce is ready, a solution is devised.

## Scenario 5b: workers have different roles

Introduces: Must distinguish between workers. Solution devised must involve several processes with potential forks and joins between them.

## Scenario 6: there is no solution available

Questions: How to detect? Notify player? All task types?

<!-- ## Tasking Scenario 2 - collaboration

- Player wants to "build this big thing"
- (minimum) 1 dwarf with hammer needs to go build it
- it needs hammer
- it should be "close by"
- select the best of the following
  - distance to closest dwarf wielding a hammer (if there is one)
  - distance between closest hammer + its closest dwarf

-->
