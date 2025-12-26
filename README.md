harptabber (harmonica tab transposer)
--------
A tool for transposing harmonica tabs into different positions and tunings

Available as a CLI app, native GUI version and web app. Includes a visual "tab keyboard" for quick tab input.
Displays which positions a tab is playable in without overblows or without bends and OBs.

Try the web app and download binaries at
[https://seebass22.itch.io/harmonica-tab-transposer](https://seebass22.itch.io/harmonica-tab-transposer)

### GUI app
<img width="1000" height="545" alt="tab transposer screenshot" src="https://github.com/user-attachments/assets/632a404d-477f-472b-ada2-75bc7bbd01c1" />

### CLI app
```
$ harptabber --from 2 <(echo "-2 -3' 4 -4' -4 -5 6") --output-tuning pentaharp --playable-positions --no-bends
1st  position,   -7 semitones
1 -1 2 -2 3 -3 4 

1st  position,   +5 semitones
4 -4 5 -5 6 -6 7 

1st  position,  +17 semitones
7 -7 8 -8 9 -9 10 
```
