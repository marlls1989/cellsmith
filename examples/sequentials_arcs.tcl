define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
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
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
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
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-when "!CLK*!D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-when "!CLK*D*!Q" \
	{ DFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
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
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-when "!CLK*D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-when "!CLK*!D*Q" \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-when "!CLK*D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-when "!CLK*!D*Q" \
	{ UCDFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ UCDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ UCDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {0 R R X} \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 R X} \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 X R} \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 X F} \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F 1 1 1} \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin CLK \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*M*Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*M*Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*M*Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-when "!CLK*D*M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-when "!CLK*!D*!M*Q" \
	{ EMDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {M Q} \
	{ EMDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ EMDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X F} \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {0 R X R} \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X R} \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F X F} \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 F X} \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 R X} \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin CLK \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q*T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q*T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*Q*T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-when "!CLK*!D*Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-when "!CLK*D*!Q*T" \
	{ TAPDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q T} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ TAPDFF }

define_arc \
	-type setup \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type hold \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type setup \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type hold \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q T} \
	{ TAPDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 F} \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 R} \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 1} \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R 1} \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {1 F 0} \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 0} \
	-pin CLK \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-when "!CLK*D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-when "!CLK*!D*!Q" \
	{ IDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ IDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ IDFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 R X} \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 X F} \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 F X} \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 X R} \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 1 0} \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD 0 0 $VDD" \
	-vector {1 R 0 1} \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 1 0} \
	-pin CLK \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-when "!CLK*D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-when "!CLK*!D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-when "!CLK*D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-when "!CLK*!D*Q*!Qn" \
	{ XN }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q Qn} \
	{ XN }

define_arc \
	-type setup \
	-pinlist {CLK D M Q Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R R X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type hold \
	-pinlist {CLK D M Q Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R R X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type setup \
	-pinlist {CLK D M Q Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type hold \
	-pinlist {CLK D M Q Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q Qn M} \
	{ XN }

define_arc \
	-type edge \
	-pinlist {CLK R Q} \
	-ic "0 0 0" \
	-vector {R 0 R} \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-type async \
	-pinlist {CLK R Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R F} \
	-related_pin R \
	-pin Q \
	{ TFF }

define_arc \
	-type edge \
	-pinlist {CLK R Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 0} \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin R \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 0} \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-pinlist {CLK R Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin R \
	{ TFF }

define_leakage -when "!CLK*!Q*R" { TFF }

define_leakage -when "CLK*!Q*R" { TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {11 10} \
	-when "CLK*!Q*!R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {01 00} \
	-when "!CLK*!Q*!R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10} \
	-when "CLK*Q*!R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10 00} \
	-when "!CLK*Q*!R" \
	{ TFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK R M Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK R M Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type setup \
	-pinlist {CLK R M Q} \
	-ic "0 $VDD 0 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type hold \
	-pinlist {CLK R M Q} \
	-ic "0 $VDD 0 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK R M Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {1 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ TFF }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-when "!CLK*D*Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-when "!CLK*!D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-when "!CLK*D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-when "!CLK*!D*Q" \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L1 Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L1 Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L2 Q} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {L2} \
	{ DET }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D L1 Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {L1} \
	{ DET }

define_arc \
	-type setup \
	-pinlist {CLK D L1 Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type hold \
	-pinlist {CLK D L1 Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {L1} \
	{ DET }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D L2 Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {L2} \
	{ DET }

define_arc \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ MOR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R 1 1 F} \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 0 F 0} \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ MOR }

define_leakage -when "CLK*!D*!Q*R" { MOR }

define_leakage -when "CLK*D*!Q*R" { MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-when "CLK*!D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-when "!CLK*!D*!Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-when "!CLK*D*!Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-when "CLK*D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-when "CLK*D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001 000} \
	-when "!CLK*!D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110 010} \
	-when "!CLK*D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-when "CLK*!D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-when "!CLK*D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-when "!CLK*D*Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-when "!CLK*!D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-when "!CLK*!D*Q*R" \
	{ MOR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ MOR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MOR }

define_arc \
	-type async \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ MORA }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R 1 1 F} \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 0 F 0} \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 1 0} \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin R \
	{ MORA }

define_leakage -when "CLK*!D*!Q*R" { MORA }

define_leakage -when "CLK*D*!Q*R" { MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-when "!CLK*D*!Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-when "CLK*!D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-when "!CLK*!D*!Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-when "CLK*D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-when "CLK*D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-when "CLK*!D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110 010} \
	-when "!CLK*D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001 000} \
	-when "!CLK*!D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-when "!CLK*D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-when "!CLK*D*Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-when "!CLK*!D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-when "!CLK*!D*Q*R" \
	{ MORA }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ MORA }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ MORA }

define_arc \
	-type async \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin R \
	{ BR }

define_leakage -when "!CLK*!D*!Q*R" { BR }

define_leakage -when "!CLK*D*!Q*R" { BR }

define_leakage -when "CLK*!D*!Q*R" { BR }

