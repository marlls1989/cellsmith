define_arc \
	-type combinational \
	-pinlist {A B Y} \
	-vector {R 1 R} \
	-related_pin A \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-pinlist {A B Y} \
	-vector {1 R R} \
	-related_pin B \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-pinlist {A B Y} \
	-vector {F 1 F} \
	-related_pin A \
	-pin Y \
	{ AND2 }

define_arc \
	-type combinational \
	-pinlist {A B Y} \
	-vector {1 F F} \
	-related_pin B \
	-pin Y \
	{ AND2 }

define_arc \
	-type hidden \
	-pinlist {A B Y} \
	-vector {R 0 0} \
	-pin A \
	{ AND2 }

define_arc \
	-type hidden \
	-pinlist {A B Y} \
	-vector {0 R 0} \
	-pin B \
	{ AND2 }

define_arc \
	-type hidden \
	-pinlist {A B Y} \
	-vector {0 F 0} \
	-pin B \
	{ AND2 }

define_arc \
	-type hidden \
	-pinlist {A B Y} \
	-vector {F 0 0} \
	-pin A \
	{ AND2 }

define_leakage -when "!A*!B*!Y" { AND2 }

define_leakage -when "!A*B*!Y" { AND2 }

define_leakage -when "A*!B*!Y" { AND2 }

define_leakage -when "A*B*Y" { AND2 }

define_arc \
	-type combinational \
	-pinlist {A Y} \
	-vector {R F} \
	-related_pin A \
	-pin Y \
	{ INVX1 INVX2 INVX3 }

define_arc \
	-type combinational \
	-pinlist {A Y} \
	-vector {F R} \
	-related_pin A \
	-pin Y \
	{ INVX1 INVX2 INVX3 }

define_leakage -when "!A*Y" { INVX1 INVX2 INVX3 }

define_leakage -when "A*!Y" { INVX1 INVX2 INVX3 }

