# Task Report

Script to print a daily work report, using daily work notes.

## Usage
```bash
task-report data/tasks_202303/
```
To print with a JSON structure use:
```bash
task-report data/tasks_202303/ -j
```

## Datasource
Every day I log in notes the ticket I'm working on and the previous ticket to report on the team's daily.
At the moment, the note is created manually on a daily basis.
The structure of the note is:
```
Daily:
	* MBM-1234 Yesterday task [x]
Today:
	* MBM-1235 today task []
```
`20230323_notes.md`

## Report:
![image](https://user-images.githubusercontent.com/6902179/227689217-e2eee2a4-f90b-4910-bbb6-9cebb0b6061e.png)
