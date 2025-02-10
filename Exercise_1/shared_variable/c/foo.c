// Compile with `gcc foo.c -Wall -std=gnu99 -lpthread`, or use the makefile
// The executable will be named `foo` if you use the makefile, or `a.out` if you use gcc directly
//to run: ./a.exe

//or: gcc foo.c -o foo.exe
//then ./foo.exe

/*
global shared int i = 0

main:
    spawn thread_1
    spawn thread_2
    join all threads (or wait for them to finish)
    print i

thread_1:
    do 1_000_000 times:
        i++
thread_2:
    do 1_000_000 times:
        i--
*/

#include <pthread.h>
#include <stdio.h>

int i = 0;
pthread_mutex_t count_mutex;

// Note the return type: void*
void* incrementingThreadFunction(){
    // TODO: increment i 1_000_000 times
    for(int j= 0; j < 1000000; j++){
        pthread_mutex_lock(&count_mutex);
        i++;
        pthread_mutex_unlock(&count_mutex);
    }
    return NULL;
}

void* decrementingThreadFunction(){
    // TODO: decrement i 1_000_000 times
    for(int k= 0; k < 100000; k++){
        pthread_mutex_lock(&count_mutex);
        i--;
        pthread_mutex_unlock(&count_mutex);
    }
    return NULL;
}


int main(){
    // TODO: 
    // start the two functions as their own threads using `pthread_create`
    // Hint: search the web! Maybe try "pthread_create example"?
    
/*
    int pthread_create(pthread_t* thread, 
                   const pthread_attr_t* attr, 
                   void* (*start_routine)(void*), 
                   void* arg);
*/
    pthread_t thread;
    pthread_t thread2;

    pthread_create(&thread, NULL, incrementingThreadFunction, NULL);
    pthread_create(&thread2, NULL, decrementingThreadFunction, NULL);
    
    // TODO:
    // wait for the two threads to be done before printing the final result
    // Hint: Use `pthread_join`  
    pthread_join(thread, NULL);
    pthread_join(thread, NULL);
    
    
    printf("The magic number is: %d\n", i);
    return 0;
}

//include os/os.h
//works with os_spawn(&func, minne), os_sleap, 