define_leakage -when "CLK*D*!Q*R" { BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-when "CLK*!D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-when "!CLK*!D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-when "CLK*D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-when "CLK*D*Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-when "!CLK*D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-when "CLK*!D*Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-when "!CLK*D*Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-when "!CLK*!D*Q*!R" \
	{ BR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ BR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ BR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ BR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ BR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ BR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ BR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ SYNCR }

define_leakage -when "!CLK*!D*!Q*R" { SYNCR }

define_leakage -when "!CLK*D*!Q*R" { SYNCR }

define_leakage -when "CLK*!D*!Q*R" { SYNCR }

define_leakage -when "CLK*D*!Q*R" { SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-when "!CLK*!D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-when "!CLK*D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-when "CLK*!D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-when "CLK*D*Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-when "CLK*D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-when "!CLK*D*Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-when "CLK*!D*Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100 000} \
	-when "!CLK*!D*Q*!R" \
	{ SYNCR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ SYNCR }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 R 0 F} \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 0 R F} \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {R 0 1 0 0} \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 R 1 0 0} \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 F 0 0} \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 1 R 0} \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {0 F 1 0 0} \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD 0 0" \
	-vector {F 0 1 0 0} \
	-pin CLK \
	{ SYNCRG }

define_leakage -when "!CLK*!D*!G*!Q*R" { SYNCRG }

define_leakage -when "!CLK*!D*G*!Q*!R" { SYNCRG }

define_leakage -when "!CLK*!D*G*!Q*R" { SYNCRG }

define_leakage -when "!CLK*D*!G*!Q*R" { SYNCRG }

define_leakage -when "!CLK*D*G*!Q*!R" { SYNCRG }

define_leakage -when "!CLK*D*G*!Q*R" { SYNCRG }

define_leakage -when "CLK*!D*!G*!Q*R" { SYNCRG }

define_leakage -when "CLK*!D*G*!Q*!R" { SYNCRG }

define_leakage -when "CLK*!D*G*!Q*R" { SYNCRG }

define_leakage -when "CLK*D*!G*!Q*R" { SYNCRG }

define_leakage -when "CLK*D*G*!Q*!R" { SYNCRG }

define_leakage -when "CLK*D*G*!Q*R" { SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-when "CLK*D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1101 1100} \
	-when "CLK*D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001 0000} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-when "CLK*!D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 0100} \
	-when "!CLK*D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-when "!CLK*D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-when "CLK*!D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000 0000} \
	-when "!CLK*!D*!G*Q*!R" \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 0 0 0" \
	-vector {F 1 0 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 0 $VDD $VDD" \
	-vector {1 1 R 0 X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD 0 0 0" \
	-vector {R 1 F 0 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD 0 0 0" \
	-vector {R 1 F 0 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 0 $VDD $VDD" \
	-vector {1 1 0 R X X} \
	-related_pin G \
	-pin G \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ SYNCRG }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ SYNCRG }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {1 1 1 R F} \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R 1 F} \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 R} \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {R 0 1 0 F} \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {R 0 1 1 0} \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 1 0} \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F 1 0} \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F 0} \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F 1 1 0} \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 1 1 0} \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 1 R 0} \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {1 0 R 1 0} \
	-pin R \
	{ GATEDR }

define_leakage -when "!CLK*!D*G*!Q*R" { GATEDR }

define_leakage -when "!CLK*D*G*!Q*R" { GATEDR }

define_leakage -when "CLK*!D*G*!Q*R" { GATEDR }

define_leakage -when "CLK*D*G*!Q*R" { GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-when "!CLK*!D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-when "CLK*D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001 1001} \
	-when "CLK*!D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 1010} \
	-when "CLK*!D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-when "CLK*D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-when "CLK*D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-when "!CLK*!D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-when "CLK*D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-when "CLK*!D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-when "!CLK*D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-when "!CLK*D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-when "CLK*D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-when "CLK*!D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-when "CLK*!D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-when "!CLK*D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-when "!CLK*D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000 0000} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-when "CLK*!D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110 0100} \
	-when "!CLK*D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000 1100} \
	-when "CLK*D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-when "!CLK*D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-when "!CLK*!D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-when "!CLK*!D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001 0001} \
	-when "!CLK*!D*G*Q*!R" \
	{ GATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 $VDD 0 0 0" \
	-vector {R R 1 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 $VDD 0 0 0" \
	-vector {R R 1 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD 0 0" \
	-vector {F 1 0 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R 1 X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD $VDD 0" \
	-vector {R F 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD $VDD 0" \
	-vector {R F 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 1 R X X} \
	-related_pin G \
	-pin G \
	-probe {Q M} \
	{ GATEDR }

define_arc \
	-type async \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R 1 F} \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 R} \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type async \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {1 1 1 R F} \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {R 0 1 1 0} \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 1 0} \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F 1 0} \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F 0} \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F 1 1 0} \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 1 1 0} \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {1 0 R 1 0} \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 0 R 1} \
	-pin G \
	{ AGATEDR }

define_leakage -when "!CLK*!D*G*!Q*R" { AGATEDR }

define_leakage -when "!CLK*D*G*!Q*R" { AGATEDR }

define_leakage -when "CLK*!D*G*!Q*R" { AGATEDR }