define_arc \
	-type combinational \
	-pinlist {A B Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin B \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-pinlist {A B Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin A \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-pinlist {A B Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin B \
	-pin Q \
	{ C2 }

define_arc \
	-type combinational \
	-pinlist {A B Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin A \
	-pin Q \
	{ C2 }

define_arc \
	-type hidden \
	-pinlist {A B Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-pinlist {A B Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin B \
	{ C2 }

define_arc \
	-type hidden \
	-pinlist {A B Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin A \
	{ C2 }

define_arc \
	-type hidden \
	-pinlist {A B Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin B \
	{ C2 }

define_leakage -when "!A*!B*!Q" { C2 }

define_leakage -when "A*B*Q" { C2 }

define_leakage \
	-pinlist {A B Q} \
	-vector {1 0 0} \
	-when "A*!B*!Q" \
	{ C2 }

define_leakage \
	-pinlist {A B Q} \
	-vector {0 1 0} \
	-when "!A*B*!Q" \
	{ C2 }

define_leakage \
	-pinlist {A B Q} \
	-vector {0 1 1} \
	-when "!A*B*Q" \
	{ C2 }

define_leakage \
	-pinlist {A B Q} \
	-vector {1 0 1} \
	-when "A*!B*Q" \
	{ C2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F 0 F} \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin A \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 0 F} \
	-related_pin A \
	-pin Q \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin A \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin B \
	{ RCELEM2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin A \
	{ RCELEM2 }

define_leakage -when "!A*!B*!Q*!R" { RCELEM2 }

define_leakage -when "!A*!B*!Q*R" { RCELEM2 }

define_leakage -when "!A*B*!Q*R" { RCELEM2 }

define_leakage -when "A*!B*!Q*R" { RCELEM2 }

define_leakage -when "A*B*Q*!R" { RCELEM2 }

define_leakage -when "A*B*!Q*R" { RCELEM2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 1 0 1} \
	-when "!A*B*Q*!R" \
	{ RCELEM2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 1 0 0} \
	-when "!A*B*!Q*!R" \
	{ RCELEM2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 0 0 0} \
	-when "A*!B*!Q*!R" \
	{ RCELEM2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 0 0 1} \
	-when "A*!B*Q*!R" \
	{ RCELEM2 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 $VDD $VDD 0 0 0" \
	-vector {0 0 1 1 R 0 R} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 $VDD $VDD $VDD 0 $VDD" \
	-vector {0 0 1 1 F 0 F} \
	-related_pin C \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 $VDD $VDD $VDD 0 $VDD" \
	-vector {0 0 1 1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 $VDD $VDD $VDD $VDD 0" \
	-vector {0 0 1 1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 $VDD $VDD 0 $VDD 0 0" \
	-vector {0 1 1 R 1 0 R} \
	-related_pin P2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 $VDD $VDD $VDD 0 0 $VDD" \
	-vector {0 F 1 1 0 0 F} \
	-related_pin M2 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 $VDD 0 $VDD $VDD 0 0" \
	-vector {0 1 R 1 1 0 R} \
	-related_pin P1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "$VDD 0 $VDD $VDD 0 0 $VDD" \
	-vector {F 0 1 1 0 0 F} \
	-related_pin M1 \
	-pin Q \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 0 0" \
	-vector {R 0 0 0 0 0 0} \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 R 0 0 0 0 0} \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 R 0 0 0 0} \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 0 R 0 0 0} \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 0 0 R 0 0} \
	-pin C \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 0 0 0 R 0} \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 0 $VDD 0" \
	-vector {0 0 0 0 0 F 0} \
	-pin R \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 $VDD 0 0 0" \
	-vector {0 0 0 F 0 0 0} \
	-pin P2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 $VDD 0 0 0 0" \
	-vector {0 0 F 0 0 0 0} \
	-pin P1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 $VDD 0 0 0 $VDD 0" \
	-vector {0 F 0 0 0 1 0} \
	-pin M2 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "$VDD 0 0 0 0 $VDD 0" \
	-vector {F 0 0 0 0 1 0} \
	-pin M1 \
	{ RACELEM21 }

define_arc \
	-type hidden \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-ic "0 0 0 0 $VDD $VDD 0" \
	-vector {0 0 0 0 F 1 0} \
	-pin C \
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

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 0 0 0} \
	-when "!C*M1*!M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 1 0 0} \
	-when "C*!M1*M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 1 0 0} \
	-when "C*M1*M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 1 0 0} \
	-when "C*!M1*!M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 1 0 0} \
	-when "C*M1*M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 0 0 1} \
	-when "!C*!M1*M2*P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 1 0 0} \
	-when "C*!M1*M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 1 0 0} \
	-when "C*M1*!M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 0 0 0} \
	-when "!C*!M1*M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 1 0 0} \
	-when "C*!M1*!M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 1 0 1} \
	-when "C*!M1*M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 0 0 0} \
	-when "!C*!M1*M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 1 0 1} \
	-when "C*!M1*!M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 1 1 0 0} \
	-when "C*!M1*!M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 0 0 0} \
	-when "!C*M1*M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 1 0 1} \
	-when "C*M1*!M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 1 0 0 0} \
	-when "!C*!M1*M2*P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 1 0 0} \
	-when "C*M1*M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 1 0 1} \
	-when "C*M1*M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 0 0 1} \
	-when "!C*M1*!M2*P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 0 0 0} \
	-when "!C*M1*!M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 1 0 1} \
	-when "C*M1*M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 0 0 1} \
	-when "!C*M1*M2*P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 0 0 0} \
	-when "!C*M1*M2*P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 1 0 1} \
	-when "C*M1*!M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 0 0 0} \
	-when "!C*M1*!M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 1 0 0 0} \
	-when "!C*M1*M2*P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 1 0 0} \
	-when "C*!M1*M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 1 0 0 0} \
	-when "!C*M1*!M2*P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 0 0 0} \
	-when "!C*M1*M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 1 0 0} \
	-when "C*M1*!M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 1 0 1 0 1} \
	-when "C*!M1*!M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 1 0 1} \
	-when "C*!M1*M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 1 0 0} \
	-when "C*M1*!M2*!P1*!P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 0 0 0} \
	-when "!C*!M1*M2*!P1*P2*!Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 1 0 0 1} \
	-when "!C*M1*M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 1 0 0 1} \
	-when "!C*!M1*M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 1 0 1} \
	-when "C*M1*!M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 1 0 0 0 1} \
	-when "!C*M1*!M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 1 0 0 0 1} \
	-when "!C*!M1*M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 1 0 1} \
	-when "C*!M1*M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 1 0 1} \
	-when "C*M1*M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 0 0 0 1 0 1} \
	-when "C*!M1*!M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 1 0 0 1} \
	-when "!C*M1*!M2*!P1*P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 1 0 0 0 1} \
	-when "!C*M1*M2*P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 1 0 0 0 0 1} \
	-when "!C*M1*M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {0 1 0 0 0 0 1} \
	-when "!C*!M1*M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_leakage \
	-pinlist {M1 M2 P1 P2 C R Q} \
	-vector {1 0 0 0 0 0 1} \
	-when "!C*M1*!M2*!P1*!P2*Q*!R" \
	{ RACELEM21 }

