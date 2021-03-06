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

char* digits_from_chars(char* chars, int offset) {
	char cur = chars[offset];
	int consecutive_numbers = 0;
	while (cur >= '0' && cur <= '9') {
		consecutive_numbers++;
		cur = chars[offset + consecutive_numbers];
	}

	char *digits = malloc(consecutive_numbers);
	strncpy(digits, chars + offset, consecutive_numbers);
	return digits;
}

char* password_from_chars(char* chars, int offset) {
	char cur = chars[offset];
	int consecutive_letters = 0;
	while (cur >= 'a' && cur <= 'z') {
		consecutive_letters++;
		cur = chars[offset + consecutive_letters];
	}

	char *letters = malloc(consecutive_letters);
	strncpy(letters, chars + offset, consecutive_letters);
	return letters;
}

int check_password(char *password, int min, int max, char rule_char) {
	int matches = 0;
	if (password[min - 1] == rule_char || password[max - 1] == rule_char) {
		if (!(password[min - 1] == rule_char && password[max - 1] == rule_char)) {
			matches = 1;
		}
	}
	return matches;
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

	int offset = 0;
	while (cur != '\0') {
		cur = input[offset];
		if (cur >= '0' && cur <= '9') {
			if (min == 0) {
				char *min_digits = digits_from_chars(input, offset);
				min = atoi(min_digits);
				offset += strlen(min_digits);
				continue;
			} else if (max == 0) {
				char *max_digits = digits_from_chars(input, offset);
				max = atoi(max_digits);
				offset += strlen(max_digits);
				continue;
			}
		} else if (cur >= 'a' && cur <= 'z') {
			if (rule_char == '\0') {
				rule_char = cur;
			} else {
				char *password = password_from_chars(input, offset);
				int matches = check_password(password, min, max, rule_char);
				if (matches == 1) {
					valid_count++;
				}
				offset += strlen(password) - 1;
			}
		} else if (cur == '\n') {
			rule_char = '\0';
			matching_char_count = 0;
			min = 0;
			max = 0;
		}
		offset++;
	}
	printf("we got %d valid passwords\n", valid_count);
	return 0;
}