define_leakage -when "CLK*D*G*!Q*R" { AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-when "CLK*D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001 1001} \
	-when "CLK*!D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-when "CLK*D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-when "!CLK*D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-when "CLK*D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-when "CLK*D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-when "!CLK*!D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-when "!CLK*!D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-when "!CLK*D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-when "CLK*!D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 1010} \
	-when "CLK*!D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-when "CLK*D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-when "!CLK*D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-when "CLK*!D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-when "!CLK*D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110 1100} \
	-when "CLK*D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-when "!CLK*D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-when "CLK*!D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000 0000} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-when "CLK*!D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110 0100} \
	-when "!CLK*D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000 0000} \
	-when "!CLK*!D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001 0001} \
	-when "!CLK*!D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-when "!CLK*!D*!G*Q*R" \
	{ AGATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R F X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 $VDD 0 0 0" \
	-vector {R R 1 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 0 $VDD 0 0 0" \
	-vector {R R 1 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 1 R X X} \
	-related_pin G \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ AGATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD 0 0" \
	-vector {F 1 0 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 1 F X X} \
	-related_pin CLK \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 F R X X} \
	-related_pin R \
	-pin G \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {R 1 F 1 X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type setup \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD $VDD 0" \
	-vector {R F 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type hold \
	-pinlist {CLK D R G M Q} \
	-ic "0 $VDD 0 $VDD $VDD 0" \
	-vector {R F 0 1 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R G M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD $VDD" \
	-vector {1 1 R 1 X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ AGATEDR }

define_arc \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ RDFF }

define_leakage -when "!CLK*!D*!Q*R" { RDFF }

define_leakage -when "!CLK*D*!Q*R" { RDFF }

define_leakage -when "CLK*!D*!Q*R" { RDFF }

define_leakage -when "CLK*D*!Q*R" { RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-when "CLK*D*Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-when "!CLK*!D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-when "CLK*D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-when "!CLK*D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-when "CLK*!D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-when "!CLK*D*Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-when "CLK*!D*Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100 000} \
	-when "!CLK*!D*Q*!R" \
	{ RDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D R M Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type setup \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type hold \
	-pinlist {CLK D R M Q} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R M Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ RDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ RDFF }

define_arc \
	-type async \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F R} \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type combinational \
	-pinlist {CLK D B R Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {1 1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-type edge \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type edge \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {R 0 0 1 0} \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 R 0 1 0} \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {0 F 0 1 0} \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {F 0 0 1 0} \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F 0 1} \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-pinlist {CLK D B R Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {1 1 0 R 0} \
	-pin R \
	{ COEX }

define_leakage -when "!B*!CLK*!D*!Q*R" { COEX }

define_leakage -when "!B*!CLK*D*!Q*R" { COEX }

define_leakage -when "!B*CLK*!D*!Q*R" { COEX }

define_leakage -when "!B*CLK*D*!Q*R" { COEX }

define_leakage -when "B*!CLK*!D*Q*!R" { COEX }

define_leakage -when "B*!CLK*!D*!Q*R" { COEX }

define_leakage -when "B*!CLK*D*Q*!R" { COEX }

define_leakage -when "B*!CLK*D*!Q*R" { COEX }

define_leakage -when "B*CLK*!D*Q*!R" { COEX }

define_leakage -when "B*CLK*!D*!Q*R" { COEX }

define_leakage -when "B*CLK*D*Q*!R" { COEX }

define_leakage -when "B*CLK*D*!Q*R" { COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010 1000} \
	-when "!B*CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0100 1100} \
	-when "!B*CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101 1100} \
	-when "!B*CLK*D*!Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-when "!B*!CLK*D*!Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001 0000} \
	-when "!B*!CLK*!D*!Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-when "!B*!CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110 0100} \
	-when "!B*!CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0000 1000} \
	-when "!B*CLK*!D*!Q*!R" \
	{ COEX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD 0 0 0 $VDD $VDD" \
	-vector {F 0 0 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD $VDD 0 0 0 0" \
	-vector {1 1 R 0 X X} \
	-related_pin B \
	-pin B \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin B \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin B \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin B \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin B \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D B R M Q} \
	-ic "$VDD 0 0 0 $VDD $VDD" \
	-vector {1 0 0 R X X} \
	-related_pin R \
	-pin R \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type setup \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type hold \
	-pinlist {CLK D B R M Q} \
	-ic "0 $VDD 0 0 $VDD 0" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ COEX }

define_arc \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 1 R F} \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F R} \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type edge \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type edge \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R 0 R} \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {R 0 1 0 1} \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 R 1 0 1} \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F 0 1} \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 F 1 0 1} \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {F 0 1 0 1} \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 0 R 0} \
	-pin CLR \
	{ CAFF }

define_leakage -when "!CLK*!CLR*!D*PRE*Q" { CAFF }

define_leakage -when "!CLK*!CLR*D*PRE*Q" { CAFF }

define_leakage -when "!CLK*CLR*!D*!PRE*!Q" { CAFF }

define_leakage -when "!CLK*CLR*!D*PRE*!Q" { CAFF }

define_leakage -when "!CLK*CLR*D*!PRE*!Q" { CAFF }

define_leakage -when "!CLK*CLR*D*PRE*!Q" { CAFF }

define_leakage -when "CLK*!CLR*!D*PRE*Q" { CAFF }

define_leakage -when "CLK*!CLR*D*PRE*Q" { CAFF }

define_leakage -when "CLK*CLR*!D*!PRE*!Q" { CAFF }

define_leakage -when "CLK*CLR*!D*PRE*!Q" { CAFF }

define_leakage -when "CLK*CLR*D*!PRE*!Q" { CAFF }

