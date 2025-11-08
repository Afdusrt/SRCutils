# SRCutils
Utilities made in Rust for interacting with the speedrun.com platform via curl requests.

## Features
<b>edl mode:</b>
<p>converts a given edl(cmx3600) file into a dsv file (° as the delimiter) whilst adding timestamps to a given youtube link based on the edl(cmx3600) file</p>
<b>sub mode</b>
<p>submits speedruns from a dsv file (° as the delimiter) to speedrun.com using curl</p>
<p>It does this by taking an example request and iterating it with level ids and other different submission fields</p>

## Dependencies
<p>The program relies on:</p>

```rust
std::process::Command("curl").arg(...
```
to send requests to speedrun.com, so make sure curl is on your path.
## Usage
## Edl mode
<b>To get a compatible edl file:</b>
<p>Name your individual speedrun video files in this format:</p>

```shell
Level_name--h-m-s-ms.mp4 (extension does not matter)
```

<p>If you don't need to use hours or other, you can just use the ones you need from the right:</p>

```shell
Level_name--s-ms.mp4
```

<p>There has to be a double dash before the time, since the script searches for it to split between level name and time, it the splits the time into pieces with a singular dash.</p>
<p>Now you can edit your run, export the video, export the edl file (I have tested the video editor known as ShotCut) and upload your video to the YouTube platform (this is because the script appends &t=x to timestamp the speedruns)</p>

### Using it

```shell
$ SRCutils edl youtube_video_link edl_file_path*
```

<p>First argument sets the mode.</p>
<p>The youtube video link must not be of shortened format, it should be the full one like this: https://www.youtube.com/watch?v=00000000000</p>
<p>The edl_file_path argument is optional as you can input "//" to default to "edl.edl"</p>
<p>The program will output to a file named: output.csv (this is so you can open it easily in a spreadsheet processer like LibreOffice Calc)</p>

## Sub mode
```shell
$ SRCutils sub game_abbreviation output.csv example_command.txt
```

<p>First argument sets the mode.</p>
<p>You get the game abbreviation from the speedrun.com url of a game. </p>
<p>The "output.csv" argument is the filepath to your csv file that uses "°" as the delimiter (If you use the edl mode to make your csv file then you dont have to worry about it, if you make your csv file manually, see <b>the misc section</b>)</p>
<p></p>
<p>The example_command.txt is explained below.</p>

<b>Getting an example_command.txt</b>

<p>Submit 1 run that alligns in categories, variables, description and player names with the ones you submit from the csv file to src while having the network tab open in your browser, you need to grab a "PutRunSettings" request and copy it as Curl(POSIX)</p>
<p>Now paste that copy into the example_command.txt file (remove previous example), ensure the -H arguments are 2 spaces of the side.</p>

<b>Now you should be ready to let the program submit the runs</b>
<p>You can put a "//" argument at the end of the command to preview the data you will submit without submitting it, this is to ensure if your levels got parsed correctly.</p>
<b>Example output:</b>

```shell
$ SRCutils sub seterra output.csv example_command.txt

mode is sub - run submitter
game seterra
dsv output.csv
command example_command.txt
==========
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   360    0   360    0     0    360      0 --:--:--  0:00:01 --:--:--   676
100  629k    0  629k    0     0   209k      0 --:--:--  0:00:03 --:--:--  670k


Central Africa: Countries -- kwjnk6z9
{"csrfToken":"0","settings":{"levelId":"kwjnk6z9","categoryId":"9kvggmjk","playerNames":["afdusrt"],"values":[{"variableId":"j84vge28","valueId":"qyzzwo21"}],"gameId":"nd28p43d","platformId":"o7e25xew","date":1762634611,"igt":{"hour":0,"minute":0,"second":0,"millisecond":752},"video":"https://www.youtube.com/watch?v=nuAeFoRm3do&t=0","videoState":0,"comment":"00:00:00:00"},"autoverify":false}
{"runId":"m32rn76y"}

Austria: State Capitals -- kwjnjzn9
{"csrfToken":"0","settings":{"levelId":"kwjnjzn9","categoryId":"9kvggmjk","playerNames":["afdusrt"],"values":[{"variableId":"j84vge28","valueId":"qyzzwo21"}],"gameId":"nd28p43d","platformId":"o7e25xew","date":1762634611,"igt":{"hour":0,"minute":0,"second":1,"millisecond":229},"video":"https://www.youtube.com/watch?v=nuAeFoRm3do&t=8","videoState":0,"comment":"00:00:08:14"},"autoverify":false}
{"runId":"y40r27qz"}
```

## Misc