define_arc \
	-type combinational \
	-pinlist {S R Q Qn} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X R} \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-pinlist {S R Q Qn} \
	-ic "$VDD $VDD 0 0" \
	-vector {1 F R X} \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-pinlist {S R Q Qn} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R F X} \
	-related_pin R \
	-pin Q \
	{ SR }

define_arc \
	-type combinational \
	-pinlist {S R Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 X F} \
	-related_pin S \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-pinlist {S R Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {0 R X R} \
	-related_pin R \
	-pin Qn \
	{ SR }

define_arc \
	-type combinational \
	-pinlist {S R Q Qn} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 R X} \
	-related_pin S \
	-pin Q \
	{ SR }

define_arc \
	-type hidden \
	-pinlist {S R Q Qn} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin S \
	{ SR }

define_arc \
	-type hidden \
	-pinlist {S R Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F 0 1} \
	-pin R \
	{ SR }

define_arc \
	-type hidden \
	-pinlist {S R Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin S \
	{ SR }

define_arc \
	-type hidden \
	-pinlist {S R Q Qn} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 1} \
	-pin R \
	{ SR }

define_leakage -when "!Q*!Qn*R*S" { SR }

define_leakage -when "Q*!Qn*!R*S" { SR }

define_leakage -when "!Q*Qn*R*!S" { SR }

define_leakage \
	-pinlist {S R Q Qn} \
	-vector {0 0 1 0} \
	-when "Q*!Qn*!R*!S" \
	{ SR }

define_leakage \
	-pinlist {S R Q Qn} \
	-vector {0 0 0 1} \
	-when "!Q*Qn*!R*!S" \
	{ SR }

define_arc \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-ic "0 0 0 0" \
	-vector {R 0 R X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-ic "0 0 0 0" \
	-vector {0 R X R} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F X F} \
	-related_pin B \
	-pin Qb \
	{ MUT }

define_arc \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 F X} \
	-related_pin A \
	-pin Qa \
	{ MUT }

define_arc \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 X R} \
	-related_pin A \
	-pin Qb \
	{ MUT }

define_arc \
	-type combinational \
	-pinlist {A B Qa Qb} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 F R X} \
	-related_pin B \
	-pin Qa \
	{ MUT }

define_arc \
	-type hidden \
	-pinlist {A B Qa Qb} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 0 1} \
	-pin A \
	{ MUT }

define_arc \
	-type hidden \
	-pinlist {A B Qa Qb} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin B \
	{ MUT }

define_arc \
	-type hidden \
	-pinlist {A B Qa Qb} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin B \
	{ MUT }

define_arc \
	-type hidden \
	-pinlist {A B Qa Qb} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {F 1 0 1} \
	-pin A \
	{ MUT }

define_leakage -when "!A*!B*!Qa*!Qb" { MUT }

define_leakage -when "!A*B*!Qa*Qb" { MUT }

define_leakage -when "A*!B*Qa*!Qb" { MUT }

define_leakage \
	-pinlist {A B Qa Qb} \
	-vector {1 1 1 0} \
	-when "A*B*Qa*!Qb" \
	{ MUT }

define_leakage \
	-pinlist {A B Qa Qb} \
	-vector {1 1 0 1} \
	-when "A*B*!Qa*Qb" \
	{ MUT }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DFF }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ DFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ DFF }

define_arc \
	-type combinational \
	-pinlist {G D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLH }

define_arc \
	-type combinational \
	-pinlist {G D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLH }

