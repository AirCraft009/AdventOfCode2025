#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 *  Plan:
 *
 *  Find out if the paper roll is surrounded by more or less than 4 rolls of paper
 *  simple:
 *      load file into mem, go from pos to pos and add up the values of the chars next to it
 *      then just check if it's below a threshold.
 *      . has ASCII 46 and @ has 64
 *      so if there are only dot's it would be 46*8 = 368
 *      if it's all @ it 64 * 8 = 512
 *      so 368 can be the base val and 512 the max
 *      46 * 4 + 64 * 4 = 440.
 *      If the value is lower than this it should be marked as movable
 *      else it should be immovable
 *
 */


// Source - https://stackoverflow.com/a
// Posted by Superlokkus, modified by community. See post 'Timeline' for change history
// Retrieved 2025-12-06, License - CC BY-SA 4.0

char* replace_char(char* str, char find, char replace){
    char *current_pos = strchr(str,find);
    while (current_pos) {
        *current_pos = replace;
        current_pos = strchr(current_pos + 1,find);
    }
    return str;
}



void part1(void) {
    FILE *file = fopen("input.txt", "r");
    if (file == NULL) {
        printf("Error opening file\n");
    }
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    rewind(file);

    char *str = malloc(size + 1);
    fread(str, 1, size, file);
    fclose(file);
    str[size] = '\0';

    //this makes the currently valid string into an invalid one
    // breaking it up into multiple peaces
    // still good because this char will now not interfere with the sum
    replace_char(str, '\n', '\0');
    //we can now get the lenght of a line by calling strlen() aswell
    int linelen = strlen(str) + 1;
    int accessible = 0;
    //bool modified = false;
    for (int i = 0; i < size; i++) {
        if (str[i] != '@') {
            continue;
        }

        int col = i % linelen;

        char prevC='.', nextC='.';
        if (i > 0) prevC = str[i - 1];
        if (i + 1 < size) nextC = str[i + 1];

        if (col == 0) prevC = '.';
        if (col == linelen - 1) nextC = '.';

        char upperC='.', upperLC='.', upperRC='.';
        if (i >= linelen) {
            upperC = str[i - linelen];
            if (col > 0) upperLC = str[i - linelen - 1];
            if (col < linelen - 1) upperRC = str[i - linelen + 1];
        }

        char lowerC='.', lowerLC='.', lowerRC='.';
        if (i + linelen < size) {
            lowerC = str[i + linelen];
            if (col > 0) lowerLC = str[i + linelen - 1];
            if (col < linelen - 1) lowerRC = str[i + linelen + 1];
        }


        int rollCount = 0;
        if (upperC == '@') rollCount++;
        if (lowerC == '@') rollCount++;
        if (upperLC == '@') rollCount++;
        if (upperRC == '@') rollCount++;
        if (lowerLC == '@') rollCount++;
        if (lowerRC == '@') rollCount++;
        if (prevC  == '@') rollCount++;
        if (nextC  == '@') rollCount++;


        if (rollCount < 4) {
            accessible ++;
            //modified = true;
        }
    }
    printf("%d", accessible);
    free(str);
}
