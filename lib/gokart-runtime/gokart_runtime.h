#include <stdint.h>

struct gokart_value {
    uint64_t header;
    struct gokart_value* next;
    uint64_t size;
};

struct gokart_gc {
    struct gokart_value* head;
    uint64_t bytes_allocated;
    uint64_t objects_allocated;
    uint64_t objects_threshold;
    uint64_t bytes_threshold;
};

struct gokart_stack {
    uint64_t capacity;
    uint64_t length;
    struct gokart_value** data;
};

struct gokart_machine {
    struct gokart_value* env;
    struct gokart_stack stack;
    struct gokart_gc* gc;
    uint64_t is_running;
    uint64_t ip;
};

uint64_t gokart_get_tag(struct gokart_value* v);
void gokart_set_tag(struct gokart_value* v, uint64_t tag);
uint8_t gokart_get_color(struct gokart_value* v);
void gokart_set_color(struct gokart_value* v,  uint8_t color);

struct gokart_value* gokart_allocate(struct gokart_machine* m, uint64_t size, void (*finalizer)(struct gokart_value*));
struct gokart_value* gokart_allocate_vector_int(struct gokart_machine* m);
struct gokart_value* gokart_allocate_string(struct gokart_machine* m, uint64_t size, uint8_t* ptr);
struct gokart_value* gokart_allocate_int(struct gokart_machine* m, int64_t data);
struct gokart_value* gokart_allocate_double(struct gokart_machine* m, double data);
struct gokart_value* gokart_allocate_label(struct gokart_machine* m, uint64_t lbl);
struct gokart_value* gokart_allocate_pair(struct gokart_machine* m, struct gokart_value* lhs, struct gokart_value* rhs);
struct gokart_value* gokart_allocate_tagged(struct gokart_machine* m, uint64_t tag, struct gokart_value* rhs);
struct gokart_value* gokart_allocate_closure(struct gokart_machine* m, struct gokart_value* lhs, uint64_t lbl);

void gokart_sweep(struct gokart_machine* m);
void gokart_mark_sweep(struct gokart_machine* m, struct gokart_value* tmp);

void gokart_stack_push(struct gokart_machine* m, struct gokart_value* v);
struct gokart_value* gokart_stack_peek(struct gokart_machine *m);
struct gokart_value* gokart_stack_pop(struct gokart_machine *m);

struct gokart_machine* gokart_machine_init();
void gokart_machine_free(struct gokart_machine* m);

