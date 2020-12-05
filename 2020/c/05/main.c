#include <assert.h>
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

int read_boarding_pass(char* input, int* cursor) {
	int row_min = 0, row_max = 127, col_min = 0, col_max = 7;
	int row = 0, col = 0;
	char cur = input[*cursor];
	while (cur == 'F' || cur == 'B' || cur == 'L' || cur == 'R') {
		switch (cur) {
			case 'F':
			{
				int row_diff = row_max  - row_min;
				if (row_diff == 1) {
					row = row_min;
				} else {
					row_max = row_max - (row_diff / 2) - 1;
				}
				break;
			}
			case 'B':
			{
				int row_diff = row_max - row_min;
				if (row_diff == 1) {
					row = row_max;
				} else {
					row_min = row_min + (row_diff / 2) + 1;
				}
				break;
			}
			case 'L':
			{
				int col_diff = col_max - col_min;
				if (col_diff == 1) {
					col = col_min;
				} else {
					col_max = col_max - (col_diff / 2) - 1;
				}
				break;
			}
			case 'R':
			{
				int col_diff = col_max - col_min;
				if (col_diff == 1) {
					col = col_max;
				} else {
					col_min = col_min + (col_diff / 2) + 1;
				}
				break;
			}
		}
		advance(cursor);
		cur = input[*cursor];
	}
	return 8 * row + col;
}

void part_one(char *input) {
	int cursor = 0, highest_seat_id = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		if (cur == 'F' || cur == 'B' || cur == 'L' || cur == 'R') {
			int seat_id = read_boarding_pass(input, &cursor);
			if (seat_id > highest_seat_id) {
				highest_seat_id = seat_id;
			}
		} else {
			advance(&cursor);
		}
		cur = input[cursor];
	}
	printf("highest seat id: %d\n", highest_seat_id);
}

void part_two(char *input) {
	int cursor = 0, lines = 0, i = 0, min = 10000, max = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		if (cur == '\n') {
			lines++;
		}
		cursor++;
		cur = input[cursor];
	}
	int* all_seats = malloc(lines);
	cursor = 0;
	cur = input[cursor];
	while (cur != '\0') {
		if (cur == 'F' || cur == 'B' || cur == 'L' || cur == 'R') {
			int seat_id = read_boarding_pass(input, &cursor);
			if (seat_id < min) {
				min = seat_id;
			}
			if (seat_id > max) {
				max = seat_id;
			}
			all_seats[i] = seat_id;
			i++;
		} else {
			advance(&cursor);
		}
		cur = input[cursor];
	}

	// this seems slow but oh well
	for (int j = 0; j < lines; j++) {
		int candidate_seat = all_seats[j];
		if (candidate_seat != min && candidate_seat != max) {
			int found_ahead = 0;
			int found_behind = 0;

			for (int k = 0; k < lines; k++) {
				if (all_seats[k] == (candidate_seat + 1)) {
					found_ahead = 1;
				}
				if (all_seats[k] == (candidate_seat -1 )) {
					found_behind = 1;
				}
			}

			if (!found_ahead) {
				printf("missing ahead of %d\n", candidate_seat);
			}
			if (!found_behind) {
				printf("missing behind of %d\n", candidate_seat);
			}
		}
	}
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
//   char* fakebuf = "FBFBBFFRLR\n";
//   int cursor = 0;
//   int result = read_boarding_pass(fakebuf, &cursor);
//   assert(result == 357);
// }