define_leakage -when "CLK*CLR*D*PRE*!Q" { CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0100 1100} \
	-when "CLK*!CLR*D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110 0100} \
	-when "!CLK*!CLR*D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-when "!CLK*!CLR*!D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-when "!CLK*!CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010 1000} \
	-when "CLK*!CLR*!D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101 1100} \
	-when "CLK*!CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0000 1000} \
	-when "CLK*!CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001 0000} \
	-when "!CLK*!CLR*!D*!PRE*!Q" \
	{ CAFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD $VDD 0 0 0 0" \
	-vector {1 1 R 0 X X} \
	-related_pin PRE \
	-pin PRE \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 F X X} \
	-related_pin CLK \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type non_seq_setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin PRE \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type non_seq_hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {1 0 F F X X} \
	-related_pin PRE \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 0 0 0 $VDD" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 0 0 0 $VDD" \
	-vector {R R 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 0 $VDD $VDD" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 $VDD 0 0 $VDD $VDD" \
	-vector {R F 0 0 X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type setup \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin PRE \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type hold \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "0 0 $VDD 0 $VDD $VDD" \
	-vector {R 0 F 0 X X} \
	-related_pin CLK \
	-pin PRE \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ CAFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD $VDD 0 0 $VDD $VDD" \
	-vector {1 1 0 R X X} \
	-related_pin CLR \
	-pin CLR \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D PRE CLR M Q} \
	-ic "$VDD 0 0 0 $VDD $VDD" \
	-vector {F 0 0 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ CAFF }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT }

define_leakage -when "CLK*!D*!Q" { DLAT }

define_leakage -when "CLK*D*Q" { DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-when "!CLK*D*Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-when "!CLK*!D*!Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-when "!CLK*!D*Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-when "!CLK*D*!Q" \
	{ DLAT }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type setup \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type hold \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type setup \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type hold \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin CLK \
	-pin D \
	-probe {Q} \
	{ DLAT }

define_arc \
	-type combinational \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type combinational \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type edge \
	-pinlist {EN D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type edge \
	-pinlist {EN D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-pinlist {EN D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT_EN }

define_leakage -when "!D*EN*!Q" { DLAT_EN }

define_leakage -when "D*EN*Q" { DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {10 00} \
	-when "!D*!EN*!Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {11 01} \
	-when "D*!EN*Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {11 01 00} \
	-when "!D*!EN*Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {10 00 01} \
	-when "D*!EN*!Q" \
	{ DLAT_EN }

define_arc \
	-type setup \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type hold \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type min_pulse_width \
	-pinlist {EN D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 X} \
	-related_pin EN \
	-pin EN \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type setup \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type hold \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin EN \
	-pin D \
	-probe {Q} \
	{ DLAT_EN }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-pinlist {E D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {E D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT_E }

define_leakage -when "!D*E*!Q" { DLAT_E }

define_leakage -when "D*E*Q" { DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {10 00} \
	-when "!D*!E*!Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {11 01} \
	-when "D*!E*Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {11 01 00} \
	-when "!D*!E*Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {10 00 01} \
	-when "D*!E*!Q" \
	{ DLAT_E }

define_arc \
	-type min_pulse_width \
	-pinlist {E D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 X} \
	-related_pin E \
	-pin E \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_setup \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_hold \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_setup \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type non_seq_hold \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F F X} \
	-related_pin E \
	-pin D \
	-probe {Q} \
	{ DLAT_E }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R 1} \
	-pin D \
	{ GLAT }

define_leakage -when "CLK*D*Q" { GLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-when "!CLK*D*Q" \
	{ GLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 10} \
	-when "CLK*!D*Q" \
	{ GLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 10 00} \
	-when "!CLK*!D*Q" \
	{ GLAT }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 1 R R} \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 1 F F} \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 0} \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin D \
	{ MUXLAT }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD $VDD" \
	-vector {0 0 F 1} \
	-pin D \
	{ MUXLAT }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MUXLAT }

define_leakage -when "!CLKA*CLKB*D*Q" { MUXLAT }

define_leakage -when "CLKA*!CLKB*!D*!Q" { MUXLAT }

define_leakage -when "CLKA*!CLKB*D*Q" { MUXLAT }

define_leakage -when "CLKA*CLKB*!D*!Q" { MUXLAT }

define_leakage -when "CLKA*CLKB*D*Q" { MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000 001} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001 000} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 R X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 R X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F R X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F R X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin CLKB \
	-pin D \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q} \
	{ MUXLAT }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 1 R R} \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 1 F F} \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {F 1 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 0} \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD $VDD" \
	-vector {0 0 F 1} \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 0 0} \
	-pin CLKA \
	{ MCDFF }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MCDFF }

define_leakage -when "!CLKA*CLKB*D*Q" { MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-when "CLKA*CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-when "CLKA*CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-when "CLKA*CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110 100} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100 101} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101 100} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100 101} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D M Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {F 1 1 X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q M} \
	{ MCDFF }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q} \
	{ MCDFF }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R X R} \
	-related_pin D \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {0 1 F X F} \
	-related_pin D \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {F 1 1 X R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 X F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 F 0 X 0} \
	-pin CLKB \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 0 0 0" \
	-vector {0 R 0 X 0} \
	-pin CLKB \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 0 0 0" \
	-vector {0 0 R X 0} \
	-pin D \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX1 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F X 1} \
	-pin D \
	{ MCDFFX1 }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R X R} \
	-related_pin D \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {0 1 F X F} \
	-related_pin D \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {F 1 1 X R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 X F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 F 0 X 0} \
	-pin CLKB \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 0 0 0" \
	-vector {0 R 0 X 0} \
	-pin CLKB \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 0 0 0" \
	-vector {0 0 R X 0} \
	-pin D \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X 0} \
	-pin CLKA \
	{ MCDFFX4 }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD $VDD $VDD $VDD" \
	-vector {1 1 F X 1} \
	-pin D \
	{ MCDFFX4 }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MCDFFX1 MCDFFX4 }

