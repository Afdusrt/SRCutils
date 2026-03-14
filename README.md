# SRCutils

Utilities for interacting with the speedrun.com platform via curl requests.

## Features:
- **Prepare sheet:** Prepares a spreadsheet, for *submit sheet* feature.
- **Submit sheet:** Submits speedruns with data from a spreadsheet.
> Legacy feature: EDL mode, prepares a spreadsheet from an edl file. Currently is **NOT** compatible with *submit sheet* feature. 
#### Note:
The program relies on:
```rust
std::process::Command("curl").arg(...
```
to send requests to speedrun.com, so make sure curl is on your path.

## Usage
For this example, we will prepare and submit a spreadsheet of runs for speedrun.com/seterra.
### 1. Prepare sheet:
1. Running the command without arguments show you a help screen
```shell
HELP:
====
arg 1 - game abbreviation
arg 2 - csv file to save
```
2. Supply arguments
```shell
$ prepare-sheet.exe seterra spreadsheet.csv
```
Note: Throughout the programs runtime, you will see sections like this, these are just outputs from Curl being ran, we can ignore these. (they still get printed to show that the program didnt halt or something)
```shell
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   376    0   376    0     0    187      0 --:--:--  0:00:02 --:--:--   462
100 18239    0 18239    0     0   4783      0 --:--:--  0:00:03 --:--:-- 23087
```
3. The first prompt you recieve, is to choose a category.
```shell
Pick category?:
========
0/ jdz0yzv2/ Europe: Countries (with Kosovo)/ false
1/ 02qw5zp2/ The U.S.: 50 States/ false
...
9/ 5dwvozlk/ Pin/ true
...

```
The true or false statement at the end, indicates whether the category is for ILs. We will enter the index of a category to choose it. For this example we will pick 9, for the IL category Pin.
After picking a category, the categorie's variables are printed, we can ignore this if we so choose to.

4. You will now be prompted to choose the platform of your run. For games that do not have platforms, pick 0, NO.
```shell
Now, select a platform for the game: 
0/ NO
1/ o7e25xew/ Web
```
same goes for the next prompt, about regions.
NOTE: some games require regions to be set.

5. Now, the spreadsheet will be written to the file. You will see it get printed on the screen. This is for debug reasons, as if this fails, it means something went wrong before submitting.
### 2. Modify sheet:
NOTE: For this example, libreoffice calc is used.

1. Open sheet, uncheck to use anything other than '|' as a delimiter, use no string delimiter. This is important, as otherwise it would change formatting.

2. You now see the format. Do not modify A1 -> A4. Each row below the headers (date, region...) is a seperate run. Fill in all fields. Use a full video link. Do not include '"' in your comment. To choose variables, copy cells from the reference. (in this example,  the first run will set the platform variable to website, and the second run will set it to touch web). NOTE: For APIv1, you have to submit with archived variables set. In the screenshot, this spreadsheet will fail to submit, because not every row is full. you can delete rows of levels you didnt do.
3. Some variables are per level only, these are represented at the end of the row, next to the affected level. manually unwrap the value you want. (run 3, (highlighted in green), will be invalid, because we didnt unwrap our chosen value)
4. we are now ready for **submit-sheet** feature.
### 3. Submit sheet:
```shell
HELP:
====
arg 1 - api key, from speedrun.com -> settings -> api key
arg 2 - csv file to submit
```
```shell
submit-sheet.exe keykeykeykeykeykey spreadsheet.csv
```
Now we compare the output of the command line, where each json payload represents the run to our spreadsheet.
> **BIG NOTE:** If a game has level-specific variables, the output, will be very crowded, because of api v1 quirk, where you need to declare each level-specific variable on each level, this might get fixed some day.
If it matches up. we type "yes", and our runs will be submitted, with a limit of 1 request per 2 seconds.

**ALWAYS CHECK PENDING AFTERWARDS**
