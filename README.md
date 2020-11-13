# nilszen-task-8
The binding of isaac klon skriven i rust.

## Utveckling
Utveckling under de 2 veckorna.

### 06/11-20 (en vecka efter vi fick uppgiften)
[Här](https://youtu.be/FO3xSMTCcX0) finns en video på hur spelet ser ut och fungerar just nu. Det som fungerar nu är:
* HP på spelare och fiender
* rum generation (man kan gå tillbaka till ett tidigare rum utan att nya fiender spawnar, dvs state:en av rummet sparas)
* röra sig
* skjuta
* dö
* se hur många fiender man har dödat (efter man själv har dött)
* säkert fler saker som jag inte kommer på nu.

### 13/11-20 (två veckor efter vi fick uppgiften)
[Här](https://youtu.be/RRVNMzsyr48) finns en video på hur spelet ser ut och fungerar efter två veckor. Det som är nytt är:
* fler bakgrunder på rummen
* start meny
* game over skärm
* tidsbegränsning i rummen
* hp, score och tid kvar in game
* high score lista i game menu
* säker mer.

## Info
`./_regular` är bara regular uppgiften och har ingeting med mitt spel att göra.

## How to
### Starta spelet
Starta spelet genom att antingen köra `RUN.sh` eller `cargo run --release`. 

### In-game
`w a s d` för att röra sig. Piltangenterna för att skjuta.