define_leakage -when "!CLKA*CLKB*D*Q" { MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-when "CLKA*CLKB*D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-when "CLKA*CLKB*!D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 101} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-when "CLKA*CLKB*D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 101 100} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100 101} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100 101} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFFX1 MCDFFX4 }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101 100} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFFX1 MCDFFX4 }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q} \
	{ MCDFFX1 }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 X X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q} \
	{ MCDFFX4 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 F X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 R X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D XI7/m Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q XI7/m} \
	{ MCDFFX1 }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D XI4/m Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F 1 0 X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q XI4/m} \
	{ MCDFFX4 }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R R} \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 R} \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ TCASC }

define_arc \
	-type hidden \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ TCASC }

define_leakage -when "!CLK*!D*!Q" { TCASC }

define_leakage -when "!CLK*D*Q" { TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*!Q" \
	{ TCASC }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type setup \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type hold \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {Q M} \
	{ TCASC }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "0 $VDD 0" \
	-vector {0 F R} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type edge \
	-pinlist {CLK D T} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R F} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type edge \
	-pinlist {CLK D T} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 F} \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D T} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ XLAT }

define_arc \
	-type hidden \
	-pinlist {CLK D T} \
	-ic "0 $VDD 0" \
	-vector {R 1 0} \
	-pin CLK \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-when "CLK*!D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-when "!CLK*D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-when "CLK*D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-when "!CLK*!D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-when "CLK*!D*T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-when "CLK*D*T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-when "!CLK*D*T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-when "!CLK*!D*T" \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M2 T} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M2 T} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M2 T} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M2 T} \
	-ic "$VDD 0 0 0" \
	-vector {F R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M T} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M T} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type setup \
	-pinlist {CLK D M T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type hold \
	-pinlist {CLK D M T} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin D \
	-probe {M} \
	{ XLAT }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M2 T} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {M2} \
	{ XLAT }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK D M T} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {M} \
	{ XLAT }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 F 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {1 0 F 1} \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin CLKA \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101} \
	-when "CLKA*!CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100} \
	-when "CLKA*!CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111} \
	-when "CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001} \
	-when "!CLKA*!CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110} \
	-when "CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101} \
	-when "CLKA*!CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 000} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101 111} \
	-when "CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011} \
	-when "!CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110} \
	-when "CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101 001} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110 010} \
	-when "!CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011} \
	-when "!CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111} \
	-when "CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110 010 110} \
	-when "CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110 010 110 111} \
	-when "CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111 110} \
	-when "CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110 010 110 010} \
	-when "!CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111 011} \
	-when "!CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110 010 110 010 011} \
	-when "!CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111 011 010} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "$VDD 0 0 $VDD $VDD $VDD" \
	-vector {F 0 0 X X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 $VDD $VDD $VDD $VDD" \
	-vector {R 0 F X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 $VDD $VDD $VDD $VDD" \
	-vector {R 0 F X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 R X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB D M1 M2 Q} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 R X X X} \
	-related_pin CLKA \
	-pin D \
	-probe {Q M1 M2} \
	{ HPIPE }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q} \
	{ HPIPE }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB D M2 Q} \
	-ic "0 0 0 $VDD $VDD" \
	-vector {R 0 0 X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q M2} \
	{ HPIPE }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {1 R 0 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 1 0 R} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 F 0 1 F} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 R 1 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 0 0 $VDD $VDD" \
	-vector {R 0 0 1 F} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {F 1 1 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {F 0 0 1 0} \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {1 0 R 1 0} \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {1 0 0 F 0} \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 0} \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 F 0 0 0} \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 0 R 0} \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 0 0" \
	-vector {1 R 0 0 0} \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {1 0 F 0 1} \
	-pin DA \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-when "CLKA*!CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100} \
	-when "!CLKA*CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-when "CLKA*!CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-when "CLKA*!CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-when "CLKA*!CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-when "!CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101} \
	-when "!CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-when "!CLKA*CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011} \
	-when "CLKA*!CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001} \
	-when "!CLKA*!CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001} \
	-when "CLKA*!CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000} \
	-when "CLKA*!CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 1101} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 0000} \
	-when "!CLKA*!CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100} \
	-when "!CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0010} \
	-when "!CLKA*!CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-when "!CLKA*!CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-when "!CLKA*!CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100} \
	-when "CLKA*CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101} \
	-when "!CLKA*CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0011} \
	-when "!CLKA*!CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010} \
	-when "CLKA*!CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 1111} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110} \
	-when "!CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111} \
	-when "!CLKA*CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110 1100} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 1111 1101} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100 1100} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001 0000} \
	-when "!CLKA*!CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010 1110} \
	-when "CLKA*CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101 1101} \
	-when "CLKA*CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111 0011} \
	-when "!CLKA*!CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 1101 1111} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110 1110} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111 1111} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111 1111 1101} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110 1110 1100} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 1101 1111 1110} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101 1101 1111} \
	-when "CLKA*CLKB*DA*DB*!Q" \
	{ DCMUX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {R 0 1 0 X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q} \
	{ DCMUX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB DA DB MA Q} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {F 0 1 1 X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA Q} \
	-ic "0 $VDD $VDD 0 $VDD 0" \
	-vector {R 1 F 0 X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA Q} \
	-ic "0 $VDD $VDD 0 $VDD 0" \
	-vector {R 1 F 0 X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB DA DB MB Q} \
	-ic "0 $VDD 0 0 $VDD $VDD" \
	-vector {0 F 0 0 X X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type non_seq_setup \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F F 0 1 X} \
	-related_pin CLKA \
	-pin CLKB \
	-probe {Q} \
	{ DCMUX }

define_arc \
	-type non_seq_hold \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {F F 0 1 X} \
	-related_pin CLKA \
	-pin CLKB \
	-probe {Q} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MA Q} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 R 0 X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MA Q} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 R 0 X X} \
	-related_pin CLKA \
	-pin DA \
	-probe {Q MA} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MB Q} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 R 0 R X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MB Q} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 R 0 R X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB DA DB MB Q} \
	-ic "$VDD 0 0 $VDD $VDD 0" \
	-vector {1 R 0 F X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB DA DB MB Q} \
	-ic "$VDD 0 0 $VDD $VDD 0" \
	-vector {1 R 0 F X X} \
	-related_pin CLKB \
	-pin DB \
	-probe {Q MB} \
	{ DCMUX }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 R 0 1 X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {Q} \
	{ DCMUX }

