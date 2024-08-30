# First 3 lines, single threaded
tl;dr: Saw a 42% decrease in time spent

## base after adding counters
```
28s - 295,702/295,729 @ 9,953.2029/s
State { connections: 10, idle_connections: 9 }
Stats = {"total_searched": 295841, "rediscovered": 2708283, "total_processed": 295841, "found_terminal": 62379}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```

## Board.get() pointers
```
23s - 295,764/295,793 @ 12,639.088/s
State { connections: 10, idle_connections: 9 }
Stats = {"total_processed": 295841, "total_searched": 295841, "found_terminal": 62379, "rediscovered": 2708283}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```

## Simpler board destruction
```
21s - 295,635/295,668 @ 13,585.1613/s
State { connections: 10, idle_connections: 9 }
Stats = {"found_terminal": 62379, "total_processed": 295841, "total_searched": 295841, "rediscovered": 2708283}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```

## Smarter gravity application
```
22s - 295,515/295,562 @ 13,119.5761/s
State { connections: 10, idle_connections: 9 }
Stats = {"rediscovered": 2708283, "total_searched": 295841, "found_terminal": 62379, "total_processed": 295841}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```
didn't help, so stashed!

## making terminal board lookup faster
```
18s - 295,387/295,437 @ 16,284.2998/s
State { connections: 10, idle_connections: 9 }
Stats = {"total_processed": 295841, "total_searched": 295841, "rediscovered": 2708283, "found_terminal": 62379}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```

## Better string & vec handling!
```
16s - 295,541/295,583 @ 18,485.5193/s
State { connections: 10, idle_connections: 9 }
Stats = {"total_processed": 295841, "rediscovered": 2708283, "found_terminal": 62379, "total_searched": 295841}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```

## After adding some more stats & dropping the db state print
```
16s - 295,841/295,841 @ 18,605.5105/s
has_path: queries = 125301367, hits 125292032, hit ratio = 99.9925, db queries = 9335
 is_word: queries = 35951396, hits 35949519, hit ratio = 99.9948, db queries = 1877
Stats = {"already_queued_previously": 1241, "rediscovered_searched": 2563819, "total_searched": 295841, "total_processed": 295841, "found_terminal": 62379, "already_queued_this_board": 143223}
Found 62379 unique terminal boards
Highest scoring had a score of 253
Using a path of: ["ons", "mics", "rich", "dopiest"]
```