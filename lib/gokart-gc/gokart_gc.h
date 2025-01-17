#include <stdint.h>

struct gokart_value {
    uint64_t header;
    struct gokart_value* next;
    uint64_t size;
};

struct gokart_heap {
    struct gokart_value* head;
    uint64_t bytes_allocated;
    uint64_t objects_allocated;
};

uint64_t gokart_get_tag(struct gokart_value* v);
void gokart_set_tag(struct gokart_value* v, uint64_t tag);
uint8_t gokart_get_color(struct gokart_value* v);
void gokart_set_color(struct gokart_value* v,  uint8_t color);

struct gokart_value* gokart_allocate(struct gokart_heap* h, uint64_t size, void (*finalizer)(struct gokart_value*));
void gokart_sweep(struct gokart_heap* h);