define_arc \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-pinlist {CLK EN GCLK} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ ICG }

define_leakage -when "!CLK*!EN*!GCLK" { ICG }

define_leakage -when "!CLK*EN*!GCLK" { ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11} \
	-when "CLK*EN*GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10} \
	-when "CLK*!EN*!GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11 10} \
	-when "CLK*!EN*GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10 11} \
	-when "CLK*EN*!GCLK" \
	{ ICG }

define_arc \
	-type setup \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type hold \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type min_pulse_width \
	-pinlist {CLK EN EL GCLK} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X X} \
	-related_pin CLK \
	-pin CLK \
	-probe {EL} \
	{ ICG }

define_arc \
	-type setup \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type hold \
	-pinlist {CLK EN EL GCLK} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin CLK \
	-pin EN \
	-probe {EL} \
	{ ICG }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD 0" \
	-vector {1 R 1 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 $VDD 0 $VDD 0 0" \
	-vector {R 1 0 1 0 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 0 $VDD 0 $VDD" \
	-vector {F 0 0 1 0 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 0 $VDD 0 $VDD" \
	-vector {1 0 R 1 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 F 1 0 1 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 1 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {R 0 1 1 0 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 R 1 1 0 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 0 F 1 0 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 0 1 F 0 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 0 1 1 R 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0" \
	-vector {0 0 1 1 F 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {0 F 1 1 0 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {F 0 1 1 0 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD $VDD $VDD 0 0 0" \
	-vector {1 1 1 R 0 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD $VDD 0 $VDD $VDD 0" \
	-vector {1 1 R 1 1 0} \
	-pin RA \
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
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011 00001} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101 11001} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100 10000} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 01000} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011 10001 11001} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11001 10001} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 01000 11000 01000} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01100 00100} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001 00000} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 10000} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 00001} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01100 00100 01100} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 01001} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 10000} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100 11000} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 10000 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10010} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01100 00100 01100 01101} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11000 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10001} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011 11010} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01100 00100 01100 01000} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011 10001} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11001 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100 11101} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01100 00100 01100 01000 01001} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00011 10011 10001 10000} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011 11001 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100 11101 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 00000} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 00001} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 10000} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 11001 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000 01000} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 11001 10001} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 00001 01001} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 00000 10000} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001 01000} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 00001 01001 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 00000 10000 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 11001 10001} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 10000 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 10000 11000 11001} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001 11001 11000} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB RA RB S enA sela2 selb1 GCLK} \
	-ic "0 0 0 0 $VDD $VDD $VDD 0 0" \
	-vector {R 0 0 0 1 X X X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {enA sela2 selb1} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S selb1 selb2 GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD $VDD 0 0" \
	-vector {1 R 1 0 F X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S selb1 selb2 GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD $VDD 0 0" \
	-vector {1 R 1 0 F X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S selb1 selb2 GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0 0 0" \
	-vector {0 R 1 F 1 X X X} \
	-related_pin CLKB \
	-pin RB \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S selb1 selb2 GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0 0 0" \
	-vector {0 R 1 F 1 X X X} \
	-related_pin CLKB \
	-pin RB \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0" \
	-vector {R 0 F 1 0 X X X} \
	-related_pin CLKA \
	-pin RA \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 GCLK} \
	-ic "0 0 $VDD $VDD 0 0 0 0" \
	-vector {R 0 F 1 0 X X X} \
	-related_pin CLKA \
	-pin RA \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB RA RB S enA sela1 sela2 selb1 GCLK} \
	-ic "$VDD 0 0 0 $VDD 0 $VDD $VDD $VDD 0" \
	-vector {F 0 0 0 1 X X X X X} \
	-related_pin CLKA \
	-pin CLKA \
	-probe {enA sela1 sela2 selb1} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S selb1 selb2 GCLK} \
	-ic "0 0 $VDD 0 0 0 0 0" \
	-vector {0 R 1 0 R X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S selb1 selb2 GCLK} \
	-ic "0 0 $VDD 0 0 0 0 0" \
	-vector {0 R 1 0 R X X X} \
	-related_pin CLKB \
	-pin S \
	-probe {selb1 selb2} \
	{ ICM }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB RA RB S enA sela1 sela2 selb1 GCLK} \
	-ic "$VDD 0 0 0 $VDD $VDD $VDD $VDD 0 $VDD" \
	-vector {1 0 R 0 1 X X X X X} \
	-related_pin RA \
	-pin RA \
	-probe {enA sela1 sela2 selb1} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 GCLK} \
	-ic "0 0 0 $VDD 0 $VDD 0 0" \
	-vector {R 0 0 1 R X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 GCLK} \
	-ic "0 0 0 $VDD 0 $VDD 0 0" \
	-vector {R 0 0 1 R X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB RA RB S enB sela1 selb1 selb2 GCLK} \
	-ic "0 $VDD 0 0 0 $VDD 0 $VDD $VDD $VDD" \
	-vector {0 1 0 R 0 X X X X X} \
	-related_pin RB \
	-pin RB \
	-probe {enB sela1 selb1 selb2} \
	{ ICM }

define_arc \
	-type setup \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 GCLK} \
	-ic "0 0 0 $VDD $VDD 0 0 0" \
	-vector {R 0 0 1 F X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type hold \
	-pinlist {CLKA CLKB RA RB S sela1 sela2 GCLK} \
	-ic "0 0 0 $VDD $VDD 0 0 0" \
	-vector {R 0 0 1 F X X X} \
	-related_pin CLKA \
	-pin S \
	-probe {sela1 sela2} \
	{ ICM }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB RA RB S enB sela1 selb2 GCLK} \
	-ic "0 0 0 0 0 $VDD 0 $VDD 0" \
	-vector {0 R 0 0 0 X X X X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {enB sela1 selb2} \
	{ ICM }

define_arc \
	-type min_pulse_width \
	-pinlist {CLKA CLKB RA RB S enB sela1 selb1 selb2 GCLK} \
	-ic "0 $VDD 0 0 0 0 $VDD $VDD $VDD 0" \
	-vector {0 F 0 0 0 X X X X X} \
	-related_pin CLKB \
	-pin CLKB \
	-probe {enB sela1 selb1 selb2} \
	{ ICM }

define_arc \
	-type combinational \
	-pinlist {C D Y} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type combinational \
	-pinlist {C D Y} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin C \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-pinlist {C D Y} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin C \
	{ GL }

define_leakage -when "!C*!D*!Y" { GL }

define_leakage -when "!C*D*!Y" { GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {01 11} \
	-when "C*D*Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {00 10} \
	-when "C*!D*!Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {00 10 11} \
	-when "C*D*!Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {01 11 10} \
	-when "C*!D*Y" \
	{ GL }

define_arc \
	-type min_pulse_width \
	-pinlist {C D L Y} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X X} \
	-related_pin C \
	-pin C \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_setup \
	-pinlist {C D L Y} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_hold \
	-pinlist {C D L Y} \
	-ic "0 0 0 0" \
	-vector {R R X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_setup \
	-pinlist {C D L Y} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type non_seq_hold \
	-pinlist {C D L Y} \
	-ic "0 $VDD $VDD 0" \
	-vector {R F X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ GL }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {0 0 R 1 R X} \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 0 0 X R} \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 R 0 0 X R} \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {F 1 0 0 X F} \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {1 F 0 0 X F} \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD $VDD $VDD $VDD 0" \
	-vector {0 1 F 1 F X} \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 0 0 0 0} \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 R 0 0 0 0} \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 0 R 0 0 0} \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 0 0 R 0 0} \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {0 0 0 F 0 0} \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {0 F 0 0 0 0} \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {F 0 0 0 0 0} \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 $VDD 0 0 0" \
	-vector {1 0 F 0 0 0} \
	-pin C \
	{ MIX }

define_leakage -when "!A*!B*!C*!D*!Y*!Z" { MIX }

define_leakage -when "!A*!B*!C*D*!Y*!Z" { MIX }

define_leakage -when "!A*B*!C*!D*!Y*!Z" { MIX }

define_leakage -when "!A*B*!C*D*!Y*!Z" { MIX }

define_leakage -when "A*!B*!C*!D*!Y*!Z" { MIX }

define_leakage -when "A*!B*!C*D*!Y*!Z" { MIX }

define_leakage -when "A*B*!C*!D*!Y*Z" { MIX }

define_leakage -when "A*B*!C*D*!Y*Z" { MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111} \
	-when "!A*B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010} \
	-when "A*!B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011} \
	-when "!A*!B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010} \
	-when "!A*!B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110} \
	-when "!A*B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011} \
	-when "A*!B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110} \
	-when "A*B*C*!D*!Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111} \
	-when "A*B*C*D*Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010 0011} \
	-when "!A*!B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110 0111} \
	-when "!A*B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010 1011} \
	-when "A*!B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011 1010} \
	-when "A*!B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110 1111} \
	-when "A*B*C*D*!Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111 1110} \
	-when "A*B*C*!D*Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111 0110} \
	-when "!A*B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011 0010} \
	-when "!A*!B*C*!D*Y*!Z" \
	{ MIX }