define_arc \
	-type edge \
	-pinlist {G D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin G \
	-pin Q \
	{ DLH }

define_arc \
	-type edge \
	-pinlist {G D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin G \
	-pin Q \
	{ DLH }

define_arc \
	-type hidden \
	-pinlist {G D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin G \
	{ DLH }

define_arc \
	-type hidden \
	-pinlist {G D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin G \
	{ DLH }

define_arc \
	-type hidden \
	-pinlist {G D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLH }

define_arc \
	-type hidden \
	-pinlist {G D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLH }

define_leakage -when "!D*G*!Q" { DLH }

define_leakage -when "D*G*Q" { DLH }

define_leakage \
	-pinlist {G D Q} \
	-vector {0 0 0} \
	-when "!D*!G*!Q" \
	{ DLH }

define_leakage \
	-pinlist {G D Q} \
	-vector {0 1 1} \
	-when "D*!G*Q" \
	{ DLH }

define_leakage \
	-pinlist {G D Q} \
	-vector {0 1 0} \
	-when "D*!G*!Q" \
	{ DLH }

define_leakage \
	-pinlist {G D Q} \
	-vector {0 0 1} \
	-when "!D*!G*Q" \
	{ DLH }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 $VDD 0 $VDD 0 $VDD $VDD $VDD 0 0 0 0" \
	-vector {R 1 0 1 0 X X X X X X R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD 0 0 0 $VDD $VDD $VDD 0" \
	-vector {1 R 1 0 1 X X X X X X R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 0 0 $VDD $VDD $VDD $VDD" \
	-vector {1 F 1 0 1 X X X X X X F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD 0 0 0 $VDD $VDD $VDD $VDD" \
	-vector {1 1 1 R 1 X X X X X X F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD 0 $VDD 0 $VDD $VDD $VDD 0 0 0 $VDD" \
	-vector {F 1 0 1 0 X X X X X X F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD $VDD 0 $VDD 0 $VDD $VDD $VDD 0 0 0 $VDD" \
	-vector {1 1 R 1 0 X X X X X X F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {R 0 1 1 0 X X X X X X 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 R 1 1 0 X X X X X X 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 0 F 1 0 X X X X X X 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 0 1 F 0 X X X X X X 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 0 1 1 R X X X X X X 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0 0 0 0 0 0 0" \
	-vector {0 0 1 1 F X X X X X X 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 $VDD $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {0 F 1 1 0 X X X X X X 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "$VDD 0 $VDD $VDD 0 0 0 0 0 0 0 0" \
	-vector {F 0 1 1 0 X X X X X X 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 $VDD 0 $VDD 0 $VDD 0 0 0 0 0 0" \
	-vector {0 1 R 1 0 X X X X X X 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-ic "0 0 $VDD 0 0 0 0 0 0 0 0 0" \
	-vector {0 0 1 R 0 X X X X X X 0} \
	-pin RB \
	{ ICM }

define_leakage -when "!CLKA*!CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "!CLKA*!CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage -when "!CLKA*CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "!CLKA*CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage -when "CLKA*!CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "CLKA*!CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage -when "CLKA*CLKB*!GCLK*RA*RB*!S" { ICM }

define_leakage -when "CLKA*CLKB*!GCLK*RA*RB*S" { ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 0 1 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 0 0 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 1 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 0 1 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 1 0 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 1 0 0 0 1 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 1 0 0 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 1 0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 1 1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 0 1 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 1 0 0 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 0 1 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 0 0 1 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 1 0 0 0 1 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 1 1 0 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 1 1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 1 0 0 0 0 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 1 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 1 0 0 0 0 0 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 1 1 0 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 0 0 1 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 0 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 0 0 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 1 1 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 1 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 1 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 0 0 0 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 0 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 1 0 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 1 0 1 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 1 0 1 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 1 0 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 0 0 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 1 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 0 0 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 1 1 1 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 0 0 1 1 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 0 0 0 1 0 1 1 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 1 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 1 1 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 0 0 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 0 0 0 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 0 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 1 0 1 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 1 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 0 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 1 1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 0 0 0 1 1 1 1 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {0 1 0 0 0 0 1 1 1 1 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 0 0 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 0 0 0 1 1 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 enA selb1 selb2 enB GCLK} \
	-vector {1 1 0 0 1 1 1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {A B QN Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 X R} \
	-related_pin A \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-pinlist {A B QN Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F X F} \
	-related_pin B \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-pinlist {A B QN Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R X R} \
	-related_pin B \
	-pin Q \
	{ C2GATE }

define_arc \
	-type combinational \
	-pinlist {A B QN Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 X F} \
	-related_pin A \
	-pin Q \
	{ C2GATE }

define_arc \
	-type hidden \
	-pinlist {A B QN Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 X 0} \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-pinlist {A B QN Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R X 0} \
	-pin B \
	{ C2GATE }

define_arc \
	-type hidden \
	-pinlist {A B QN Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {F 1 X 1} \
	-pin A \
	{ C2GATE }

define_arc \
	-type hidden \
	-pinlist {A B QN Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 F X 1} \
	-pin B \
	{ C2GATE }

define_leakage -when "!A*!B*!Q" { C2GATE }

define_leakage -when "A*B*Q" { C2GATE }

define_leakage \
	-pinlist {A B QN Q} \
	-vector {0 1 1 0} \
	-when "!A*B*!Q" \
	{ C2GATE }

define_leakage \
	-pinlist {A B QN Q} \
	-vector {0 1 0 1} \
	-when "!A*B*Q" \
	{ C2GATE }

define_leakage \
	-pinlist {A B QN Q} \
	-vector {1 0 1 0} \
	-when "A*!B*!Q" \
	{ C2GATE }

define_leakage \
	-pinlist {A B QN Q} \
	-vector {1 0 0 1} \
	-when "A*!B*Q" \
	{ C2GATE }

