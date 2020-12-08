#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* load_input(FILE *fp) {
	fseek(fp, 0, SEEK_END);
	size_t length = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char *buffer = malloc(length + 1);
	fread(buffer, sizeof(char), length, fp);
	return buffer;
}

void advance(int* cursor) {
	*cursor += 1;
}

int find_group_record_length(char* full_input, int start_pos) {
	char cur = full_input[start_pos];
	int read_char_count = 0;
	while (cur != '\0') {
		if (cur == '\n') {
			char peek = full_input[start_pos + read_char_count + 1];
			if (peek == '\n') {
				read_char_count++;
				break;
			}
		}
		read_char_count++;
		cur = full_input[start_pos + read_char_count];
	}
	return read_char_count;
}

int add_char_to_set(char* set, char ch, int length, int size) {
	if (ch == '\n') {
		return 0;
	}

	int added = 1;
	for (int i = 0; i < length; i++) {
		if (set[i] == ch) {
			added = 0;
		}
	}
	if (added == 1) {
		set[size] = ch;
	}
	return added;
}

void add_char_to_list(char* set, char ch, int size) {
	if (ch == '\n') {
		return;
	}

	set[size] = ch;
}

int read_group(char* input, int* cursor) {
	char cur = input[*cursor];
	int met_delimiter = 0;
	int answers_added_to_set = 0;
	int length = find_group_record_length(input, *cursor);
	char* group_answers = malloc(length + 1);
	group_answers[length + 1] = 0;
	while (cur != '\0' && met_delimiter == 0) {
		switch (cur) {
			case '\n':
			{
				char peek = input[*cursor + 1];
				if (peek == '\n') {
					met_delimiter = 1;
					advance(cursor);
				};
				advance(cursor);
				break;
			}
			default:
			{
				answers_added_to_set += add_char_to_set(group_answers, cur, length, answers_added_to_set);
				advance(cursor);
				break;
			}
		}
		cur = input[*cursor];
	}
	return answers_added_to_set;
}

int real_answers_for_group(char* answers, int people_count, int answers_count) {
	int acceptable_answers = 0;
	for (int i = 0; i < answers_count; i++) {
		char ch = answers[i];
		int count = 0;
		for (int j = 0; j < answers_count; j++) {
			if (ch != 0 && ch == answers[j]) {
				count++;
			}
		}
		if (count == people_count) {
			acceptable_answers++;
			for (int j = 0; j < answers_count; j++) {
				if (ch == answers[j]) {
					answers[j] = 0;
				}
			}
		}
	}
	return acceptable_answers;
}

int read_group_2(char* input, int* cursor) {
	char cur = input[*cursor];
	int answers_for_group_count = 0, people_in_group = 0, total_answers = 0;
	int length = find_group_record_length(input, *cursor);
	char* group_answers = malloc(length);
	group_answers[length] = 0;
	while (cur != '\0') {
		switch (cur) {
			case '\n':
			{
				people_in_group++;
				char peek = input[*cursor + 1];
				if (peek == '\n' || peek == '\0') {
					total_answers += real_answers_for_group(group_answers, people_in_group, answers_for_group_count);
					people_in_group = 0;
					answers_for_group_count = 0;
					advance(cursor);
				}
				advance(cursor);
				break;
			}
			default:
			{
				add_char_to_list(group_answers, cur, answers_for_group_count);
				answers_for_group_count++;
				advance(cursor);
				break;
			}
		}
		cur = input[*cursor];
	}
	return total_answers;
}

void part_one(char *input) {
	int cursor = 0, all_answers = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		all_answers += read_group(input, &cursor);
		cur = input[cursor];
	}
	printf("part1: %d\n", all_answers);
}
void part_two(char *input) {
	int cursor = 0, all_answers = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		all_answers += read_group_2(input, &cursor);
		cur = input[cursor];
	}
	printf("part2: %d\n", all_answers);
}

int main() {
	FILE *fp;
	fp = fopen("input", "r");
	if (fp == NULL) {
		puts("could not open file");
		return 1;
	}
	char *input = load_input(fp);
	if (input == NULL) {
		puts("could not malloc");
		return 1;
	}
	fclose(fp);

	part_one(input);
	part_two(input);

	return 0;
}

// int main() {
//   // test example from the prompt
//   char* fakebuf = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n\0";
// 	// char* fakebuf = "a\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n\0";
//   int cursor = 0;
//   part_one(fakebuf);
//   // assert(result == 11);
// }
