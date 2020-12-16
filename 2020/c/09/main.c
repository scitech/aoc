#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define OP_LENGTH 3

char* load_input(FILE *fp) {
	fseek(fp, 0, SEEK_END);
	size_t length = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char *buffer = malloc(length + 1);
	fread(buffer, sizeof(char), length, fp);
	return buffer;
}

typedef struct reader {
	char* text;
	char current_ch;
	int position;
} reader;

reader new_reader(char* text) {
	reader r;
	r.text = text;
	r.position = 0;
	r.current_ch = text[0];
	return r;
}

int reader_input_ended(reader* r) {
	return r->current_ch == '\0';
}

typedef struct intlist {
	int* numbers;
	int size;
} intlist;

intlist new_intlist() {
	intlist list;
	list.size = 0;
	return list;
}

void intlist_add(intlist* ilist, int val) {
	int new_size = ilist->size + 1;
	if (ilist->size == 0) {
		ilist->numbers = malloc(new_size * sizeof(int));
	} else {
		ilist->numbers = realloc(ilist->numbers, new_size * sizeof(int));
	}
	(ilist->numbers)[ilist->size] = val;
	ilist->size++;
}

void move_reader_to_offset(reader* r, int offset) {
	r->position = offset;
	r->current_ch = (r->text)[offset];
}

void read_next(reader* r) {
	move_reader_to_offset(r, r->position + 1);
}

void move_reader_to_next_line(reader* r) {
	if (r->current_ch == '\n') {
		read_next(r);
		return;
	}
	while (r->current_ch != '\0') {
		read_next(r);
		if (r->current_ch == '\n') {
			read_next(r);
			break;
		}
	}
}

int read_integer(reader* r) {
	int count = strtoul(r->text + r->position, NULL, 10);
	int num_digits = 0;
	int result = count;
	if (count == 0) {
		num_digits = 1;
	} else {
		while (result != 0) {
			result /= 10;
			num_digits++;
		}
	}
	move_reader_to_offset(r, r->position + num_digits);
	return count;
}

intlist read_integer_list(reader* r) {
	intlist il = new_intlist();
	while (reader_input_ended(r) == 0) {
		int num = read_integer(r);
		intlist_add(&il, num);
		move_reader_to_next_line(r);
	}
	return il;
}

intlist slice_integer_list(intlist* input, int min, int max) {
	intlist il = new_intlist();
	for (int i = min; i <= max; i++) {
		intlist_add(&il, *(input->numbers + i));
	}
	return il;
}

void print_intlist(intlist* ilist) {
	for (int i = 0; i < ilist->size; i++) {
		printf("%d\n", *(ilist->numbers + i));
	}
}

int find_bad_number(intlist* ilist, int chunk_size) {
	int bad_number = 0;

	for (int i = chunk_size; i < ilist->size; i++) {
		int is_current_number_valid = 0;
		int number_to_check = *(ilist->numbers + i);
		int min = i - chunk_size;
		int max = i;
		for (int j = min; j < max; j++) {
			int adder_a = *(ilist->numbers + j);
			for (int k = min; k < max; k++) {
				int adder_b = *(ilist->numbers + k);
				if (adder_a != adder_b) {
					int sum = adder_a + adder_b;
					if (number_to_check == sum) {
						is_current_number_valid = 1;
						break;
					}
				}
			}
			if (is_current_number_valid == 1) {
				break;
			}
		}

		if (is_current_number_valid == 0) {
			bad_number = number_to_check;
		}
	}

	return bad_number;
}


void clear_intlist(intlist* ilist) {
	free(ilist->numbers);
	ilist->size = 0;
}

int sum_intlist(intlist* ilist) {
	int sum = 0;
	for (int i = 0; i < ilist->size; i++) {
		sum += *(ilist->numbers + i);
	}
	return sum;
}

int intlist_min(intlist* ilist) {
	int min = *(ilist->numbers);
	for (int i = 0; i < ilist->size; i++) {
		int num = *(ilist->numbers + i);
		min = min > num ? num : min;
	}
	return min;
}
int intlist_max(intlist* ilist) {
	int max = *(ilist->numbers);
	for (int i = 0; i < ilist->size; i++) {
		int num = *(ilist->numbers + i);
		max = max > num ? max : num;
	}
	return max;
}

intlist find_bad_range(intlist* ilist, int bad_number) {
	intlist il = new_intlist();
	for (int i = 0; *(ilist->numbers + i) != bad_number; i++) {
		int val = *(ilist->numbers + i);
		intlist_add(&il, val);
		for (int j = i + 1; *(ilist->numbers + j) != bad_number; j++) {
			int next_val = *(ilist->numbers + j);
			intlist_add(&il, next_val);
			int sum = sum_intlist(&il);
			if (sum > bad_number) {
				break;
			}

			if (sum == bad_number) {
				break;
			}
		}
		if (sum_intlist(&il) == bad_number) {
			printf("we did it\n");
			break;
		} else {
			clear_intlist(&il);
		}
	}
	return il;
}

void part_one(intlist* ilist) {
	int bad_number = find_bad_number(ilist, 25);
	printf("bad number: %d\n", bad_number);
}

void part_two(intlist* ilist) {
	int bad_number = find_bad_number(ilist, 25);
	printf("bad number: %d\n", bad_number);
	intlist bad_range = find_bad_range(ilist, bad_number);
	print_intlist(&bad_range);
	int min = intlist_min(&bad_range);
	int max = intlist_max(&bad_range);
	printf("really we did it, min: %d, max: %d\n", min, max);
	printf("encryption weakness: %d\n", min + max);
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

	reader r = new_reader(input);
	intlist il = read_integer_list(&r);
	// printf("part1:\n");
	// part_one(&il);

	printf("\npart2:\n");
	part_two(&il);
	printf("\n");

	return 0;
}