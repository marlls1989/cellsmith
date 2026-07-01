define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q} \
	-vector {1 R R} \
	-related_pin B \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q} \
	-vector {R 1 R} \
	-related_pin A \
	-pin Q \
	{ C2 }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 10 11 01} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {0 F F} \
	-related_pin B \
	-pin Q \
	{ C2 }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 10 11 10} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {F 0 F} \
	-related_pin A \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {R 1 0 R} \
	-related_pin A \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {000 100 110} \
	-type async \
	-pinlist {A B R Q} \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {000 100 110 010} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {0 F 0 F} \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {000 100 110 010} \
	-type async \
	-pinlist {A B R Q} \
	-vector {0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {000 100 110 100} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {F 0 0 F} \
	-related_pin A \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {000 100 110 100} \
	-type async \
	-pinlist {A B R Q} \
	-vector {1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type async \
	-prevector_pinlist {A B R} \
	-prevector {000 100 110 111} \
	-pinlist {A B R Q} \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {00} \
	-pinlist {S R Q Qn} \
	-vector {R 0 R X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {00} \
	-pinlist {S R Q Qn} \
	-vector {0 R X R} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {00} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {0 R F X} \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {00} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {R 0 X F} \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {00 10} \
	-pinlist {S R Q Qn} \
	-vector {1 R X R} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {00 01} \
	-pinlist {S R Q Qn} \
	-vector {R 1 R X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {00 10 11} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {F 1 F X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {00 10 11} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {1 F X F} \
	-related_pin R \
	-pin Qn \
	{ SR }

# arbitration: A*B metastable; grants {Qa, Qb} mutually exclusive ({Qa=0, Qb=1} | {Qa=1, Qb=0})
define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {R 0 R X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {0 R X R} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {F 0 F X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {0 F X F} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 10 11} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {F 1 F X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10 11} \
	-pinlist {A B Qa Qb} \
	-vector {F 1 X R} \
	-related_pin A \
	-pin Qb \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01 11} \
	-pinlist {A B Qa Qb} \
	-vector {1 F R X} \
	-related_pin B \
	-pin Qa \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 01 11} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {1 F X F} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-type combinational \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

