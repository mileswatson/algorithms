from strip_hints import strip_file_to_string
code_string = strip_file_to_string("maxflow.py", to_empty=False, strip_nl=False,
                                   no_ast=False, no_colon_move=False,
                                   no_equal_move=False,
                                   only_assigns_and_defs=False,
                                   only_test_for_changes=False)
print(code_string)
