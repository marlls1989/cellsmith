define_arc \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-type combinational \
	-pinlist {A B Y} \
	-vector {F 1 F} \
	-when "B" \
	-related_pin A \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Y} \
	-vector {R 1 R} \
	-when "B" \
	-related_pin A \
	-pin Y \
	{ AND2 }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-type combinational \
	-pinlist {A B Y} \
	-vector {1 F F} \
	-when "A" \
	-related_pin B \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Y} \
	-vector {1 R R} \
	-when "A" \
	-related_pin B \
	-pin Y \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Y} \
	-vector {F 0 0} \
	-when "!B*!Y" \
	-pin A \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Y} \
	-vector {R 0 0} \
	-when "!B*!Y" \
	-pin A \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Y} \
	-vector {0 F 0} \
	-when "!A*!Y" \
	-pin B \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Y} \
	-vector {0 R 0} \
	-when "!A*!Y" \
	-pin B \
	{ AND2 }

define_leakage -when "!A*!B*!Y" { AND2 }
define_leakage -when "!A*B*!Y" { AND2 }
define_leakage -when "A*!B*!Y" { AND2 }
define_leakage -when "A*B*Y" { AND2 }
define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {F 0 F} \
	-when "!B" \
	-related_pin A \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q} \
	-vector {R 1 R} \
	-when "B" \
	-related_pin A \
	-pin Q \
	{ C2 }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {0 F F} \
	-when "!A" \
	-related_pin B \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q} \
	-vector {1 R R} \
	-when "A" \
	-related_pin B \
	-pin Q \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q} \
	-vector {F 0 0} \
	-when "!B*!Q" \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {F 1 1} \
	-when "B*Q" \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {R 0 0} \
	-when "!B*!Q" \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-pinlist {A B Q} \
	-vector {R 1 1} \
	-when "B*Q" \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q} \
	-vector {0 F 0} \
	-when "!A*!Q" \
	-pin B \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {1 F 1} \
	-when "A*Q" \
	-pin B \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {0 R 0} \
	-when "!A*!Q" \
	-pin B \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-pinlist {A B Q} \
	-vector {1 R 1} \
	-when "A*Q" \
	-pin B \
	{ C2 }

define_leakage -when "!A*!B*!Q" { C2 }
define_leakage -when "A*B*Q" { C2 }
define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {F 0 0 F} \
	-when "!B*!R" \
	-related_pin A \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {R 1 0 R} \
	-when "B*!R" \
	-related_pin A \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {0 F 0 F} \
	-when "!A*!R" \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {1 R 0 R} \
	-when "A*!R" \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {0 1 R F} \
	-when "!A*B" \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {1 0 R F} \
	-when "A*!B" \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {1 1 R F} \
	-when "A*B" \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 1 F R} \
	-when "A*B" \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {F 0 0 0} \
	-when "!B*!Q*!R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {F 0 1 0} \
	-when "!B*!Q*R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {F 1 1 0} \
	-when "B*!Q*R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-pinlist {A B R Q} \
	-vector {F 1 0 1} \
	-when "B*Q*!R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {R 0 0 0} \
	-when "!B*!Q*!R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {R 0 1 0} \
	-when "!B*!Q*R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {R 1 1 0} \
	-when "B*!Q*R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-pinlist {A B R Q} \
	-vector {R 1 0 1} \
	-when "B*Q*!R" \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {0 F 0 0} \
	-when "!A*!Q*!R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {0 F 1 0} \
	-when "!A*!Q*R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 F 1 0} \
	-when "A*!Q*R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-pinlist {A B R Q} \
	-vector {1 F 0 1} \
	-when "A*Q*!R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {0 R 0 0} \
	-when "!A*!Q*!R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {0 R 1 0} \
	-when "!A*!Q*R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {1 R 1 0} \
	-when "A*!Q*R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-pinlist {A B R Q} \
	-vector {1 R 0 1} \
	-when "A*Q*!R" \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {0 0 F 0} \
	-when "!A*!B*!Q" \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {0 1 F 0} \
	-when "!A*B*!Q" \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {1 0 F 0} \
	-when "A*!B*!Q" \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {0 0 R 0} \
	-when "!A*!B*!Q" \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {0 1 R 0} \
	-when "!A*B*!Q" \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {1 0 R 0} \
	-when "A*!B*!Q" \
	-pin R \
	{ RCELEM2 }

