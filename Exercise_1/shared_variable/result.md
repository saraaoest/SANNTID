
3: Sharing a variable
Both the c and the go program are struggeling with race conditions. The one thread is interupting the other thread in the middle of an execution. Therfor the answere never ends up to be 0. The answer in go, ends up beeing around 1000000 or -999999 every time, so it is more consisten of the one thread allways interupting the other one. But the answere in the c program is more random. 

4: Sharing a variable, but properly
Worked with mutex, don't know why to use semaphores.