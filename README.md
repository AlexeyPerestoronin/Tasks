# Introduction
**The Repository for interviews performing!**

# Table of content
1. [The Dedication](#the-dedication)
2. [How to use?](#how-to-use)  
2.1. [How to build?](#how-to-build)  
2.2. [How to add new tasks?](#how-to-add-new-tasks)  
2.2.1. [Example-1: your own task](#example-1-your-own-task)  
2.2.2. [Example-2: a company task](#example-2-a-company-task)  
2.2.2. [Example-3: an external task](#example-3-an-external-task)  
2.2. [How to interact with a candidate via IDE?](#how-to-interact-with-a-candidate-via-ide)
  
# [The Dedication](#table-of-content)
This repo is intended to perform an interview with a candidate to test his/her skills in C++.

The main features/advantages of using this:  
1. Coding via Visual Studio IDE with its hints.
2. Possibility of checking a candidate's solution through the building of it and via G-Tests.
3. Convenient way of collecting all internal and external tests being used during an interview.

# [How to use?](#table-of-content)
## [How to build?](#how-to-use)
1. Clone this repository.
2. Move into and run `./build.sh` via bash.

## [How to add new tasks?](#how-to-use)
Observe some of the already created tasks through the Visual Studio solution:  
![](./solution-explorer.png?raw=true "solution explorer")

### [Example-1: your own task](#how-to-add-new-tasks)
If:
- you want to add your own task and,
- and your task name is `My Super Task`, -

you have to create two files:
1. `./tests/suit-tests/Your Name/<SomeFilter-1/<SomeFilter-2/>>my-supper-task.cpp`
2. `./tests/suit-tests/Your Name/<SomeFilter-1/<SomeFilter-2/>>my-supper-task.md`

**And run `./build.sh` once again (the VS-solution will be adopted)!**

### [Example-2: a company task](#how-to-add-new-tasks)
If:
- your want to add a task from a company,
- and the name of this company is `Very-Important Company`,
- and the name of the task is `Super Task`, -

you have to create two files:  
1. `./tests/suit-tests/VeryImportantCompany/<SomeFilter-1/<SomeFilter-2/>>SuperTask.cpp`
2. `./tests/suit-tests/VeryImportantCompany/<SomeFilter-1/<SomeFilter-2/>>SuperTask.md`

**And run `./build.sh` once again (the VS-solution will be adopted)!**

### [Example-3: an external task](#how-to-add-new-tasks)
If:
- you want to add an external task (lets from the [LeetCode](https://leetcode.com)),
- and this task has `medium` difficulty,
- and its name is [maximum-length-of-a-concatenated-string-with-unique-characters](https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/), -

you have to create two files:
1. `./tests/suit-tests/LetCode/Medium/VeryImportantTeam/maximum-length-of-a-concatenated-string-with-unique-characters.cpp`
2. `./tests/suit-tests/LetCode/Medium/VeryImportantTeam/maximum-length-of-a-concatenated-string-with-unique-characters.md`

**And run `./build.sh` once again (the VS-solution will be adopted)!**

## [How to interact with a candidate via IDE?](#how-to-use)

The best way to interact with a candidate is usage of Life Share for:
1. [VSCode](https://code.visualstudio.com/learn/collaboration/live-share) or
2. [Visual Studio](https://visualstudio.microsoft.com/services/live-share/)
