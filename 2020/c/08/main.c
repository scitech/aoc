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

typedef struct intset {
	int* numbers;
	int size;
} intset;

intset new_intset() {
	intset iset;
	iset.size = 0;
	return iset;
}

int intset_has(intset* iset, int val) {
	int has = 0;
	for (int i = 0; i < iset->size; i++) {
		if (*(iset->numbers + i) == val) {
			has = 1;
			break;
		}
	}
	return has;
}

void intset_add(intset* iset, int val) {
	if (intset_has(iset, val) == 1) return;

	int new_size = iset->size + 1;
	if (iset->size == 0) {
		iset->numbers = malloc(new_size * sizeof(int));
	} else {
		iset->numbers = realloc(iset->numbers, new_size * sizeof(int));
	}
	(iset->numbers)[iset->size] = val;
	iset->size++;
}

typedef struct instruction {
	char operation[OP_LENGTH + 1];
	int argument;
} instruction;

typedef struct program {
	struct instruction* instructions;
	int accumulator;
	int instructions_count;
	int current_instruction_index;
} program;

void move_reader_to_offset(reader* r, int offset) {
	r->position = offset;
	r->current_ch = (r->text)[offset];
}

void read_next(reader* r) {
	move_reader_to_offset(r, r->position + 1);
}

void move_reader_to_next_line(reader* r) {
	while (r->current_ch != '\0') {
		read_next(r);
		if (r->current_ch == '\n') {
			read_next(r);
			break;
		}
	}
}

int find_next_line_start(reader* r) {
	int pos = r->position;
	char ch = r->current_ch;
	while (ch != '\0') {
		pos++;
		ch = (r->text)[pos];
		if (ch == '\n') {
			break;
		}
	}
	return pos + 1;
}

instruction init_instruction() {
	instruction new_instruction = {
		{ '\0' },
		0
	};
	return new_instruction;
}
program init_program() {
	program p;
	p.instructions = NULL;
	p.accumulator = 0;
	p.instructions_count = 0;
	p.current_instruction_index = 0;
	return p;
}

void read_operation(reader* r, char* op) {
	char* text_from_current_position = r->text + r->position;
	strncpy(op, text_from_current_position, OP_LENGTH);
	move_reader_to_offset(r, r->position + OP_LENGTH + 1);
}

char read_character(reader* r) {
	char sign = r->current_ch;
	read_next(r);
	return sign;
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

void add_instruction(program* p, instruction in) {
	int new_instructions_count = p->instructions_count + 1;
	if (p->instructions_count == 0) {
		p->instructions = malloc(new_instructions_count * sizeof(instruction));
	} else {
		p->instructions = realloc(p->instructions, new_instructions_count * sizeof(instruction));
	}
	(p->instructions)[p->instructions_count] = in;
	p->instructions_count++;
}

void print_instruction(instruction* in) {
	printf("instruction: %s %d\n", in->operation, in->argument);
}

instruction read_instruction(reader* r) {
	instruction in = init_instruction();
	read_operation(r, in.operation);
	char sign = read_character(r);
	int number = read_integer(r);
	if (sign == '-') {
		number = -1 * number;
	}
	in.argument = number;
	read_next(r);
	return in;
}

void do_acc(program* p, int arg) {
	p->accumulator += arg;
	p->current_instruction_index++;
}

void do_jmp(program* p, int arg) {
	p->current_instruction_index += arg;
}

void do_nop(program* p) {
	p->current_instruction_index++;
}

void reset_program(program* p) {
	p->accumulator = 0;
	p->current_instruction_index = 0;
}

int run_program(program* p) {
	intset lines_we_ran = new_intset();
	int success = 1;
	while (p->current_instruction_index < p->instructions_count) {
		if (intset_has(&lines_we_ran, p->current_instruction_index) == 1) {
			success = 0;
			break;
		}
		intset_add(&lines_we_ran, p->current_instruction_index);
		instruction current_instruction = *(p->instructions + p->current_instruction_index);
		if (strcmp(current_instruction.operation, "acc") == 0) {
			do_acc(p, current_instruction.argument);
		} else if (strcmp(current_instruction.operation, "jmp") == 0) {
			do_jmp(p, current_instruction.argument);
		} else if (strcmp(current_instruction.operation, "nop") == 0) {
			do_nop(p);
		} else {
			printf("illegal operation\n");
			exit(1);
		}
	}
	printf("ran to end? %d, result: %d\n", success, p->accumulator);
	reset_program(p);
	return success;
}

void part_one(program* p) {
	run_program(p);
}


void part_two(program* p) {
	for (int i = 0; i < p->instructions_count; i++) {
		instruction* current_instruction = p->instructions + i;
		int ran_program_to_end = 0;
		if (strcmp(current_instruction->operation, "jmp") == 0) {
			// flip instruction to nop and run program
			strncpy(current_instruction->operation, "nop", OP_LENGTH);
			ran_program_to_end = run_program(p);
			if (ran_program_to_end == 1) {
				break;
			} else {
				strncpy(current_instruction->operation, "jmp", OP_LENGTH);
			}
			p->accumulator = 0;
		} else if (strcmp(current_instruction->operation, "nop") == 0) {
			// flip instruction to jmp and run program
			strncpy(current_instruction->operation, "jmp", OP_LENGTH);
			ran_program_to_end = run_program(p);
			if (ran_program_to_end == 1) {
				break;
			} else {
				strncpy(current_instruction->operation, "nop", OP_LENGTH);
			}
			p->accumulator = 0;
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

	reader r = new_reader(input);
	program p = init_program();
	while (r.current_ch != 0) {
		instruction in = read_instruction(&r);
		add_instruction(&p, in);
	}

	printf("part1:\n");
	part_one(&p);
	printf("\npart2:\n");
	part_two(&p);
	printf("\n");

	return 0;
}