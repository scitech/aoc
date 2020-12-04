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

enum field_name {
	birth_year = 1,
	issue_year,
	expiration_year,
	height,
	hair_color,
	eye_color,
	passport_id,
	country_id,
};

enum field_name field_name_from_string(char* name) {
	if (strcmp(name, "byr") == 0) {
		return birth_year;
	} else if (strcmp(name, "iyr") == 0) {
		return issue_year;
	} else if (strcmp(name, "eyr") == 0) {
		return expiration_year;
	} else if (strcmp(name, "hgt") == 0) {
		return height;
	} else if (strcmp(name, "hcl") == 0) {
		return hair_color;
	} else if (strcmp(name, "ecl") == 0) {
		return eye_color;
	} else if (strcmp(name, "pid") == 0) {
		return passport_id;
	} else if (strcmp(name, "cid") == 0) {
		return country_id;
	}

	return 0;
}

struct Field {
	enum field_name name;
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
			char* field_name_str = malloc(FIELD_NAME_LENGTH);
			char* field_value = malloc(field_length - FIELD_NAME_LENGTH - 1);
			strncpy(field_name_str, input + field_start, FIELD_NAME_LENGTH);
			enum field_name field_name_type = field_name_from_string(field_name_str);
			if (field_name_type != 0) {
				strncpy(field_value, input + field_start + FIELD_NAME_LENGTH + 1, field_length - FIELD_NAME_LENGTH - 1);
				f.name = field_name_type;
				f.value = field_value;
				passport[fields_read] = f;
			}

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


/*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
        If cm, the number must be at least 150 and at most 193.
        If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
*/
int year_in_range(char* year, char* min, char* max) {
	int year_val = atoi(year);
	int min_val = atoi(min);
	int max_val = atoi(max);
	return year_val >= min_val && year_val <= max_val;
}
int check_field(struct Field f) {
	int valid = 0;
	switch (f.name) {
		case birth_year:
		{
			valid = year_in_range(f.value, "1920", "2002");
			break;
		}
		case issue_year:
		{
			valid = year_in_range(f.value, "2010", "2020");
			break;
		}
		case expiration_year:
		{
			valid = year_in_range(f.value, "2020", "2030");
			break;
		}
		case height:
		{
			int amount = 0;
			char unit[3] = { '\0' };
			int matches = sscanf(f.value, "%d%2c", &amount, unit);
			if (matches == 2) {
				if (strcmp(unit, "cm") == 0) {
					valid = amount >= 150 && amount <= 193;
				} else if (strcmp(unit, "in") == 0) {
					valid = amount >= 59 && amount <= 76;
				}
			}
			break;
		}
		case hair_color:
		{
			if (f.value[0] == '#') {
				char unused_value[6];
				int matches = sscanf(f.value + 1, "%6[a-f0-9]", unused_value);
				if (matches) {
					valid = 1;
				}
			}
			break;
		}
		case eye_color:
		{
			char *valid_colors[7] = {
				"amb", "blu", "brn", "gry", "grn", "hzl", "oth",
			};
			for (int i = 0; i < 7; i++) {
				if (strcmp(valid_colors[i], f.value) == 0) {
					valid = 1;
				}
			}
			break;
		}
		case passport_id:
		{
			if (strlen(f.value) == 9) {
				char unused_id[9];
				valid = sscanf(f.value, "%9[0-9]", unused_id);
			}
			break;
		}
		case country_id:
		{
			valid = 1;
			break;
		}
		default:
			break;
	}
	return valid;
}

int check_passport(struct Field* passport, int fields_read) {
	int required_fields_present = 0;

	if (fields_read == 8) {
		required_fields_present = 1;
	}

	if (fields_read == 7) {
		int missing_cid_only = 1;
		for (int i = 0; i < fields_read; i++) {
			struct Field f = passport[i];
			if (f.name == country_id) {
				missing_cid_only = 0;
			}
		}
		if (missing_cid_only) {
			required_fields_present = 1;
		};
	}
	if (required_fields_present == 0) return 0;

	int all_fields_valid = 1;

	for (int i = 0; i < fields_read; i++) {
		struct Field f = passport[i];
		all_fields_valid = all_fields_valid * check_field(f);
	}
	return all_fields_valid;
}

void part_two(char *input) {
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

	part_two(input);

	return 0;
}