define_arc \
	-type min_pulse_width \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 $VDD $VDD 0 0 0" \
	-vector {0 0 F 1 X X X} \
	-related_pin C \
	-pin C \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_setup \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 R R X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_hold \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 0 0 0 0" \
	-vector {0 0 R R X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_setup \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 $VDD $VDD 0 0" \
	-vector {0 0 R F X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type non_seq_hold \
	-pinlist {A B C D L Y Z} \
	-ic "0 0 0 $VDD $VDD 0 0" \
	-vector {0 0 R F X X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ MIX }

define_arc \
	-type combinational \
	-pinlist {C D E Z2} \
	-ic "0 0 0 0" \
	-vector {0 0 R R} \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {C D E Z2} \
	-ic "0 0 $VDD $VDD" \
	-vector {0 0 F F} \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {C D E Z2} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {F 1 0 F} \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E Z2} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E Z2} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E Z2} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {F 1 1 1} \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 1 F 1} \
	-pin E \
	{ TRW }

define_arc \
	-type hidden \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R 1} \
	-pin E \
	{ TRW }

define_leakage -when "!C*!D*!E*!Z2" { TRW }

define_leakage -when "!C*!D*E*Z2" { TRW }

define_leakage -when "!C*D*!E*!Z2" { TRW }

define_leakage -when "!C*D*E*Z2" { TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {011 111} \
	-when "C*D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-when "C*D*!E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {001 101} \
	-when "C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {000 100} \
	-when "C*!D*!E*!Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {000 100 110} \
	-when "C*D*!E*!Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {011 111 101} \
	-when "C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {010 110 100} \
	-when "C*!D*!E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {001 101 111} \
	-when "C*D*E*Z2" \
	{ TRW }

define_arc \
	-type min_pulse_width \
	-pinlist {C D E L Z2} \
	-ic "$VDD $VDD 0 0 0" \
	-vector {F 1 0 X X} \
	-related_pin C \
	-pin C \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_setup \
	-pinlist {C D E L Z2} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_hold \
	-pinlist {C D E L Z2} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R F 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_setup \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type non_seq_hold \
	-pinlist {C D E L Z2} \
	-ic "0 0 0 0 0" \
	-vector {R R 0 X X} \
	-related_pin C \
	-pin D \
	-probe {L} \
	{ TRW }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin Q_st \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-pinlist {A Q_st Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin Q_st \
	{ COLL }

define_leakage -when "!A*!Q*!Q_st" { COLL }

define_leakage -when "A*Q*Q_st" { COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {00 10} \
	-when "A*!Q*!Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {00 01} \
	-when "!A*!Q*Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {11 10} \
	-when "A*Q*!Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {11 01} \
	-when "!A*Q*Q_st" \
	{ COLL }

define_arc \
	-type min_pulse_width \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 X} \
	-related_pin A \
	-pin A \
	-probe {Q} \
	{ COLL }

define_arc \
	-type min_pulse_width \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 X} \
	-related_pin A \
	-pin A \
	-probe {Q} \
	{ COLL }

define_arc \
	-type min_pulse_width \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F X} \
	-related_pin Q_st \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type min_pulse_width \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 0" \
	-vector {1 R X} \
	-related_pin Q_st \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_setup \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_hold \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 0" \
	-vector {F R X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_setup \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R F X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type non_seq_hold \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R F X} \
	-related_pin A \
	-pin Q_st \
	-probe {Q} \
	{ COLL }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F F X X} \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X F X} \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X X R} \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 F X X} \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X F X} \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X X R} \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R R X X} \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X R X} \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X X F} \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 R X X} \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X R X} \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X X F} \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 1} \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 0 1} \
	-pin B \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD $VDD $VDD $VDD 0" \
	-vector {F 1 1 1 0} \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD $VDD $VDD $VDD 0" \
	-vector {1 F 1 1 0} \
	-pin B \
	{ C2P }

