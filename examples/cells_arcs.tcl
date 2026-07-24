define_arc \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-type combinational \
	-pinlist {A B Y} \
	-vector {F 1 F} \
	-related_pin A \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Y} \
	-vector {R 1 R} \
	-related_pin A \
	-pin Y \
	{ AND2 }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-type combinational \
	-pinlist {A B Y} \
	-vector {1 F F} \
	-related_pin B \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Y} \
	-vector {1 R R} \
	-related_pin B \
	-pin Y \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Y} \
	-vector {F 0 0} \
	-pin A \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Y} \
	-vector {R 0 0} \
	-pin A \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Y} \
	-vector {0 F 0} \
	-pin B \
	{ AND2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Y} \
	-vector {0 R 0} \
	-pin B \
	{ AND2 }

define_leakage -when "!A*!B*!Y" { AND2 }
define_leakage -when "!A*B*!Y" { AND2 }
define_leakage -when "A*!B*!Y" { AND2 }
define_leakage -when "A*B*Y" { AND2 }
define_arc \
	-prevector_pinlist {A} \
	-prevector {0} \
	-type combinational \
	-pinlist {A Y} \
	-vector {R F} \
	-related_pin A \
	-pin Y \
	{ INVX1 INVX2 INVX3 }

define_arc \
	-type combinational \
	-prevector_pinlist {A} \
	-prevector {1} \
	-pinlist {A Y} \
	-vector {F R} \
	-related_pin A \
	-pin Y \
	{ INVX1 INVX2 INVX3 }

define_leakage -when "!A*Y" { INVX1 INVX2 INVX3 }
define_leakage -when "A*!Y" { INVX1 INVX2 INVX3 }
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
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {F 1 1} \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {R 0 0} \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {1 F 1} \
	-pin B \
	{ C2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {0 R 0} \
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
	-prevector {110} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {F 0 1 0} \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {R 0 0 0} \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {0 F 1 0} \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {0 R 0 0} \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {0 0 F 0} \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {0 0 R 0} \
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
	-prevector {011110 011100} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 1 1 0 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100 000110} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 1 1 0 R} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000 001010} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 R 1 0 R} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001110} \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001111} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000011} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 F 1 0} \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 R 0 0} \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {100001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {F 0 0 0 0 1 0} \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {R 0 0 0 0 0 0} \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {010001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 F 0 0 0 1 0} \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 R 0 0 0 0 0} \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {001000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 F 0 0 0 0} \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 R 0 0 0 0} \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000100} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 F 0 0 0} \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 R 0 0 0} \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000001} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 0 F 0} \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-prevector_pinlist {M1 M2 P1 P2 C R} \
	-prevector {000000} \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 0 R 0} \
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
	-prevector {10} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {1 R F X} \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-pinlist {S R Q Qn} \
	-vector {1 F R X} \
	-related_pin R \
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
	-prevector {10 00} \
	-pinlist {S R Q Qn} \
	-vector {0 R X R} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-prevector_pinlist {S R} \
	-prevector {01} \
	-type combinational \
	-pinlist {S R Q Qn} \
	-vector {R 1 X F} \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-prevector_pinlist {S R} \
	-prevector {11} \
	-pinlist {S R Q Qn} \
	-vector {F 1 X R} \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {01} \
	-pinlist {S R Q Qn} \
	-vector {0 F 0 1} \
	-pin R \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {01 00} \
	-pinlist {S R Q Qn} \
	-vector {0 R 0 1} \
	-pin R \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {10} \
	-pinlist {S R Q Qn} \
	-vector {F 0 1 0} \
	-pin S \
	{ SR }

define_arc \
	-type hidden \
	-prevector_pinlist {S R} \
	-prevector {10 00} \
	-pinlist {S R Q Qn} \
	-vector {R 0 1 0} \
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
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Qa Qb} \
	-vector {0 R X R} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {01 11} \
	-pinlist {A B Qa Qb} \
	-vector {F 1 0 1} \
	-pin A \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {01} \
	-pinlist {A B Qa Qb} \
	-vector {R 1 0 1} \
	-pin A \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {10 11} \
	-pinlist {A B Qa Qb} \
	-vector {1 F 1 0} \
	-pin B \
	{ MUT }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {10} \
	-pinlist {A B Qa Qb} \
	-vector {1 R 1 0} \
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
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
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
	-related_pin D \
	-pin Q \
	{ DLH }

define_arc \
	-type combinational \
	-prevector_pinlist {G D} \
	-prevector {10} \
	-pinlist {G D Q} \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLH }

define_arc \
	-prevector_pinlist {G D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {G D Q} \
	-vector {R 0 F} \
	-related_pin G \
	-pin Q \
	{ DLH }

define_arc \
	-type edge \
	-prevector_pinlist {G D} \
	-prevector {10 00 01} \
	-pinlist {G D Q} \
	-vector {R 1 R} \
	-related_pin G \
	-pin Q \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {11 01} \
	-pinlist {G D Q} \
	-vector {0 F 1} \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10 00} \
	-pinlist {G D Q} \
	-vector {0 R 0} \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10} \
	-pinlist {G D Q} \
	-vector {F 0 0} \
	-pin G \
	{ DLH }

define_arc \
	-type hidden \
	-prevector_pinlist {G D} \
	-prevector {10 00} \
	-pinlist {G D Q} \
	-vector {R 0 0} \
	-pin G \
	{ DLH }

define_leakage -when "!D*G*!Q" { DLH }
define_leakage -when "D*G*Q" { DLH }
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
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 1 0 R} \
	-related_pin CLKA \
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
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 1 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 1 0 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 1 0 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 1 1 0 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 1 1 0 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 1 0 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 0 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 F 0 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 0 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 1 F 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 1 R 0} \
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
	-related_pin A \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q} \
	-vector {R 1 R} \
	-related_pin A \
	-pin Q \
	{ C2GATE }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q} \
	-vector {0 F F} \
	-related_pin B \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q} \
	-vector {1 R R} \
	-related_pin B \
	-pin Q \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {F 1 1} \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {R 0 0} \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q} \
	-vector {1 F 1} \
	-pin B \
	{ C2GATE }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q} \
	-vector {0 R 0} \
	-pin B \
	{ C2GATE }

define_leakage -when "!A*!B*!Q" { C2GATE }
define_leakage -when "A*B*Q" { C2GATE }