define_leakage -when "!A*!B*!Q*!R" { RCELEM2 }
define_leakage -when "!A*!B*!Q*R" { RCELEM2 }
define_leakage -when "!A*B*!Q*R" { RCELEM2 }
define_leakage -when "A*!B*!Q*R" { RCELEM2 }
define_leakage -when "A*B*Q*!R" { RCELEM2 }
define_leakage -when "A*B*!Q*R" { RCELEM2 }
define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110 000010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 F 0 F} \
	-when "!M1*!M2*!P1*!P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 F 0 F} \
	-when "!M1*!M2*!P1*P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 001010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 F 0 F} \
	-when "!M1*!M2*P1*!P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 F 0 F} \
	-when "!M1*!M2*P1*P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 R 0 R} \
	-when "!M1*!M2*P1*P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 R 0 R} \
	-when "!M1*M2*P1*P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 R 0 R} \
	-when "M1*!M2*P1*P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 R 0 R} \
	-when "M1*M2*P1*P2*!R" \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 0 0 F} \
	-when "!C*!M2*!P1*!P2*!R" \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 0 0 F} \
	-when "!C*!M2*!P1*P2*!R" \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 0 0 F} \
	-when "!C*!M2*P1*!P2*!R" \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 1 0 0 F} \
	-when "!C*!M2*P1*P2*!R" \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 0 0 F} \
	-when "!C*!M1*!P1*!P2*!R" \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 0 0 F} \
	-when "!C*!M1*!P1*P2*!R" \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 0 0 F} \
	-when "!C*!M1*P1*!P2*!R" \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 0 0 F} \
	-when "!C*!M1*P1*P2*!R" \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 1 0 R} \
	-when "C*!M1*!M2*P2*!R" \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 1 0 R} \
	-when "C*!M1*M2*P2*!R" \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 1 0 R} \
	-when "C*M1*!M2*P2*!R" \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 1 0 R} \
	-when "C*M1*M2*P2*!R" \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 1 0 R} \
	-when "C*!M1*!M2*P1*!R" \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 1 0 R} \
	-when "C*!M1*M2*P1*!R" \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 1 0 R} \
	-when "C*M1*!M2*P1*!R" \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 1 0 R} \
	-when "C*M1*M2*P1*!R" \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 0 R F} \
	-when "!C*!M1*M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 0 R F} \
	-when "!C*!M1*M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 0 R F} \
	-when "!C*!M1*M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 0 R F} \
	-when "!C*!M1*M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 0 R F} \
	-when "!C*M1*!M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 0 R F} \
	-when "!C*M1*!M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 0 R F} \
	-when "!C*M1*!M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 0 R F} \
	-when "!C*M1*!M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 0 R F} \
	-when "!C*M1*M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 0 R F} \
	-when "!C*M1*M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 0 R F} \
	-when "!C*M1*M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 0 R F} \
	-when "!C*M1*M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110 000010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 1 R F} \
	-when "C*!M1*!M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 1 R F} \
	-when "C*!M1*!M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 001010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 1 R F} \
	-when "C*!M1*!M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 1 R F} \
	-when "C*!M1*!M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 1 R F} \
	-when "C*!M1*M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 1 R F} \
	-when "C*!M1*M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 1 R F} \
	-when "C*!M1*M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 1 R F} \
	-when "C*!M1*M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 1 R F} \
	-when "C*M1*!M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 1 R F} \
	-when "C*M1*!M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 1 R F} \
	-when "C*M1*!M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 1 R F} \
	-when "C*M1*!M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 1 R F} \
	-when "C*M1*M2*!P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 1 R F} \
	-when "C*M1*M2*!P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 1 R F} \
	-when "C*M1*M2*P1*!P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 1 R F} \
	-when "C*M1*M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 1 F R} \
	-when "C*!M1*!M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 1 F R} \
	-when "C*!M1*M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 1 F R} \
	-when "C*M1*!M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 1 F R} \
	-when "C*M1*M2*P1*P2" \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 F 0 0} \
	-when "!M1*!M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 F 1 0} \
	-when "!M1*!M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 F 0 0} \
	-when "!M1*!M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 F 1 0} \
	-when "!M1*!M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 F 0 0} \
	-when "!M1*!M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 F 1 0} \
	-when "!M1*!M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 F 1 0} \
	-when "!M1*!M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 F 0 0} \
	-when "!M1*M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 F 1 0} \
	-when "!M1*M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 F 0 1} \
	-when "!M1*M2*!P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 F 0 0} \
	-when "!M1*M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 F 1 0} \
	-when "!M1*M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 F 0 1} \
	-when "!M1*M2*!P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 F 0 0} \
	-when "!M1*M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 F 1 0} \
	-when "!M1*M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 F 0 1} \
	-when "!M1*M2*P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 F 1 0} \
	-when "!M1*M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 F 0 1} \
	-when "!M1*M2*P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 F 0 0} \
	-when "M1*!M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 F 1 0} \
	-when "M1*!M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 F 0 1} \
	-when "M1*!M2*!P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 F 0 0} \
	-when "M1*!M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 F 1 0} \
	-when "M1*!M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 F 0 1} \
	-when "M1*!M2*!P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 F 0 0} \
	-when "M1*!M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 F 1 0} \
	-when "M1*!M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 F 0 1} \
	-when "M1*!M2*P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 F 1 0} \
	-when "M1*!M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 F 0 1} \
	-when "M1*!M2*P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 F 0 0} \
	-when "M1*M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 F 1 0} \
	-when "M1*M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 F 0 1} \
	-when "M1*M2*!P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 F 0 0} \
	-when "M1*M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 F 1 0} \
	-when "M1*M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 F 0 1} \
	-when "M1*M2*!P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 F 0 0} \
	-when "M1*M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 F 1 0} \
	-when "M1*M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 F 0 1} \
	-when "M1*M2*P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 F 1 0} \
	-when "M1*M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 F 0 1} \
	-when "M1*M2*P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 R 0 0} \
	-when "!M1*!M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 R 1 0} \
	-when "!M1*!M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 R 0 0} \
	-when "!M1*!M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 R 1 0} \
	-when "!M1*!M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 R 0 0} \
	-when "!M1*!M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 R 1 0} \
	-when "!M1*!M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 R 1 0} \
	-when "!M1*!M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 R 0 0} \
	-when "!M1*M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 R 1 0} \
	-when "!M1*M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 R 0 1} \
	-when "!M1*M2*!P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 R 0 0} \
	-when "!M1*M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 R 1 0} \
	-when "!M1*M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 R 0 1} \
	-when "!M1*M2*!P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 R 0 0} \
	-when "!M1*M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 R 1 0} \
	-when "!M1*M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 R 0 1} \
	-when "!M1*M2*P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 R 1 0} \
	-when "!M1*M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 R 0 1} \
	-when "!M1*M2*P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 R 0 0} \
	-when "M1*!M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 R 1 0} \
	-when "M1*!M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 R 0 1} \
	-when "M1*!M2*!P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 R 0 0} \
	-when "M1*!M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 R 1 0} \
	-when "M1*!M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 R 0 1} \
	-when "M1*!M2*!P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 R 0 0} \
	-when "M1*!M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 R 1 0} \
	-when "M1*!M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 R 0 1} \
	-when "M1*!M2*P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 R 1 0} \
	-when "M1*!M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 R 0 1} \
	-when "M1*!M2*P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 R 0 0} \
	-when "M1*M2*!P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 R 1 0} \
	-when "M1*M2*!P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 R 0 1} \
	-when "M1*M2*!P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 R 0 0} \
	-when "M1*M2*!P1*P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 R 1 0} \
	-when "M1*M2*!P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 R 0 1} \
	-when "M1*M2*!P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 R 0 0} \
	-when "M1*M2*P1*!P2*!Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 R 1 0} \
	-when "M1*M2*P1*!P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 R 0 1} \
	-when "M1*M2*P1*!P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 R 1 0} \
	-when "M1*M2*P1*P2*!Q*R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 R 0 1} \
	-when "M1*M2*P1*P2*Q*!R" \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 0 0 0} \
	-when "!C*!M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 0 1 0} \
	-when "!C*!M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 0 0 0} \
	-when "!C*!M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 0 1 0} \
	-when "!C*!M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 0 0 0} \
	-when "!C*!M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 0 1 0} \
	-when "!C*!M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 1 0 0 0} \
	-when "!C*!M2*P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 1 0 1 0} \
	-when "!C*!M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 0 0 0 0} \
	-when "!C*M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 0 0 1 0} \
	-when "!C*M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 0 0 0 1} \
	-when "!C*M2*!P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 1 0 0 0} \
	-when "!C*M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 1 0 1 0} \
	-when "!C*M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 1 0 0 1} \
	-when "!C*M2*!P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 0 0 0 0} \
	-when "!C*M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 0 0 1 0} \
	-when "!C*M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 0 0 0 1} \
	-when "!C*M2*P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 1 0 0 0} \
	-when "!C*M2*P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 1 0 1 0} \
	-when "!C*M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 1 0 0 1} \
	-when "!C*M2*P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 1 0 0} \
	-when "C*!M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 1 1 0} \
	-when "C*!M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 1 0 1} \
	-when "C*!M2*!P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 1 0 0} \
	-when "C*!M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 1 1 0} \
	-when "C*!M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 1 1 0 1} \
	-when "C*!M2*!P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 1 0 0} \
	-when "C*!M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 1 1 0} \
	-when "C*!M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 0 1 0 1} \
	-when "C*!M2*P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 1 1 1 0} \
	-when "C*!M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 1 1 1 0 1} \
	-when "C*!M2*P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 0 1 0 0} \
	-when "C*M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 0 1 1 0} \
	-when "C*M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 0 1 0 1} \
	-when "C*M2*!P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 1 1 0 0} \
	-when "C*M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 1 1 1 0} \
	-when "C*M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 0 1 1 0 1} \
	-when "C*M2*!P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 0 1 0 0} \
	-when "C*M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 0 1 1 0} \
	-when "C*M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 0 1 0 1} \
	-when "C*M2*P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 1 1 1 0} \
	-when "C*M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 1 1 1 1 0 1} \
	-when "C*M2*P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 0 0 0 0} \
	-when "!C*!M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 0 0 1 0} \
	-when "!C*!M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 1 0 0 0} \
	-when "!C*!M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 1 0 1 0} \
	-when "!C*!M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 0 0 0 0} \
	-when "!C*!M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 0 0 1 0} \
	-when "!C*!M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 1 0 0 0} \
	-when "!C*!M2*P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 1 0 1 0} \
	-when "!C*!M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 0 0 0 0} \
	-when "!C*M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 0 0 1 0} \
	-when "!C*M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 0 0 0 1} \
	-when "!C*M2*!P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 1 0 0 0} \
	-when "!C*M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 1 0 1 0} \
	-when "!C*M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 1 0 0 1} \
	-when "!C*M2*!P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 0 0 0 0} \
	-when "!C*M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 0 0 1 0} \
	-when "!C*M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 0 0 0 1} \
	-when "!C*M2*P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 1 0 0 0} \
	-when "!C*M2*P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 1 0 1 0} \
	-when "!C*M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 1 0 0 1} \
	-when "!C*M2*P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 0 1 0 0} \
	-when "C*!M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 0 1 1 0} \
	-when "C*!M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 0 1 0 1} \
	-when "C*!M2*!P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 1 1 0 0} \
	-when "C*!M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 1 1 1 0} \
	-when "C*!M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 1 1 0 1} \
	-when "C*!M2*!P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 0 1 0 0} \
	-when "C*!M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 0 1 1 0} \
	-when "C*!M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 0 1 0 1} \
	-when "C*!M2*P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 1 1 1 0} \
	-when "C*!M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 1 1 1 0 1} \
	-when "C*!M2*P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 0 1 0 0} \
	-when "C*M2*!P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 0 1 1 0} \
	-when "C*M2*!P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 0 1 0 1} \
	-when "C*M2*!P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 1 1 0 0} \
	-when "C*M2*!P1*P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 1 1 1 0} \
	-when "C*M2*!P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 0 1 1 0 1} \
	-when "C*M2*!P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 0 1 0 0} \
	-when "C*M2*P1*!P2*!Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 0 1 1 0} \
	-when "C*M2*P1*!P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 0 1 0 1} \
	-when "C*M2*P1*!P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 1 1 1 0} \
	-when "C*M2*P1*P2*!Q*R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 1 1 1 1 0 1} \
	-when "C*M2*P1*P2*Q*!R" \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 0 0 0} \
	-when "!C*!M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 0 1 0} \
	-when "!C*!M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 0 0 0} \
	-when "!C*!M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 0 1 0} \
	-when "!C*!M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 0 0 0} \
	-when "!C*!M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 0 1 0} \
	-when "!C*!M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 0 0 0} \
	-when "!C*!M1*P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 0 1 0} \
	-when "!C*!M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 0 0 0 0} \
	-when "!C*M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 0 0 1 0} \
	-when "!C*M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 0 0 0 1} \
	-when "!C*M1*!P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 1 0 0 0} \
	-when "!C*M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 1 0 1 0} \
	-when "!C*M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 1 0 0 1} \
	-when "!C*M1*!P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 0 0 0 0} \
	-when "!C*M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 0 0 1 0} \
	-when "!C*M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 0 0 0 1} \
	-when "!C*M1*P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 1 0 0 0} \
	-when "!C*M1*P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 1 0 1 0} \
	-when "!C*M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 1 0 0 1} \
	-when "!C*M1*P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 1 0 0} \
	-when "C*!M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 1 1 0} \
	-when "C*!M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 1 0 1} \
	-when "C*!M1*!P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 1 0 0} \
	-when "C*!M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 1 1 0} \
	-when "C*!M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 1 1 0 1} \
	-when "C*!M1*!P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 1 0 0} \
	-when "C*!M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 1 1 0} \
	-when "C*!M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 0 1 0 1} \
	-when "C*!M1*P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 1 1 0} \
	-when "C*!M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 1 0 1} \
	-when "C*!M1*P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 0 1 0 0} \
	-when "C*M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 0 1 1 0} \
	-when "C*M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 0 1 0 1} \
	-when "C*M1*!P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 1 1 0 0} \
	-when "C*M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 1 1 1 0} \
	-when "C*M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 0 1 1 0 1} \
	-when "C*M1*!P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 0 1 0 0} \
	-when "C*M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 0 1 1 0} \
	-when "C*M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 0 1 0 1} \
	-when "C*M1*P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 1 1 1 0} \
	-when "C*M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 F 1 1 1 0 1} \
	-when "C*M1*P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 0 0 0 0} \
	-when "!C*!M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 0 0 1 0} \
	-when "!C*!M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 1 0 0 0} \
	-when "!C*!M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 1 0 1 0} \
	-when "!C*!M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 0 0 0 0} \
	-when "!C*!M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 0 0 1 0} \
	-when "!C*!M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 1 0 0 0} \
	-when "!C*!M1*P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 1 0 1 0} \
	-when "!C*!M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 0 0 0 0} \
	-when "!C*M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 0 0 1 0} \
	-when "!C*M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 0 0 0 1} \
	-when "!C*M1*!P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 1 0 0 0} \
	-when "!C*M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 1 0 1 0} \
	-when "!C*M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 1 0 0 1} \
	-when "!C*M1*!P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 0 0 0 0} \
	-when "!C*M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 0 0 1 0} \
	-when "!C*M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 0 0 0 1} \
	-when "!C*M1*P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 1 0 0 0} \
	-when "!C*M1*P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 1 0 1 0} \
	-when "!C*M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 1 0 0 1} \
	-when "!C*M1*P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 0 1 0 0} \
	-when "C*!M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 0 1 1 0} \
	-when "C*!M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 0 1 0 1} \
	-when "C*!M1*!P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 1 1 0 0} \
	-when "C*!M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 1 1 1 0} \
	-when "C*!M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 1 1 0 1} \
	-when "C*!M1*!P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 0 1 0 0} \
	-when "C*!M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 0 1 1 0} \
	-when "C*!M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 0 1 0 1} \
	-when "C*!M1*P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 1 1 1 0} \
	-when "C*!M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 1 1 1 0 1} \
	-when "C*!M1*P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 0 1 0 0} \
	-when "C*M1*!P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 0 1 1 0} \
	-when "C*M1*!P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 0 1 0 1} \
	-when "C*M1*!P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 1 1 0 0} \
	-when "C*M1*!P1*P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 1 1 1 0} \
	-when "C*M1*!P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 0 1 1 0 1} \
	-when "C*M1*!P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 0 1 0 0} \
	-when "C*M1*P1*!P2*!Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 0 1 1 0} \
	-when "C*M1*P1*!P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 0 1 0 1} \
	-when "C*M1*P1*!P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 1 1 1 0} \
	-when "C*M1*P1*P2*!Q*R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 R 1 1 1 0 1} \
	-when "C*M1*P1*P2*Q*!R" \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 0 0 0 0} \
	-when "!C*!M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 0 0 1 0} \
	-when "!C*!M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 1 0 0 0} \
	-when "!C*!M1*!M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 1 0 1 0} \
	-when "!C*!M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 0 0 0 0} \
	-when "!C*!M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 0 0 1 0} \
	-when "!C*!M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 0 0 0 1} \
	-when "!C*!M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 0 0 0} \
	-when "!C*!M1*M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 0 1 0} \
	-when "!C*!M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 0 0 1} \
	-when "!C*!M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 0 0 0 0} \
	-when "!C*M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 0 0 1 0} \
	-when "!C*M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 0 0 0 1} \
	-when "!C*M1*!M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 1 0 0 0} \
	-when "!C*M1*!M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 1 0 1 0} \
	-when "!C*M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 1 0 0 1} \
	-when "!C*M1*!M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 0 0 0 0} \
	-when "!C*M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 0 0 1 0} \
	-when "!C*M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 0 0 0 1} \
	-when "!C*M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 1 0 0 0} \
	-when "!C*M1*M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 1 0 1 0} \
	-when "!C*M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 1 0 0 1} \
	-when "!C*M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 0 1 0 0} \
	-when "C*!M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 0 1 1 0} \
	-when "C*!M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 0 1 0 1} \
	-when "C*!M1*!M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 1 1 1 0} \
	-when "C*!M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 1 1 0 1} \
	-when "C*!M1*!M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 0 1 0 0} \
	-when "C*!M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 0 1 1 0} \
	-when "C*!M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 0 1 0 1} \
	-when "C*!M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 1 1 0} \
	-when "C*!M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 F 1 1 0 1} \
	-when "C*!M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 0 1 0 0} \
	-when "C*M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 0 1 1 0} \
	-when "C*M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 0 1 0 1} \
	-when "C*M1*!M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 1 1 1 0} \
	-when "C*M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 F 1 1 0 1} \
	-when "C*M1*!M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 0 1 0 0} \
	-when "C*M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 0 1 1 0} \
	-when "C*M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 0 1 0 1} \
	-when "C*M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 1 1 1 0} \
	-when "C*M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 F 1 1 0 1} \
	-when "C*M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 0 0 0 0} \
	-when "!C*!M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 0 0 1 0} \
	-when "!C*!M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 0 0 0} \
	-when "!C*!M1*!M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 0 1 0} \
	-when "!C*!M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 0 0 0 0} \
	-when "!C*!M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 0 0 1 0} \
	-when "!C*!M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 0 0 0 1} \
	-when "!C*!M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 0 0 0} \
	-when "!C*!M1*M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 0 1 0} \
	-when "!C*!M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 0 0 1} \
	-when "!C*!M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 0 0 0 0} \
	-when "!C*M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 0 0 1 0} \
	-when "!C*M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 0 0 0 1} \
	-when "!C*M1*!M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 0 0 0} \
	-when "!C*M1*!M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 0 1 0} \
	-when "!C*M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 0 0 1} \
	-when "!C*M1*!M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 0 0 0 0} \
	-when "!C*M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 0 0 1 0} \
	-when "!C*M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 0 0 0 1} \
	-when "!C*M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 0 0 0} \
	-when "!C*M1*M2*P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 0 1 0} \
	-when "!C*M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 0 0 1} \
	-when "!C*M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 0 1 0 0} \
	-when "C*!M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 0 1 1 0} \
	-when "C*!M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 0 1 0 1} \
	-when "C*!M1*!M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 1 1 0} \
	-when "C*!M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 1 0 1} \
	-when "C*!M1*!M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 0 1 0 0} \
	-when "C*!M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 0 1 1 0} \
	-when "C*!M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 0 1 0 1} \
	-when "C*!M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 1 1 0} \
	-when "C*!M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 R 1 1 0 1} \
	-when "C*!M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 0 1 0 0} \
	-when "C*M1*!M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 0 1 1 0} \
	-when "C*M1*!M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 0 1 0 1} \
	-when "C*M1*!M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 1 1 0} \
	-when "C*M1*!M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 R 1 1 0 1} \
	-when "C*M1*!M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 0 1 0 0} \
	-when "C*M1*M2*!P2*!Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 0 1 1 0} \
	-when "C*M1*M2*!P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 0 1 0 1} \
	-when "C*M1*M2*!P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 1 1 0} \
	-when "C*M1*M2*P2*!Q*R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 R 1 1 0 1} \
	-when "C*M1*M2*P2*Q*!R" \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 F 0 0 0} \
	-when "!C*!M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 F 0 1 0} \
	-when "!C*!M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 F 0 0 0} \
	-when "!C*!M1*!M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 F 0 1 0} \
	-when "!C*!M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 F 0 0 0} \
	-when "!C*!M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 F 0 1 0} \
	-when "!C*!M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 F 0 0 1} \
	-when "!C*!M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 0 0 0} \
	-when "!C*!M1*M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 0 1 0} \
	-when "!C*!M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 0 0 1} \
	-when "!C*!M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 F 0 0 0} \
	-when "!C*M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 F 0 1 0} \
	-when "!C*M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 F 0 0 1} \
	-when "!C*M1*!M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 F 0 0 0} \
	-when "!C*M1*!M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 F 0 1 0} \
	-when "!C*M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 F 0 0 1} \
	-when "!C*M1*!M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 F 0 0 0} \
	-when "!C*M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 F 0 1 0} \
	-when "!C*M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 F 0 0 1} \
	-when "!C*M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 F 0 0 0} \
	-when "!C*M1*M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 F 0 1 0} \
	-when "!C*M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 F 0 0 1} \
	-when "!C*M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 F 1 0 0} \
	-when "C*!M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 F 1 1 0} \
	-when "C*!M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 F 1 0 1} \
	-when "C*!M1*!M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 F 1 1 0} \
	-when "C*!M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 F 1 0 1} \
	-when "C*!M1*!M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 F 1 0 0} \
	-when "C*!M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 F 1 1 0} \
	-when "C*!M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 F 1 0 1} \
	-when "C*!M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 1 1 0} \
	-when "C*!M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 F 1 0 1} \
	-when "C*!M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 F 1 0 0} \
	-when "C*M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 F 1 1 0} \
	-when "C*M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 F 1 0 1} \
	-when "C*M1*!M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 F 1 1 0} \
	-when "C*M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 F 1 0 1} \
	-when "C*M1*!M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 F 1 0 0} \
	-when "C*M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 F 1 1 0} \
	-when "C*M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 F 1 0 1} \
	-when "C*M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 F 1 1 0} \
	-when "C*M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 F 1 0 1} \
	-when "C*M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 R 0 0 0} \
	-when "!C*!M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 R 0 1 0} \
	-when "!C*!M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 0 0 0} \
	-when "!C*!M1*!M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 0 1 0} \
	-when "!C*!M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 R 0 0 0} \
	-when "!C*!M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 R 0 1 0} \
	-when "!C*!M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 R 0 0 1} \
	-when "!C*!M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 0 0 0} \
	-when "!C*!M1*M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 0 1 0} \
	-when "!C*!M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 0 0 1} \
	-when "!C*!M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 R 0 0 0} \
	-when "!C*M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 R 0 1 0} \
	-when "!C*M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 R 0 0 1} \
	-when "!C*M1*!M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 0 0 0} \
	-when "!C*M1*!M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 0 1 0} \
	-when "!C*M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 0 0 1} \
	-when "!C*M1*!M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 R 0 0 0} \
	-when "!C*M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 R 0 1 0} \
	-when "!C*M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 R 0 0 1} \
	-when "!C*M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 0 0 0} \
	-when "!C*M1*M2*P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 0 1 0} \
	-when "!C*M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 0 0 1} \
	-when "!C*M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 R 1 0 0} \
	-when "C*!M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 R 1 1 0} \
	-when "C*!M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 000110 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 R 1 0 1} \
	-when "C*!M1*!M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 1 1 0} \
	-when "C*!M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 1 0 1} \
	-when "C*!M1*!M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 R 1 0 0} \
	-when "C*!M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 R 1 1 0} \
	-when "C*!M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 010110 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 R 1 0 1} \
	-when "C*!M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 1 1 0} \
	-when "C*!M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011110 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 R 1 0 1} \
	-when "C*!M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 R 1 0 0} \
	-when "C*M1*!M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 R 1 1 0} \
	-when "C*M1*!M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 100110 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 R 1 0 1} \
	-when "C*M1*!M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 1 1 0} \
	-when "C*M1*!M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101110 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 R 1 0 1} \
	-when "C*M1*!M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 R 1 0 0} \
	-when "C*M1*M2*!P1*!Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 R 1 1 0} \
	-when "C*M1*M2*!P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 110110 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 R 1 0 1} \
	-when "C*M1*M2*!P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 1 1 0} \
	-when "C*M1*M2*P1*!Q*R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111110 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 R 1 0 1} \
	-when "C*M1*M2*P1*Q*!R" \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 0 F 0} \
	-when "!C*!M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 0 F 0} \
	-when "!C*!M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 0 F 0} \
	-when "!C*!M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 0 F 0} \
	-when "!C*!M1*!M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 0 F 0} \
	-when "!C*!M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 0 F 0} \
	-when "!C*!M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 0 F 0} \
	-when "!C*!M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 0 F 0} \
	-when "!C*!M1*M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 0 F 0} \
	-when "!C*M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 0 F 0} \
	-when "!C*M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 0 F 0} \
	-when "!C*M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 0 F 0} \
	-when "!C*M1*!M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 0 F 0} \
	-when "!C*M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 0 F 0} \
	-when "!C*M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 0 F 0} \
	-when "!C*M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 0 F 0} \
	-when "!C*M1*M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 1 F 0} \
	-when "C*!M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 1 F 0} \
	-when "C*!M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 1 F 0} \
	-when "C*!M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 1 F 0} \
	-when "C*!M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 1 F 0} \
	-when "C*!M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 1 F 0} \
	-when "C*!M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 1 F 0} \
	-when "C*M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 1 F 0} \
	-when "C*M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 1 F 0} \
	-when "C*M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 1 F 0} \
	-when "C*M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 1 F 0} \
	-when "C*M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 1 F 0} \
	-when "C*M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 0 R 0} \
	-when "!C*!M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 0 R 0} \
	-when "!C*!M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 0 R 0} \
	-when "!C*!M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 0 R 0} \
	-when "!C*!M1*!M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 010000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 0 R 0} \
	-when "!C*!M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 010100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 0 R 0} \
	-when "!C*!M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 011000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 0 R 0} \
	-when "!C*!M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 011100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 0 R 0} \
	-when "!C*!M1*M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 100000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 0 R 0} \
	-when "!C*M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 100100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 0 R 0} \
	-when "!C*M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 101000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 0 R 0} \
	-when "!C*M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001100 101100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 0 R 0} \
	-when "!C*M1*!M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110001 110000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 0 R 0} \
	-when "!C*M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110101 110100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 0 R 0} \
	-when "!C*M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111001 111000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 0 R 0} \
	-when "!C*M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111101 111100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 0 R 0} \
	-when "!C*M1*M2*P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000 000010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 1 R 0} \
	-when "C*!M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 1 R 0} \
	-when "C*!M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 1 R 0} \
	-when "C*!M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010011 010010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 1 R 0} \
	-when "C*!M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010111 010110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 1 R 0} \
	-when "C*!M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {011011 011010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 1 R 0} \
	-when "C*!M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100011 100010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 1 R 0} \
	-when "C*M1*!M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100111 100110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 1 R 0} \
	-when "C*M1*!M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {101011 101010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 1 R 0} \
	-when "C*M1*!M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110011 110010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 1 R 0} \
	-when "C*M1*M2*!P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {110111 110110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 1 R 0} \
	-when "C*M1*M2*!P1*P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {111011 111010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 1 R 0} \
	-when "C*M1*M2*P1*!P2*!Q" \
	-pin R \
	{ RACELEM21 }

