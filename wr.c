#include <stdlib.h>
#include <stdio.h>
#include <string.h>
// TODO: Add more command line options

int main(int argc, char *argv[]) {

	if (argc == 2){  
		if ( strlen(argv[1]) == 1){
			// Print a single line weather report if arg is "1"
			if (strcmp(argv[1],"1") == 0){
				system("curl wttr.in/?format=4");
			}
		}
		else{// Return if arg is not "1"
			return 0;
		}
	}
	else{
		system("curl wttr.in");
	}

	return 0;
}