define_leakage -when "!A*!B*!Q*!Qc*Qn" { C2P }

define_leakage -when "A*B*Q*Qc*!Qn" { C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-when "!A*B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-when "A*!B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-when "A*!B*!Q*!Qc*Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-when "!A*B*!Q*!Qc*Qn" \
	{ C2P }

define_arc \
	-type min_pulse_width \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X X X} \
	-related_pin B \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_setup \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_hold \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {R F X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type min_pulse_width \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X X X} \
	-related_pin A \
	-pin A \
	-probe {Q} \
	{ C2P }

define_arc \
	-type min_pulse_width \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X X X} \
	-related_pin B \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type min_pulse_width \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X X X} \
	-related_pin A \
	-pin A \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_setup \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F R X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type non_seq_hold \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F R X X X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ C2P }

define_arc \
	-type async \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type async \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F 0 F} \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 0 F} \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-pinlist {A B R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin A \
	{ RC2 }

define_leakage -when "!A*!B*!Q*!R" { RC2 }

define_leakage -when "!A*!B*!Q*R" { RC2 }

define_leakage -when "!A*B*!Q*R" { RC2 }

define_leakage -when "A*!B*!Q*R" { RC2 }

define_leakage -when "A*B*Q*!R" { RC2 }

define_leakage -when "A*B*!Q*R" { RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {101 100} \
	-when "A*!B*!Q*!R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {011 010} \
	-when "!A*B*!Q*!R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-when "!A*B*Q*!R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-when "A*!B*Q*!R" \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R F 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R F 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type min_pulse_width \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 X} \
	-related_pin A \
	-pin A \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type min_pulse_width \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F 0 X} \
	-related_pin B \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type min_pulse_width \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 0 X} \
	-related_pin A \
	-pin A \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type min_pulse_width \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 1 R X} \
	-related_pin R \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 F X} \
	-related_pin A \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 F X} \
	-related_pin A \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {F R 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {F R 0 X} \
	-related_pin A \
	-pin B \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_setup \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F F X} \
	-related_pin B \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type non_seq_hold \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F F X} \
	-related_pin B \
	-pin R \
	-probe {Q} \
	{ RC2 }

define_arc \
	-type min_pulse_width \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 X} \
	-related_pin B \
	-pin B \
	-probe {Q} \
	{ RC2 }