define_leakage -when "!C*!M1*!M2*!P1*!P2*!Q*!R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*!P1*P2*!Q*!R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*P1*!P2*!Q*!R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*P1*P2*!Q*!R" { RACELEM21 }
define_leakage -when "!C*!M1*!M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*!M1*M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*!M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*!M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*!M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*!M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "!C*M1*M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*!M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*!M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*!M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*!M2*P1*P2*Q*!R" { RACELEM21 }
define_leakage -when "C*!M1*!M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*!M1*M2*P1*P2*Q*!R" { RACELEM21 }
define_leakage -when "C*!M1*M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*!M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*!M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*!M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*!M2*P1*P2*Q*!R" { RACELEM21 }
define_leakage -when "C*M1*!M2*P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*M2*!P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*M2*!P1*P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*M2*P1*!P2*!Q*R" { RACELEM21 }
define_leakage -when "C*M1*M2*P1*P2*Q*!R" { RACELEM21 }
define_leakage -when "C*M1*M2*P1*P2*!Q*R" { RACELEM21 }
# oscillation: !S*!R risks metastability in {Q, Qn}, settling to one of {Q=0, Qn=1} | {Q=1, Qn=0}
define_arc \
	-prevector_pinlist {S R} \
	-prevector {10 00} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {0 R F X} \
	-when "!S" \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {10} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {1 R F X} \
	-when "S" \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-pinlist {S R Q Qn} \
	-vector {1 F R X} \
	-when "S" \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {01 00} \
	-pinlist {S R Q Qn} \
	-vector {R 0 R X} \
	-when "!R" \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {10 00} \
	-pinlist {S R Q Qn} \
	-vector {0 R X R} \
	-when "!S" \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {01 00} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {R 0 X F} \
	-when "!R" \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {01} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {R 1 X F} \
	-when "R" \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-pinlist {S R Q Qn} \
	-vector {F 1 X R} \
	-when "R" \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {01} \
	-pinlist {S R Q Qn} \
	-vector {0 F 0 1} \
	-when "!Q*Qn*!S" \
	-pin R \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {01 00} \
	-pinlist {S R Q Qn} \
	-vector {0 R 0 1} \
	-when "!Q*Qn*!S" \
	-pin R \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {10} \
	-pinlist {S R Q Qn} \
	-vector {F 0 1 0} \
	-when "Q*!Qn*!R" \
	-pin S \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {10 00} \
	-pinlist {S R Q Qn} \
	-vector {R 0 1 0} \
	-when "Q*!Qn*!R" \
	-pin S \
	{ SR }

