#include <stdio.h>
// i need stdlib to make this compiler warning go away?:
// implicitly declaring library function 'malloc' with type 'void *(unsigned long)'
#include <stdlib.h>
#include <string.h>

#define MIDPOINT 1010
#define TARGET 2020

char* load_input(FILE *fp) {
	fseek(fp, 0, SEEK_END);
	size_t length = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char *buffer = malloc(length + 1);
	fread(buffer, sizeof(char), length, fp);
	return buffer;
}


int number_from_chars(char* chars, int offset) {
	char cur = chars[offset];
	int consecutive_numbers = 0;
	while (cur >= '0' && cur <= '9') {
		consecutive_numbers++;
		cur = chars[offset + consecutive_numbers];
	}

	char *digits = malloc(consecutive_numbers);
	strncpy(digits, chars + offset, consecutive_numbers);
	return atoi(digits);
}

int main()
{
	FILE *fp;
	fp = fopen("input", "r");
	if (fp == NULL) {
		puts("could not open file");
		return 1;
	}
	char* input = load_input(fp);
	if (input == NULL) {
		puts("could not malloc");
		return 1;
	}
	fclose(fp);

	// kind of a cheat here, don't really know how to start iterating through this char*
	char cur = '\n';
	int min = 0, max = 0, matching_char_count = 0, valid_count = 0;
	char rule_char = '\0';

	for (int i = 0; cur != '\0'; i++) {
		cur = input[i];
		if (cur >= '0' && cur <= '9') {
			if (min == 0) {
				min = number_from_chars(input, i);
			} else if (max == 0) {
				max = number_from_chars(input, i);
			}
		} else if (cur >= 'a' && cur <= 'z') {
			if (rule_char == '\0') {
				rule_char = cur;
			} else if (cur == rule_char) {
				matching_char_count++;
			}
		} else if (cur == '\n') {
			if (matching_char_count >= min && matching_char_count <= max) {
				valid_count++;
			}
			rule_char = '\0';
			matching_char_count = 0;
			min = 0;
			max = 0;
		}
	}
	printf("we got %d valid passwords", valid_count);
	return 0;
}
