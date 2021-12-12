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

void print_intlist(intlist* ilist) {
	for (int i = 0; i < ilist->size; i++) {
		printf("%d\n", *(ilist->numbers + i));
	}
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
int compare_ints(const void* el_a, const void* el_b) {
	int a = *((int*)el_a);
	int b = *((int*)el_b);
	if (a > b) return  1;
	if (a < b) return -1;
	return 0;
}
int intlist_max(intlist* ilist) {
	int max = *(ilist->numbers);
	for (int i = 0; i < ilist->size; i++) {
		int num = *(ilist->numbers + i);
		max = max > num ? max : num;
	}
	return max;
}

void part_one(intlist* ilist) {
	int ones = 0;
	int twos = 0;
	int threes = 0;
	for (int i = 1; i < ilist->size; i++) {
		int num = *(ilist->numbers + i);
		int diff = num - *(ilist->numbers + i - 1);
		switch (diff) {
			case 1:
				ones++;
				break;
			case 2:
				twos++;
				break;
			case 3:
				threes++;
				break;
			default:
				printf("wrong diff %d\n", diff);
				exit(1);
		}
	}
	print_intlist(ilist);
	printf("ones: %d, twos: %d, threes: %d\n", ones, twos, threes);
}

void part_two(intlist* ilist) {
	intlist paths_list = new_intlist();
	intlist_add(&paths_list, 1);
	for (int i = 1; i < ilist->size; i++) {
		int adapter = *(ilist->numbers + i);
		for (int j = 1; j < 4; j++) {
			
		}
	}
	print_intlist(&paths_list);
	printf("max: %d\n", intlist_max(&paths_list));
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
	intlist_add(&il, 0);
	int built_in_adapter = intlist_max(&il) + 3;
	intlist_add(&il, built_in_adapter);
	qsort(il.numbers, il.size, sizeof(int), compare_ints);
	printf("part1:\n");
	part_one(&il);

	printf("\npart2:\n");
	part_two(&il);
	printf("\n");

	return 0;
}