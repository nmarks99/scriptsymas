#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>

void getFileCreationTime(char *filePath) {
    struct stat attrib;
    stat(filePath, &attrib);
    char date[10];
    strftime(date, 10, "%d-%m-%y", gmtime(&(attrib.st_ctime)));
    printf("The file %s was last modified at %s\n", filePath, date);
    date[0] = 0;
}

int main(int argc, char *argv[]) {

    get_file_modification_time("../rmspace.py");


    // char cmd[50];
    // sprintf(cmd,"mv %s ~/.trash/",argv[1]);
    // system(cmd); 
    return 0;
}



    
