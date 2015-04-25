#include<stdint.h>
#include<stdio.h>
#include<ruby.h>

typedef struct {
	uintptr_t data;
	uint len;
	uint capacity;
} array_t;

extern void array_push(array_t *a, uintptr_t item);
extern void array_inspect(array_t *a, char *s);

static VALUE bad_array_allocate(VALUE klass) {
	array_t *data;

	// by not defining the free here, we'll leak the data pointer
	//
	// ... so fix that someday
	VALUE vec = Data_Make_Struct(klass, array_t, NULL, -1, data);

	return vec;
}

VALUE bad_array_push(VALUE obj, VALUE item) {
        array_t *a;
        Data_Get_Struct(obj, array_t, a);

	array_push(a, item);

	return obj;
}

VALUE bad_array_inspect(VALUE obj) {
	array_t *a;
	Data_Get_Struct(obj, array_t, a);

	char *ptr = malloc(100); // LOL HAX

	array_inspect(a, ptr);
        VALUE result = rb_str_new_cstr(ptr) ;

	free(ptr);

	return result;
}

void Init_bad_idea(void) {
	VALUE array = rb_define_class("Array", rb_cObject);
	rb_define_alloc_func(array, bad_array_allocate);

	rb_define_method(array, "<<", bad_array_push, 1);
	rb_define_method(array, "inspect", bad_array_inspect, 0);
}
