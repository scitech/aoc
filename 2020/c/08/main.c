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

char* read_operation(reader* r, char* op) {
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

int main() {
	// FILE *fp;
	// fp = fopen("input", "r");
	// if (fp == NULL) {
	// 	puts("could not open file");
	// 	return 1;
	// }
	// char *input = load_input(fp);
	// if (input == NULL) {
	// 	puts("could not malloc");
	// 	return 1;
	// }
	// fclose(fp);

  char* input = "nop +0\n"
    "acc +1\n"
    "jmp +4\n"
    "acc +3\n"
    "jmp -3\n"
    "acc -99\n"
    "acc +1\n"
    "jmp -4\n"
    "acc +6\n";
  struct reader r = new_reader(input);
  struct program p = init_program();
  while (r.current_ch != 0) {
    instruction in = read_instruction(&r);
    add_instruction(&p, in);
  }

  for (int i = 0; i < p.instructions_count; i++) {
    print_instruction(p.instructions + i);
  }
  printf("\n");

	return 0;
}

// int main() {
//   // test example from the prompt
//   part_one(input);
// }
