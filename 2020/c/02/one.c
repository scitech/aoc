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

char* read_digits(char* chars, int *cursor) {
	int start_pos = *cursor;
	char cur = chars[start_pos];
	int consecutive_numbers = 0;
	while (cur >= '0' && cur <= '9') {
		consecutive_numbers++;
		cur = chars[start_pos + consecutive_numbers];
	}

	char *digits = malloc(consecutive_numbers);
	strncpy(digits, chars + start_pos, consecutive_numbers);
	*cursor = start_pos + consecutive_numbers;
	return digits;
}

char* read_until(char* chars, int *cursor, char delimiter) {
	int start_pos = *cursor;
	char cur = chars[start_pos];
	int read_char_count = 0;
	while (cur != delimiter) {
		read_char_count++;
		cur = chars[start_pos + read_char_count];
	}

	char *read_chars = malloc(read_char_count);
	strncpy(read_chars, chars + start_pos, read_char_count);
	*cursor = start_pos + read_char_count;
	return read_chars;
}

int check_password(char* password, int min, int max, char rule_char) {
	int is_valid = 0, match_count = 0, index = 0;
	char cur = password[index];
	while (cur != '\0') {
		if (cur == rule_char) {
			match_count++;
		}
		index++;
		cur = password[index];
	}
	if (match_count >= min && match_count <= max) {
		is_valid = 1;
	}

	return is_valid;
}

int main() {
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

	int min = 0, max = 0, matching_char_count = 0, valid_count = 0;
	char rule_char = '\0';
	char *password = NULL;

	int cursor = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		printf("hi %c\n", cur);
		if (cur >= '0' && cur <= '9') {
			if (min == 0) {
				char *min_digits = read_digits(input, &cursor);
				min = atoi(min_digits);
				cur = input[cursor];
				continue;
			} else if (max == 0) {
				char *max_digits = read_digits(input, &cursor);
				max = atoi(max_digits);
				cur = input[cursor];
				continue;
			}
		} else if (cur >= 'a' && cur <= 'z') {
			if (rule_char == '\0') {
				rule_char = cur;
			} else if (password == NULL) {
				password = read_until(input, &cursor, '\n');
				cur = input[cursor];
				continue;
			}
		} else if (cur == '\n') {
			if (check_password(password, min, max, rule_char)) {
				valid_count++;
			}
			rule_char = '\0';
			matching_char_count = 0;
			min = 0;
			max = 0;
			password = NULL;
		}
		cursor++;
		cur = input[cursor];
	}
	printf("we got %d valid passwords\n", valid_count);
	return 0;
}