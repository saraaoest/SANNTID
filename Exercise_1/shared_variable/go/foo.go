// Use `go run foo.go` to run your program

//or use: go mod init foo: to make a mod file
// then: go run .

package main

import (
	"fmt"
	"runtime"
)

func server(instruction chan string, done chan bool) {
	var i int = 0
	for {
		select {
		case msg := <-instruction:
			switch msg {
			case "increment":
				i++
			case "decrement":
				i--
			case "done":
				fmt.Println("The magic number is", i)
				done <- true
				return
			}
		}
	}
}

func incrementing(instruction chan string, done chan bool) {
	// Increment i 1,000,000 times
	for j := 0; j < 1000000; j++ {
		instruction <- "increment"
	}
	done <- true
}

func decrementing(instruction chan string, done chan bool) {
	// Decrement i 1,000,000 times
	for j := 0; j < 100000; j++ {
		instruction <- "decrement"
	}
	done <- true
}

func main() {
	runtime.GOMAXPROCS(2) // Use 2 CPU cores

	instruction := make(chan string)
	done := make(chan bool)

	go server(instruction, done)
	go incrementing(instruction, done)
	go decrementing(instruction, done)

	// Wait for both functions to finish
	<-done
	<-done

	instruction <- "done"
	<-done
}

/*func incrementing(ch chan int) {
    //TODO: increment i 1000000 times
    sum: = 0
    for j := 0; j < 1000000; j++ {
		sum++
	}
    ch <- sum
}

func decrementing(ch chan int) {
    //TODO: decrement i 1000000 times
    sum: = 0
    for j := 0; j < 1000000; j++ {
		sum--
	}
    ch <- sum
}

func main() {
    // What does GOMAXPROCS do? What happens if you set it to 1?
    runtime.GOMAXPROCS(2) // Use 2 CPU cores
    //The GOMAXPROCS variable limits the number of operating system threads that can execute user-level
	
    ch := make(chan int)

    // TODO: Spawn both functions as goroutines
    go incrementing(i)
    go decrementing(i)

    x, y := <-ch, <-ch // receive from c

	fmt.Println(x, y, x+y)
    
    // We have no direct way to wait for the completion of a goroutine (without additional synchronization of some sort)
    // We will do it properly with channels soon. For now: Sleep.
    time.Sleep(500*time.Millisecond)
    Println("The magic number is:", i)
}*/