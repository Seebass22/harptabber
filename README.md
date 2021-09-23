harptabber
--------
CLI tool for transposing harmonica tabs into different positions
```
$ cat tabs.txt 
-2 -3 4 -4' -4 -4
-4 -4' -3
-4 -4' 4 -3 -4' -2
```
transpose from one position to another
```
$ harptabber --from 2 --to 1 tabs.txt
1 2 -2'' -2' -2 -2 
-2 -2' 2 
-2 -2' -2'' 2 -2' 1 
```
or by number of semitones
```
$ harptabber --semitones 5 tabs.txt 
4 5 -5 5o 6 6 
6 5o 5 
6 5o -5 5 5o 4 
```
