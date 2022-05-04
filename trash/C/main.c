#include<stdio.h>
#include<stdlib.h>

int main(int argc, char *argv[]) {
    
    char cmd[50];
    sprintf(cmd,"mv %s ~/.trash/",argv[1]);
    system(cmd); 
    return 0;
}



    