define_leakage -when "Q*!Qn*!R*S" { SR }
define_leakage -when "!Q*Qn*R*!S" { SR }
define_leakage -when "!Q*!Qn*R*S" { SR }
# oscillation: A*B risks metastability in {Qa, Qb}, settling to one of {Qa=0, Qb=1} | {Qa=1, Qb=0}
define_arc \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {F 0 F X} \
	-when "!B" \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {10 11} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {F 1 F X} \
	-when "B" \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {R 0 R X} \
	-when "!B" \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {01 11} \
	-pinlist {A B Qa Qb} \
	-vector {1 F R X} \
	-when "A" \
	-related_pin B \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {10 11} \
	-pinlist {A B Qa Qb} \
	-vector {F 1 X R} \
	-when "B" \
	-related_pin A \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {0 F X F} \
	-when "!A" \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {01 11} \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-vector {1 F X F} \
	-when "A" \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {0 R X R} \
	-when "!A" \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {01 11} \
	-pinlist {A B Qa Qb} \
	-vector {F 1 0 1} \
	-when "B*!Qa*Qb" \
	-pin A \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Qa Qb} \
	-vector {R 1 0 1} \
	-when "B*!Qa*Qb" \
	-pin A \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {10 11} \
	-pinlist {A B Qa Qb} \
	-vector {1 F 1 0} \
	-when "A*Qa*!Qb" \
	-pin B \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Qa Qb} \
	-vector {1 R 1 0} \
	-when "A*Qa*!Qb" \
	-pin B \
	{ MUT }

