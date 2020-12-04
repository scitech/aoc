#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PASSPORT_FIELDS 8
#define FIELD_NAME_LENGTH 3

char* load_input(FILE *fp) {
	fseek(fp, 0, SEEK_END);
	size_t length = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char *buffer = malloc(length + 1);
	fread(buffer, sizeof(char), length, fp);
	return buffer;
}

int find_passport_record_length(char* full_input, int start_pos) {
	char cur = full_input[start_pos];
	int read_char_count = 0;
	while (cur != '\0') {
		if (cur == '\n') {
			char peek = full_input[start_pos + read_char_count];
			if (peek == '\n') {
				read_char_count++;
				break;
			}
		}
		cur = full_input[start_pos + read_char_count];
		read_char_count++;
	}
	return read_char_count;
}

struct Field {
	char* name;
	char* value;
};

int read_passport(struct Field* passport, char* input, int* cursor, int passport_length) {
	int passport_start = *cursor;
	int field_start = *cursor;
	int fields_read = 0;
	int passport_cursor = field_start;
	char current_ch;
	while (passport_cursor - passport_start < passport_length) {
		current_ch = input[passport_cursor];
		if (current_ch == ' ' || current_ch == '\n') {
			struct Field f;
			int field_length = passport_cursor - field_start;
			if (field_length <= 0) break;
			// something weird happens if I try to make this a char field_name[3]
			char* field_name = malloc(FIELD_NAME_LENGTH);
			char* field_value = malloc(field_length - FIELD_NAME_LENGTH - 1);
			strncpy(field_name, input + field_start, FIELD_NAME_LENGTH);
			strncpy(field_value, input + field_start + FIELD_NAME_LENGTH + 1, field_length - FIELD_NAME_LENGTH - 1);
			f.name = field_name;
			f.value = field_value;
			printf("%s: %s\n", field_name, field_value);
			passport[fields_read] = f;

			fields_read++;
			passport_cursor++;
			field_start = passport_cursor;
		} else {
			passport_cursor++;
		}
	}
	*cursor += passport_length;
	return fields_read;
}

int check_passport(struct Field* passport, int fields_read) {
	if (fields_read > 8) {
		return 0;
	}

	if (fields_read == 8) {
		return 1;
	}

	if (fields_read == 7) {
		int missing_cid = 1;
		for (int i = 0; i < fields_read; i++) {
			struct Field f = passport[i];
			if (strcmp(f.name, "cid") == 0) {
				missing_cid = 0;
			}
		}
		return missing_cid;
	}

	return 0;
}

void part_one(char *input) {
	int cursor = 0, valid_passport_count = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		int passport_length = find_passport_record_length(input, cursor);
		struct Field passport[PASSPORT_FIELDS];
		int fields_read = read_passport(passport, input, &cursor, passport_length);
		int valid = check_passport(passport, fields_read);
		valid_passport_count += valid;
		cur = input[cursor];
	}
	printf("%d valid passports\n", valid_passport_count);
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

	return 0;
}