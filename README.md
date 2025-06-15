# simulated-annealing

An experiment to learn [Simulated Annealing](https://en.wikipedia.org/wiki/Simulated_annealing) by solving sudoku.

## What is simulated annealing?

Simulated annealing is a probablistic technique for approximating solutions to optimization problems. Simulated annealing has the notion of a "temperature" variable which represents the likelihood that our algorithm takes an objectively worse state to move to (in the hopes that this will allow us to escape local minimas and find global ones). This temperature goes down during the course of the program. 

In trying to find maxima/minima for our optimization problem, we run into the [exploration vs. exploitation dilemma](https://en.wikipedia.org/wiki/Exploration%E2%80%93exploitation_dilemma)

Given an objective function, simulated annealing does the following:
 - start at some initial state, calculate objective
 - in a loop (ends when current objective function is low enough)
    - mutate current state
    - we set the current state to this new state if
        - this new state is better than old state
        - OR if it is worse but we generate a random number which is lower than the temperature threshold


## Is this a good algorithm?

Wellllll, depends on what you mean by good. It is simple and it is useful when your problem has many global minima and also you do not care about which one you find. It is also good for finding approximations of solutions.

Now is it good... maybe my hyperparams are just off but I cannot get it to solve sudoku reliably at all. In fact, I've never gotten it to solve it, even when running for longer and changing the cooling rate. Sudoku might just be the wrong problem for simulated annealing.

<details>
<summary>Exerpt of running this sudoku program</summary>

```
Iteration: 4965000, Temp: 0.63, Current Energy: 31, Best Energy: 31
Iteration: 4966000, Temp: 0.63, Current Energy: 31, Best Energy: 31
Iteration: 4967000, Temp: 0.63, Current Energy: 31, Best Energy: 31
Iteration: 4968000, Temp: 0.63, Current Energy: 31, Best Energy: 31
Iteration: 4969000, Temp: 0.63, Current Energy: 31, Best Energy: 31
Iteration: 4970000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4971000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4972000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4973000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4974000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4975000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4976000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4977000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4978000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4979000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4980000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4981000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4982000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4983000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4984000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4985000, Temp: 0.62, Current Energy: 31, Best Energy: 31
Iteration: 4986000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4987000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4988000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4989000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4990000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4991000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4992000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4993000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4994000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4995000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4996000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4997000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4998000, Temp: 0.61, Current Energy: 31, Best Energy: 31
Iteration: 4999000, Temp: 0.61, Current Energy: 31, Best Energy: 31

Final solution:
3 7 8 | 1 9 2 | 4 7 8 
9 2 4 | 8 6 7 | 9 3 6 
1 6 5 | 3 5 4 | 2 1 5 
------+-------+------
2 4 8 | 6 1 2 | 6 9 5 
7 1 3 | 5 9 8 | 1 4 2 
6 9 5 | 7 4 3 | 7 8 3 
------+-------+------
4 6 2 | 8 5 1 | 9 6 7 
8 9 7 | 4 3 6 | 4 5 3 
3 5 1 | 9 7 2 | 8 2 1 
Final energy: 31

WARNING: Solution not perfect. Energy: 31
```

</details>

## The code

This was mostly a learning exercise and was written mostly with the following LLMs:
 - Gemini 2.5 Flash (mostly this one)
 - Claude Sonnet 4
 - Deepseek R1

My current setup is using Roo Code in VS Code, and using Openrouter. The total cost of generating this code was $0.611. 60 cents is not bad for a quick learning exercise and I also did implement a simulated annealing program a year ago in python, so I was familiar with the domain. My prior program also failed to solve sudoku correctly so I either do not know how to get this to work, or sudoku is a bad problem for simulated annealing in general. 

If you know more and would like to let me know, please reach out! -- pranoy@utexas.edu