define_leakage -when "!A*!B*!Qa*!Qb" { MUT }
define_leakage -when "!A*B*!Qa*Qb" { MUT }
define_leakage -when "A*!B*Qa*!Qb" { MUT }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 0} \
	-when "D*!Q" \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ DFF }

define_leakage -when "!CLK*!D" { DFF }
define_leakage -when "!CLK*D" { DFF }
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

define_arc \
	-prevector_pinlist {G D} \
	-prevector {11} \
	-type combinational \
	-pinlist {G D Q} \
	-vector {1 F F} \
	-when "G" \
	-related_pin D \
	-pin Q \
	{ DLH }

define_arc \
	-type combinational \
	-prevector_pinlist {G D} \
	-prevector {10} \
	-pinlist {G D Q} \
	-vector {1 R R} \
	-when "G" \
	-related_pin D \
	-pin Q \
	{ DLH }

define_arc \
	-prevector_pinlist {G D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {G D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin G \
	-pin Q \
	{ DLH }

define_arc \
	-type edge \
	-prevector_pinlist {G D} \
	-prevector {10 00 01} \
	-pinlist {G D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin G \
	-pin Q \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10 00 01} \
	-pinlist {G D Q} \
	-vector {0 F 0} \
	-when "!G*!Q" \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {11 01} \
	-pinlist {G D Q} \
	-vector {0 F 1} \
	-when "!G*Q" \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10 00} \
	-pinlist {G D Q} \
	-vector {0 R 0} \
	-when "!G*!Q" \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {11 01 00} \
	-pinlist {G D Q} \
	-vector {0 R 1} \
	-when "!G*Q" \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10} \
	-pinlist {G D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin G \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {11} \
	-pinlist {G D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin G \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10 00} \
	-pinlist {G D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin G \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {11 01} \
	-pinlist {G D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin G \
	{ DLH }

define_leakage -when "!D*G*!Q" { DLH }
define_leakage -when "D*G*Q" { DLH }
define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 F} \
	-when "!CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 F} \
	-when "!CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 F} \
	-when "!CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 F} \
	-when "!CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 F} \
	-when "!CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 F} \
	-when "!CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 F} \
	-when "!CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 F} \
	-when "!CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 0 F} \
	-when "!CLKB*!RA*RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 0 F} \
	-when "!CLKB*!RA*RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 1 F} \
	-when "!CLKB*!RA*RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 1 F} \
	-when "!CLKB*!RA*RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 F} \
	-when "CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 F} \
	-when "CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 F} \
	-when "CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 F} \
	-when "CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 F} \
	-when "CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 F} \
	-when "CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 F} \
	-when "CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 F} \
	-when "CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011 11010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 0 F} \
	-when "CLKB*!RA*RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 0 F} \
	-when "CLKB*!RA*RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 1 F} \
	-when "CLKB*!RA*RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 1 F} \
	-when "CLKB*!RA*RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 0 R} \
	-when "!CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 0 R} \
	-when "!CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 1 R} \
	-when "!CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 1 R} \
	-when "!CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 0 R} \
	-when "!CLKB*!RA*RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 1 R} \
	-when "!CLKB*!RA*RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 R} \
	-when "CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 R} \
	-when "CLKB*!RA*!RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 R} \
	-when "CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 R} \
	-when "CLKB*!RA*!RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 1 0 R} \
	-when "CLKB*!RA*RB*!S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 1 1 R} \
	-when "CLKB*!RA*RB*S" \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 F} \
	-when "!CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 F} \
	-when "!CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 F} \
	-when "!CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 F} \
	-when "!CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 F} \
	-when "!CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 F} \
	-when "!CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 F} \
	-when "!CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 F} \
	-when "!CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 0 F} \
	-when "!CLKA*RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 0 F} \
	-when "!CLKA*RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100 01101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 1 F} \
	-when "!CLKA*RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 1 F} \
	-when "!CLKA*RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 F} \
	-when "CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 F} \
	-when "CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 F} \
	-when "CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 F} \
	-when "CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 F} \
	-when "CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 F} \
	-when "CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 F} \
	-when "CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 F} \
	-when "CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 0 F} \
	-when "CLKA*RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 0 F} \
	-when "CLKA*RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 1 F} \
	-when "CLKA*RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 1 F} \
	-when "CLKA*RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 0 R} \
	-when "!CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 0 R} \
	-when "!CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 1 R} \
	-when "!CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 1 R} \
	-when "!CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 0 R} \
	-when "!CLKA*RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 1 R} \
	-when "!CLKA*RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 R} \
	-when "CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 R} \
	-when "CLKA*!RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 R} \
	-when "CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 R} \
	-when "CLKA*!RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 0 0 R} \
	-when "CLKA*RA*!RB*!S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 0 1 R} \
	-when "CLKA*RA*!RB*S" \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 F} \
	-when "CLKA*!CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 F} \
	-when "CLKA*!CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 F} \
	-when "CLKA*!CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 F} \
	-when "CLKA*!CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 F} \
	-when "CLKA*!CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 F} \
	-when "CLKA*!CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 F} \
	-when "CLKA*!CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 10001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 F} \
	-when "CLKA*!CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 0 F} \
	-when "CLKA*!CLKB*RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 0 F} \
	-when "CLKA*!CLKB*RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 1 F} \
	-when "CLKA*!CLKB*RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 1 F} \
	-when "CLKA*!CLKB*RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 F} \
	-when "CLKA*CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 F} \
	-when "CLKA*CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 F} \
	-when "CLKA*CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 F} \
	-when "CLKA*CLKB*!RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 F} \
	-when "CLKA*CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 F} \
	-when "CLKA*CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 F} \
	-when "CLKA*CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 F} \
	-when "CLKA*CLKB*!RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011 11010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 0 F} \
	-when "CLKA*CLKB*RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 0 F} \
	-when "CLKA*CLKB*RB*!S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 1 F} \
	-when "CLKA*CLKB*RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 1 F} \
	-when "CLKA*CLKB*RB*S" \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 F} \
	-when "!CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 F} \
	-when "!CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 F} \
	-when "!CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 01000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 F} \
	-when "!CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 F} \
	-when "!CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 F} \
	-when "!CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 F} \
	-when "!CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 F} \
	-when "!CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 0 F} \
	-when "!CLKA*CLKB*RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 0 F} \
	-when "!CLKA*CLKB*RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100 01101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 F} \
	-when "!CLKA*CLKB*RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 F} \
	-when "!CLKA*CLKB*RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 F} \
	-when "CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 F} \
	-when "CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 F} \
	-when "CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 11000} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 F} \
	-when "CLKA*CLKB*!RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 F} \
	-when "CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 F} \
	-when "CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 F} \
	-when "CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 F} \
	-when "CLKA*CLKB*!RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 0 F} \
	-when "CLKA*CLKB*RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 0 F} \
	-when "CLKA*CLKB*RA*!S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 1 F} \
	-when "CLKA*CLKB*RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 1 F} \
	-when "CLKA*CLKB*RA*S" \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 0} \
	-when "!CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 0} \
	-when "!CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 0} \
	-when "!CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 1 0} \
	-when "!CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 0 0} \
	-when "!CLKB*!GCLK*!RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 0 0} \
	-when "!CLKB*!GCLK*!RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 1 0} \
	-when "!CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 1 0} \
	-when "!CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 0 0 0} \
	-when "!CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 0 0 0} \
	-when "!CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 0 1 0} \
	-when "!CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 0 1 0} \
	-when "!CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 1 0 0} \
	-when "!CLKB*!GCLK*RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 1 1 0} \
	-when "!CLKB*!GCLK*RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 0} \
	-when "CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 0} \
	-when "CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 0} \
	-when "CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 0} \
	-when "CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 0} \
	-when "CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 0} \
	-when "CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 0} \
	-when "CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 0} \
	-when "CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 0 0} \
	-when "CLKB*!GCLK*!RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 0 0} \
	-when "CLKB*!GCLK*!RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 1 0} \
	-when "CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 0 1 1 0} \
	-when "CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 0 0} \
	-when "CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 0 0} \
	-when "CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 0 1} \
	-when "CLKB*GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 0 1} \
	-when "CLKB*GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 1 0} \
	-when "CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 1 0} \
	-when "CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 1 1} \
	-when "CLKB*GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 0 1 1} \
	-when "CLKB*GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 1 0 0} \
	-when "CLKB*!GCLK*RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 1 1 1 1 0} \
	-when "CLKB*!GCLK*RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 1 0} \
	-when "!CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 1 0} \
	-when "!CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 0 0} \
	-when "!CLKB*!GCLK*!RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 1 0} \
	-when "!CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 0 0 0} \
	-when "!CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 0 0 0} \
	-when "!CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 0 1 0} \
	-when "!CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 0 1 0} \
	-when "!CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 1 0 0} \
	-when "!CLKB*!GCLK*RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 1 1 0} \
	-when "!CLKB*!GCLK*RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 0} \
	-when "CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 0} \
	-when "CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 0 1} \
	-when "CLKB*GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 0} \
	-when "CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 0} \
	-when "CLKB*!GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 0 1 1} \
	-when "CLKB*GCLK*!RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 1 0 0} \
	-when "CLKB*!GCLK*!RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 0 1 1 0} \
	-when "CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 0 0} \
	-when "CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 0 0} \
	-when "CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 0 1} \
	-when "CLKB*GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 0 1} \
	-when "CLKB*GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 1 0} \
	-when "CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 1 0} \
	-when "CLKB*!GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 1 1} \
	-when "CLKB*GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 0 1 1} \
	-when "CLKB*GCLK*RA*!RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 1 0 0} \
	-when "CLKB*!GCLK*RA*RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 1 1 1 1 0} \
	-when "CLKB*!GCLK*RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 1 0 0} \
	-when "!CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 1 0 0} \
	-when "!CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 1 1 0} \
	-when "!CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 1 1 0} \
	-when "!CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 0 0} \
	-when "!CLKA*!GCLK*RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 0 0} \
	-when "!CLKA*!GCLK*RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 1 0} \
	-when "!CLKA*!GCLK*RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 0 1 0} \
	-when "!CLKA*!GCLK*RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 1 0 0} \
	-when "!CLKA*!GCLK*RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 1 1 0} \
	-when "!CLKA*!GCLK*RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 0} \
	-when "CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 0} \
	-when "CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 0} \
	-when "CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 0} \
	-when "CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 0} \
	-when "CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 0} \
	-when "CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 0} \
	-when "CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 0} \
	-when "CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 0 0} \
	-when "CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 0 0} \
	-when "CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 0 1} \
	-when "CLKA*GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 0 1} \
	-when "CLKA*GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 1 0} \
	-when "CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 1 0} \
	-when "CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 1 1} \
	-when "CLKA*GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 0 1 1 1} \
	-when "CLKA*GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 0 0} \
	-when "CLKA*!GCLK*RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 0 0} \
	-when "CLKA*!GCLK*RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 1 0} \
	-when "CLKA*!GCLK*RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 0 1 0} \
	-when "CLKA*!GCLK*RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 1 0 0} \
	-when "CLKA*!GCLK*RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 F 1 1 1 0} \
	-when "CLKA*!GCLK*RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 1 0 0} \
	-when "!CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 1 0 0} \
	-when "!CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 1 1 0} \
	-when "!CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 1 1 0} \
	-when "!CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 0 0} \
	-when "!CLKA*!GCLK*RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 1 0} \
	-when "!CLKA*!GCLK*RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 1 0 0} \
	-when "!CLKA*!GCLK*RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 1 1 0} \
	-when "!CLKA*!GCLK*RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 0} \
	-when "CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 0} \
	-when "CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 0 1} \
	-when "CLKA*GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 0} \
	-when "CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 0} \
	-when "CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 0 1 1} \
	-when "CLKA*GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 0 0} \
	-when "CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 0 0} \
	-when "CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 0 1} \
	-when "CLKA*GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 0 1} \
	-when "CLKA*GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 1 0} \
	-when "CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 1 0} \
	-when "CLKA*!GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 1 1} \
	-when "CLKA*GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 0 1 1 1} \
	-when "CLKA*GCLK*!RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 0 0 0} \
	-when "CLKA*!GCLK*RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 0 1 0} \
	-when "CLKA*!GCLK*RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 1 0 0} \
	-when "CLKA*!GCLK*RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 R 1 1 1 0} \
	-when "CLKA*!GCLK*RA*RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 F 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 1 0 0} \
	-when "CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 F 1 1 0} \
	-when "CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 0 0} \
	-when "CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 0 0} \
	-when "CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 1 0} \
	-when "CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 1 0} \
	-when "CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 F 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 F 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 F 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 F 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 F 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 F 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 F 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 F 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 F 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 F 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 F 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 F 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 F 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 F 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 F 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 F 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 F 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 F 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 R 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 1 F 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 F 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 F 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 1 F 0} \
	-when "!CLKA*CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 F 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 F 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 1 F 0} \
	-when "CLKA*!CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 F 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 F 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 F 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 F 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 F 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 F 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 F 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 1 F 0} \
	-when "CLKA*CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 1 R 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 R 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 R 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 1 R 0} \
	-when "!CLKA*CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 00001 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 R 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 R 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 1 R 0} \
	-when "CLKA*!CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10001 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 00000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10000 10001 11001 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001 01001 01000 11000 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 R 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 R 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01011 11011 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 R 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 R 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 R 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 10100 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 R 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 R 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 1 R 0} \
	-when "CLKA*CLKB*!GCLK*RA*RB" \
	-pin S \
	{ ICM }

