#include<stdint.h>
#include<stdio.h>
#include<ruby.h>

extern int32_t double_input(int32_t input);

VALUE bad_double_input(VALUE module, VALUE obj) {
	Check_Type(obj, T_FIXNUM);

	int32_t num = NUM2INT(obj);
	int32_t doubled = double_input(num);

	printf("%i\n", doubled);

	return Qnil;
}

void Init_bad_idea(void) {
	VALUE bad_idea = rb_define_module("BadIdea");
	rb_define_singleton_method(bad_idea, "double_input", bad_double_input, 1);
}
