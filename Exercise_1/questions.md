Exercise 1 - Theory questions
-----------------------------

### Concepts

What is the difference between *concurrency* and *parallelism*?
> Cuncurrency: A single CPU is handeling muliple programs at the smae time, so it goes back and fourth forme one to the other and exexutes a little part from each at the time, befor continuing on the other and so on. 
Paralellism: dividing a problem into smaller tasks that can be executed form different CPUs or computers. 
See image.png

What is the difference between a *race condition* and a *data race*? 
> Race conditions: timing-> when shared veriables are used or editted from multiple threds, it is important that the read and writing prosesses are done in right order. 
(when to threds receve the same messages at different time.)
Data race: one thred accesses a object when another thred reads to it at the same time: undefined behaveour
 
*Very* roughly - what does a *scheduler* do, and how does it do it?
> A scheduler is organizing the workflow of different processors, keeping them buissy to be efficient when executing tasks from one or multiple threads. Is neaded when executing concurrency.  


### Engineering

Why would we use multiple threads? What kinds of problems do threads solve?
> A thred is a mark of where in the program you are.
The same core can for example have two threads, and than swhitching between executing from the different threds(concurrency). Provide zero-cycle context switching between threads. The core has then loaded the different set of variables it neead for each thread(part of the program).
To encode different parts of a probem simultaniously. Making the execution go faster. 
Used when execution time is an limited resource, espetially in embeddes systems and aplication/web development. A way of making the system multitask from

Some languages support "fibers" (sometimes called "green threads") or "coroutines"? What are they, and why would we rather use them over threads?
> User levle based scheduler. 
Both are seperated execution task for a program. 
in thread an execution can be interupted at any time, for example in the middle of an long calculation with lots of data, leaving the data in an inclompleat state. 
With fibers the execution path is only interupted by
Threads therefor doesnt have that much race conditions. 
Faster than threads?

Does creating concurrent programs make the programmer's life easier? Harder? Maybe both?
> Creating concurrent programs is harder than making a program ececuted linearly, because you have to spessify what order the different parts are going to be executed in. 
Councurrent programs it is also harder to make than paralell programs

What do you think is best - *shared variables* or *message passing*?
> Wat is the best, depends on wich program you are making. For small programs, shared variables is fast and safe, and maby causes less race conditions. Then you have a spesific place in the memmory with the variables two threads are using. But if you have too many threads you will not have space for all the shared variables, and it will not be suficcent. Then is is necessary to use message passing. 
Also with too much delay between the different execution cores, message passing will be the best option.  