define_leakage -when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*!RA*RB*S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*RA*!RB*S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*RA*RB*!S" { ICM }
define_leakage -when "!CLKA*!CLKB*!GCLK*RA*RB*S" { ICM }
define_leakage -when "!CLKA*CLKB*!RA*!RB*S" { ICM }
define_leakage -when "!CLKA*CLKB*!GCLK*!RA*RB*!S" { ICM }
define_leakage -when "!CLKA*CLKB*!GCLK*!RA*RB*S" { ICM }
define_leakage -when "!CLKA*CLKB*RA*!RB*!S" { ICM }
define_leakage -when "!CLKA*CLKB*RA*!RB*S" { ICM }
define_leakage -when "!CLKA*CLKB*!GCLK*RA*RB*!S" { ICM }
define_leakage -when "!CLKA*CLKB*!GCLK*RA*RB*S" { ICM }
define_leakage -when "CLKA*!CLKB*!RA*!RB*!S" { ICM }
define_leakage -when "CLKA*!CLKB*!RA*RB*!S" { ICM }
define_leakage -when "CLKA*!CLKB*!RA*RB*S" { ICM }
define_leakage -when "CLKA*!CLKB*!GCLK*RA*!RB*!S" { ICM }
define_leakage -when "CLKA*!CLKB*!GCLK*RA*!RB*S" { ICM }
define_leakage -when "CLKA*!CLKB*!GCLK*RA*RB*!S" { ICM }
define_leakage -when "CLKA*!CLKB*!GCLK*RA*RB*S" { ICM }
define_leakage -when "CLKA*CLKB*!RA*RB*!S" { ICM }
define_leakage -when "CLKA*CLKB*!RA*RB*S" { ICM }
define_leakage -when "CLKA*CLKB*RA*!RB*!S" { ICM }
define_leakage -when "CLKA*CLKB*RA*!RB*S" { ICM }
define_leakage -when "CLKA*CLKB*!GCLK*RA*RB*!S" { ICM }
define_leakage -when "CLKA*CLKB*!GCLK*RA*RB*S" { ICM }
define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {F 0 F} \
	-when "!B" \
	-related_pin A \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q} \
	-vector {R 1 R} \
	-when "B" \
	-related_pin A \
	-pin Q \
	{ C2GATE }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {0 F F} \
	-when "!A" \
	-related_pin B \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q} \
	-vector {1 R R} \
	-when "A" \
	-related_pin B \
	-pin Q \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q} \
	-vector {F 0 0} \
	-when "!B*!Q" \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {F 1 1} \
	-when "B*Q" \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {R 0 0} \
	-when "!B*!Q" \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-pinlist {A B Q} \
	-vector {R 1 1} \
	-when "B*Q" \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q} \
	-vector {0 F 0} \
	-when "!A*!Q" \
	-pin B \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {1 F 1} \
	-when "A*Q" \
	-pin B \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {0 R 0} \
	-when "!A*!Q" \
	-pin B \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-pinlist {A B Q} \
	-vector {1 R 1} \
	-when "A*Q" \
	-pin B \
	{ C2GATE }

define_leakage -when "!A*!B*!Q" { C2GATE }
define_leakage -when "A*B*Q" { C2GATE }
