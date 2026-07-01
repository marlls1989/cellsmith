define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {F 0 F} \
	-related_pin A \
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
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {0 F F} \
	-related_pin B \
	-pin Q \
	{ C2 }

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
	-type non_seq_setup \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Q} \
	-vector {F R X} \
	-related_pin A \
	-pin B \
	{ C2 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Q} \
	-vector {F R X} \
	-related_pin A \
	-pin B \
	{ C2 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Q} \
	-vector {R F X} \
	-related_pin A \
	-pin B \
	{ C2 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Q} \
	-vector {R F X} \
	-related_pin A \
	-pin B \
	{ C2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {F 0 0 F} \
	-related_pin A \
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
	-prevector {110 010} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {0 F 0 F} \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

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
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-type async \
	-pinlist {A B R Q} \
	-vector {0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type async \
	-pinlist {A B R Q} \
	-vector {1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-type async \
	-pinlist {A B R Q} \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type async \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {A B R} \
	-prevector {100} \
	-pinlist {A B R Q} \
	-vector {F R 0 X} \
	-related_pin A \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B R} \
	-prevector {100} \
	-pinlist {A B R Q} \
	-vector {F R 0 X} \
	-related_pin A \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {F 1 F X} \
	-related_pin A \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {F 1 F X} \
	-related_pin A \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {A B R} \
	-prevector {010} \
	-pinlist {A B R Q} \
	-vector {R F 0 X} \
	-related_pin A \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B R} \
	-prevector {010} \
	-pinlist {A B R Q} \
	-vector {R F 0 X} \
	-related_pin A \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 F F X} \
	-related_pin B \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 F F X} \
	-related_pin B \
	-pin R \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 F 0 F} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 R 0 R} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 R 0 R} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 R 0 R} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 R 0 R} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 0 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 0 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 0 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 1 0 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 1 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 1 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 1 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 0 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 0 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 0 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 0 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 1 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 1 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 1 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 1 1 0 F} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 1 0 R} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000110 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 1 0 R} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000110 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 1 0 R} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 1 0 R} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 F 1 0 F} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 1 0 R} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001010 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 1 0 R} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001010 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 1 0 R} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 1 0 R} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 0 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-type async \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type async \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type async \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type async \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type async \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 1 F 0 X} \
	-related_pin M1 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 1 F 0 X} \
	-related_pin M1 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 1 F 0 X} \
	-related_pin M2 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 1 F 0 X} \
	-related_pin M2 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 F 0 X} \
	-related_pin P1 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 F 0 X} \
	-related_pin P1 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R F 0 X} \
	-related_pin P2 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R F 0 X} \
	-related_pin P2 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 F F X} \
	-related_pin C \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 F F X} \
	-related_pin C \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 R 0 X} \
	-related_pin P1 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 R 0 X} \
	-related_pin P1 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F R 0 X} \
	-related_pin P2 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F R 0 X} \
	-related_pin P2 \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F R 0 0 0 0 X} \
	-related_pin M1 \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F R 0 0 0 0 X} \
	-related_pin M1 \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R F 0 0 0 0 X} \
	-related_pin M1 \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R F 0 0 0 0 X} \
	-related_pin M1 \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 F 1 1 0 X} \
	-related_pin M1 \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 F 1 1 0 X} \
	-related_pin M1 \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 F 1 0 X} \
	-related_pin M1 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 F 1 0 X} \
	-related_pin M1 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R F 1 1 0 X} \
	-related_pin M2 \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R F 1 1 0 X} \
	-related_pin M2 \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 F 1 0 X} \
	-related_pin M2 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 F 1 0 X} \
	-related_pin M2 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F R 1 0 X} \
	-related_pin P1 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F R 1 0 X} \
	-related_pin P1 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 1 F X} \
	-related_pin P1 \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 1 F X} \
	-related_pin P1 \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R F 1 0 X} \
	-related_pin P1 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R F 1 0 X} \
	-related_pin P1 \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 1 F X} \
	-related_pin P2 \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 1 F X} \
	-related_pin P2 \
	-pin R \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {10 00} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {0 R F X} \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {F 1 F X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {01 00} \
	-pinlist {S R Q Qn} \
	-vector {R 0 R X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {01} \
	-pinlist {S R Q Qn} \
	-vector {R 1 R X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {1 F X F} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {10 00} \
	-pinlist {S R Q Qn} \
	-vector {0 R X R} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {10} \
	-pinlist {S R Q Qn} \
	-vector {1 R X R} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {01 00} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {R 0 X F} \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-pinlist {S R Q Qn} \
	-vector {F F X X} \
	-related_pin S \
	-pin R \
	{ SR }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-pinlist {S R Q Qn} \
	-vector {F F X X} \
	-related_pin S \
	-pin R \
	{ SR }

# arbitration: A*B metastable; grants {Qa, Qb} mutually exclusive ({Qa=0, Qb=1} | {Qa=1, Qb=0})
define_arc \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {F 0 F X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {10 11} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {F 1 F X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

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
	-prevector {01 11} \
	-pinlist {A B Qa Qb} \
	-vector {1 F R X} \
	-related_pin B \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {10 11} \
	-pinlist {A B Qa Qb} \
	-vector {F 1 X R} \
	-related_pin A \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {0 F X F} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {01 11} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {1 F X F} \
	-related_pin B \
	-pin Qb \
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
	-type non_seq_setup \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {R R X X} \
	-related_pin A \
	-pin B \
	{ MUT }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {R R X X} \
	-related_pin A \
	-pin B \
	{ MUT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type combinational \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type setup \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-pinlist {CLK D Q} \
	-vector {R F X} \
	-related_pin CLK \
	-pin D \
	{ DFF }

define_arc \
	-type hold \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-pinlist {CLK D Q} \
	-vector {R F X} \
	-related_pin CLK \
	-pin D \
	{ DFF }

define_arc \
	-type setup \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q} \
	-vector {R R X} \
	-related_pin CLK \
	-pin D \
	{ DFF }

define_arc \
	-type hold \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q} \
	-vector {R R X} \
	-related_pin CLK \
	-pin D \
	{ DFF }

# arbitration: !CLKA*!CLKB*!RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=1, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=1, enB=1} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=1, enB=1})
# arbitration: CLKA*!CLKB*!RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=1, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=0, selb1=0, selb2=1, enB=1} | {sela=1, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=1, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=1, enB=1})
# arbitration: !CLKA*CLKB*!RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=1, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=1, selb2=1, enB=1})
# arbitration: CLKA*CLKB*!RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=0, selb1=1, selb2=1, enB=1} | {sela=1, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=0, sela2=0, enA=1, selb1=1, selb2=1, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=1, selb2=1, enB=1})
# arbitration: !CLKA*!CLKB*RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=1, enB=1})
# arbitration: CLKA*!CLKB*RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=1, enB=1})
# arbitration: !CLKA*CLKB*RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1})
# arbitration: CLKA*CLKB*RA*!RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1})
# arbitration: !CLKA*!CLKB*!RA*RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=1, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: CLKA*!CLKB*!RA*RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: !CLKA*CLKB*!RA*RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=1, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: CLKA*CLKB*!RA*RB*!S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=1, selb=0, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=1, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: !CLKA*!CLKB*!RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=1, enB=1})
# arbitration: CLKA*!CLKB*!RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=0, enB=0} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=1, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=1, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=1, enB=1})
# arbitration: !CLKA*CLKB*!RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=1, selb2=1, enB=1})
# arbitration: CLKA*CLKB*!RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=1, selb2=1, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=1, selb2=1, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=1} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=1, selb2=1, enB=1})
# arbitration: !CLKA*!CLKB*RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1})
# arbitration: CLKA*!CLKB*RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1})
# arbitration: !CLKA*CLKB*RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1})
# arbitration: CLKA*CLKB*RA*!RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=0} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=1} | {sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=1, selb2=1, enB=1})
# arbitration: !CLKA*!CLKB*!RA*RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: CLKA*!CLKB*!RA*RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: !CLKA*CLKB*!RA*RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
# arbitration: CLKA*CLKB*!RA*RB*S metastable; grants {sela, selb, sela1, sela2, enA, selb1, selb2, enB} mutually exclusive ({sela=0, selb=1, sela1=0, sela2=0, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=1, sela1=1, sela2=1, enA=0, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=0, sela2=0, enA=1, selb1=0, selb2=0, enB=0} | {sela=0, selb=0, sela1=1, sela2=1, enA=1, selb1=0, selb2=0, enB=0})
define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 0 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 1 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 0 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 1 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 0 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 1 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 0 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 1 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 1 0 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 1 1 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 0 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 1 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 0 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 1 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 0 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 0 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 0 0 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 1 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 1 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 0 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 0 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-type async \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F R 0 0 1 X} \
	-related_pin CLKA \
	-pin CLKB \
	{ ICM }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F R 0 0 1 X} \
	-related_pin CLKA \
	-pin CLKB \
	{ ICM }

define_arc \
	-type non_seq_setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R F 0 0 0 X} \
	-related_pin CLKA \
	-pin CLKB \
	{ ICM }

define_arc \
	-type non_seq_hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R F 0 0 0 X} \
	-related_pin CLKA \
	-pin CLKB \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 F 1 0 X} \
	-related_pin CLKA \
	-pin RA \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 F 1 0 X} \
	-related_pin CLKA \
	-pin RA \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 R 0 X} \
	-related_pin CLKA \
	-pin RB \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 R 0 X} \
	-related_pin CLKA \
	-pin RB \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 F X} \
	-related_pin CLKA \
	-pin S \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 F X} \
	-related_pin CLKA \
	-pin S \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 R X} \
	-related_pin CLKA \
	-pin S \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 R X} \
	-related_pin CLKA \
	-pin S \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R R 0 1 X} \
	-related_pin CLKB \
	-pin RA \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R R 0 1 X} \
	-related_pin CLKB \
	-pin RA \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 F 1 X} \
	-related_pin CLKB \
	-pin RB \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 F 1 X} \
	-related_pin CLKB \
	-pin RB \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 F X} \
	-related_pin CLKB \
	-pin S \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 F X} \
	-related_pin CLKB \
	-pin S \
	{ ICM }

define_arc \
	-type setup \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 R X} \
	-related_pin CLKB \
	-pin S \
	{ ICM }

define_arc \
	-type hold \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 R X} \
	-related_pin CLKB \
	-pin S \
	{ ICM }

