define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DFF_NOCOLLAPSE }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DFF_NOCOLLAPSE }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ UCDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ UCDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-type combinational \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F F X} \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 F X} \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 0" \
	-vector {0 R R X} \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 R X} \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 X R} \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-type edge \
	-pinlist {CLK D M Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 X F} \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D M Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D M Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D M Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {R 1 1 1} \
	-pin CLK \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D M Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D M Q} \
	-vector {1 1 1 1} \
	-when "CLK*D*M*Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D M Q} \
	-vector {0 1 1 1} \
	-when "!CLK*D*M*Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D M Q} \
	-vector {1 0 1 1} \
	-when "CLK*!D*M*Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D M Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D M Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D M Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*M*!Q" \
	{ EMDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10 00} \
	-pinlist {CLK D M Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*!M*Q" \
	{ EMDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 X F} \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 X R} \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {0 R X R} \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-type combinational \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 F X F} \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D Q T} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 F X} \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q T} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 R X} \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q T} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q T} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q T} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin CLK \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q T} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q T} \
	-vector {1 1 1 1} \
	-when "CLK*D*Q*T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q T} \
	-vector {1 0 1 1} \
	-when "CLK*!D*Q*T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q T} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q T} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q T} \
	-vector {0 1 1 1} \
	-when "!CLK*D*Q*T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q T} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*Q*!T" \
	{ TAPDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q T} \
	-vector {0 1 0 1} \
	-when "!CLK*D*!Q*T" \
	{ TAPDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 R} \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 F} \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 1} \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R 1} \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {1 F 0} \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 1} \
	-pin CLK \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ IDFF }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ IDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 F X} \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q Qn} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 X R} \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 R X} \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD 0 $VDD" \
	-vector {R 1 X F} \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 1 0} \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q Qn} \
	-ic "$VDD 0 0 $VDD" \
	-vector {1 R 0 1} \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q Qn} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 1 0} \
	-pin CLK \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q Qn} \
	-vector {1 1 1 0} \
	-when "CLK*D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q Qn} \
	-vector {1 0 0 1} \
	-when "CLK*!D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q Qn} \
	-vector {1 0 1 0} \
	-when "CLK*!D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q Qn} \
	-vector {0 1 1 0} \
	-when "!CLK*D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q Qn} \
	-vector {1 1 0 1} \
	-when "CLK*D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q Qn} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*!Q*Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q Qn} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*Q*!Qn" \
	{ XN }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q Qn} \
	-vector {0 1 0 1} \
	-when "!CLK*D*!Q*Qn" \
	{ XN }

define_arc \
	-type edge \
	-prevector_pinlist {CLK R} \
	-prevector {01 00} \
	-pinlist {CLK R Q} \
	-ic "0 0 0" \
	-vector {R 0 R} \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10} \
	-type async \
	-pinlist {CLK R Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R F} \
	-related_pin R \
	-pin Q \
	{ TFF }

define_arc \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10 00} \
	-type edge \
	-pinlist {CLK R Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {01} \
	-pinlist {CLK R Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 0} \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {01} \
	-pinlist {CLK R Q} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin R \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {11} \
	-pinlist {CLK R Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 0} \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {11 10} \
	-pinlist {CLK R Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin R \
	{ TFF }

define_leakage \
	-pinlist {CLK R Q} \
	-vector {0 1 0} \
	-when "!CLK*!Q*R" \
	{ TFF }

define_leakage \
	-pinlist {CLK R Q} \
	-vector {1 1 0} \
	-when "CLK*!Q*R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {11 10} \
	-pinlist {CLK R Q} \
	-vector {1 0 0} \
	-when "CLK*!Q*!R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {01 00} \
	-pinlist {CLK R Q} \
	-vector {0 0 0} \
	-when "!CLK*!Q*!R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10} \
	-pinlist {CLK R Q} \
	-vector {1 0 1} \
	-when "CLK*Q*!R" \
	{ TFF }

define_leakage \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10 00} \
	-pinlist {CLK R Q} \
	-vector {0 0 1} \
	-when "!CLK*Q*!R" \
	{ TFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DET }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ DET }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ MOR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110 010} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 0 F 0} \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {R 1 1 0} \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {1 1 R 0} \
	-pin R \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 1 0} \
	-when "CLK*!D*!Q*R" \
	{ MOR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 1 0} \
	-when "CLK*D*!Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 1} \
	-when "!CLK*D*Q*R" \
	{ MOR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 1} \
	-when "!CLK*!D*Q*R" \
	{ MOR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type async \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ MORA }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 R 1 0} \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {1 0 F 0} \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 0} \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {1 1 R 0} \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 1 0} \
	-when "CLK*!D*!Q*R" \
	{ MORA }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 1 0} \
	-when "CLK*D*!Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 1} \
	-when "!CLK*D*Q*R" \
	{ MORA }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 1} \
	-when "!CLK*!D*Q*R" \
	{ MORA }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type async \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin R \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 1 0} \
	-when "CLK*!D*!Q*R" \
	{ BR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 1 0} \
	-when "CLK*D*!Q*R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ BR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ BR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 1 R 0} \
	-pin R \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 1 0} \
	-when "CLK*!D*!Q*R" \
	{ SYNCR }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 1 0} \
	-when "CLK*D*!Q*R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ SYNCR }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ SYNCR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 0100} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 R 0 F} \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 0 $VDD" \
	-vector {1 1 0 R F} \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000 0000} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {R 0 1 0 0} \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 R 1 0 0} \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 F 0 0} \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD 0 0" \
	-vector {0 0 1 R 0} \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {0 F 1 0 0} \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010} \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD 0 0" \
	-vector {F 0 1 0 0} \
	-pin CLK \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 0} \
	-when "!CLK*!D*!G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 0} \
	-when "!CLK*!D*G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 1 0} \
	-when "!CLK*!D*G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 0} \
	-when "!CLK*D*!G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 0} \
	-when "!CLK*D*G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 1 0} \
	-when "!CLK*D*G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 0} \
	-when "CLK*!D*!G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 0} \
	-when "CLK*!D*G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 1 0} \
	-when "CLK*!D*G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 0} \
	-when "CLK*D*!G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 0} \
	-when "CLK*D*G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 1 0} \
	-when "CLK*D*G*!Q*R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!D*!G*!Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!D*!G*Q*!R" \
	{ SYNCRG }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!D*!G*Q*!R" \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R 1 F} \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 R} \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {1 1 1 R F} \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD $VDD" \
	-vector {R 0 0 1 F} \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {R 0 1 1 0} \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 1 0} \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F 1 0} \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F 0} \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F 1 1 0} \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 1 1 0} \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {0 1 R 1 0} \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 1010} \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD 0 0" \
	-vector {1 0 1 R 0} \
	-pin G \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 1 0} \
	-when "!CLK*!D*G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 1 0} \
	-when "!CLK*D*G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 1 0} \
	-when "CLK*!D*G*!Q*R" \
	{ GATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 1 0} \
	-when "CLK*D*G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 1} \
	-when "CLK*D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 0} \
	-when "!CLK*D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 0} \
	-when "CLK*!D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 0} \
	-when "CLK*D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 0} \
	-when "CLK*D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 0} \
	-when "CLK*!D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 0} \
	-when "!CLK*!D*G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 0} \
	-when "!CLK*!D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 0} \
	-when "!CLK*D*!G*!Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 1} \
	-when "CLK*D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 1} \
	-when "CLK*!D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 1} \
	-when "!CLK*D*!G*Q*R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 1} \
	-when "CLK*!D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 1} \
	-when "!CLK*D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*D*!G*!Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 1} \
	-when "!CLK*!D*G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!D*!G*Q*!R" \
	{ GATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 1} \
	-when "!CLK*!D*!G*Q*R" \
	{ GATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {R 1 0 1 R} \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-type async \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {1 1 1 R F} \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-type async \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 R 1 F} \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001 0001} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-ic "0 0 0 $VDD $VDD" \
	-vector {R 0 0 1 F} \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {R 0 1 1 0} \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 R 1 1 0} \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 F 1 0} \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F 0} \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F 1 1 0} \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 1 1 0} \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {0 1 R 1 0} \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-ic "$VDD $VDD $VDD 0 0" \
	-vector {1 1 1 R 0} \
	-pin G \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 1 0} \
	-when "!CLK*!D*G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 1 0} \
	-when "!CLK*D*G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 1 0} \
	-when "CLK*!D*G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 1 0} \
	-when "CLK*D*G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 0} \
	-when "!CLK*D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 0} \
	-when "CLK*D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 0} \
	-when "!CLK*!D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 0} \
	-when "CLK*!D*G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 0} \
	-when "CLK*D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 0} \
	-when "CLK*!D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 0 1} \
	-when "CLK*D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 1 1} \
	-when "CLK*D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 0} \
	-when "!CLK*!D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 0} \
	-when "!CLK*D*!G*!Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 0 1} \
	-when "!CLK*D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 1 1} \
	-when "!CLK*D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*D*!G*!Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 1 1} \
	-when "CLK*!D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!D*!G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 0 1} \
	-when "CLK*!D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 1 1} \
	-when "!CLK*!D*G*Q*!R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 0 1} \
	-when "!CLK*!D*!G*Q*R" \
	{ AGATEDR }

define_leakage \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!D*!G*Q*!R" \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 0} \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 0} \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 0 1 0} \
	-when "!CLK*!D*!Q*R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {0 1 1 0} \
	-when "!CLK*D*!Q*R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 0 1 0} \
	-when "CLK*!D*!Q*R" \
	{ RDFF }

define_leakage \
	-pinlist {CLK D R Q} \
	-vector {1 1 1 0} \
	-when "CLK*D*!Q*R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 0} \
	-when "!CLK*!D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 1} \
	-when "CLK*D*Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 0} \
	-when "CLK*!D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 0 0} \
	-when "CLK*D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 0} \
	-when "!CLK*D*!Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 0 1} \
	-when "CLK*!D*Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 0 1} \
	-when "!CLK*D*Q*!R" \
	{ RDFF }

define_leakage \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 0 1} \
	-when "!CLK*!D*Q*!R" \
	{ RDFF }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010} \
	-type async \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 1 R F} \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-prevector_pinlist {CLK D B R} \
	-prevector {0011} \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F R} \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D B R} \
	-prevector {0000 1000} \
	-pinlist {CLK D B R Q} \
	-ic "$VDD 0 0 0 0" \
	-vector {1 0 R 0 R} \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-type edge \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {R 0 0 1 0} \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 R 0 1 0} \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101} \
	-pinlist {CLK D B R Q} \
	-ic "0 $VDD 0 $VDD 0" \
	-vector {0 F 0 1 0} \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001} \
	-pinlist {CLK D B R Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {F 0 0 1 0} \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010} \
	-pinlist {CLK D B R Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F 0 1} \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0000 1000} \
	-pinlist {CLK D B R Q} \
	-ic "$VDD 0 0 0 0" \
	-vector {1 0 0 R 0} \
	-pin R \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 1 0} \
	-when "!B*!CLK*!D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 1 0} \
	-when "!B*!CLK*D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 1 0} \
	-when "!B*CLK*!D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 1 0} \
	-when "!B*CLK*D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 0 1 0 1} \
	-when "B*!CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 0 1 1 0} \
	-when "B*!CLK*!D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 1 1 0 1} \
	-when "B*!CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {0 1 1 1 0} \
	-when "B*!CLK*D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 0 1 0 1} \
	-when "B*CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 0 1 1 0} \
	-when "B*CLK*!D*!Q*R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 1 1 0 1} \
	-when "B*CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-pinlist {CLK D B R Q} \
	-vector {1 1 1 1 0} \
	-when "B*CLK*D*!Q*R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0000 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 0 0} \
	-when "!B*CLK*!D*!Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 0 0} \
	-when "!B*!CLK*!D*!Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 0 1} \
	-when "!B*!CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 0 1} \
	-when "!B*!CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 0 0} \
	-when "!B*!CLK*D*!Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {0100 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 0 1} \
	-when "!B*CLK*D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 0 1} \
	-when "!B*CLK*!D*Q*!R" \
	{ COEX }

define_leakage \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 0 0} \
	-when "!B*CLK*D*!Q*!R" \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 1 R F} \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0011} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD $VDD 0" \
	-vector {0 0 1 F R} \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {R 1 0 0 R} \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 R 0 R} \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-type edge \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 F} \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {R 0 1 0 1} \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 R 1 0 1} \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 0 F 0 1} \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD $VDD 0 $VDD" \
	-vector {0 F 1 0 1} \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 R 1 0} \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 0 0 $VDD 0" \
	-vector {0 0 0 F 0} \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {F 0 1 0 1} \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-ic "0 $VDD 0 0 0" \
	-vector {0 1 0 R 0} \
	-pin CLR \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 1 0 1} \
	-when "!CLK*!CLR*!D*PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 1 0 1} \
	-when "!CLK*!CLR*D*PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 1 0} \
	-when "!CLK*CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 1 1 0} \
	-when "!CLK*CLR*!D*PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 1 0} \
	-when "!CLK*CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 1 1 0} \
	-when "!CLK*CLR*D*PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 1 0 1} \
	-when "CLK*!CLR*!D*PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 1 0 1} \
	-when "CLK*!CLR*D*PRE*Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 1 0} \
	-when "CLK*CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 1 1 0} \
	-when "CLK*CLR*!D*PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 1 0} \
	-when "CLK*CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 1 1 0} \
	-when "CLK*CLR*D*PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 0 0} \
	-when "!CLK*!CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0100 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 0 1} \
	-when "CLK*!CLR*D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 0 0} \
	-when "!CLK*!CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0000 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 0 0} \
	-when "CLK*!CLR*!D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 0 1} \
	-when "CLK*!CLR*!D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 0 1} \
	-when "!CLK*!CLR*!D*!PRE*Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 0 0} \
	-when "CLK*!CLR*D*!PRE*!Q" \
	{ CAFF }

define_leakage \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 0 1} \
	-when "!CLK*!CLR*D*!PRE*Q" \
	{ CAFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ DLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 0} \
	-when "!CLK*D*!Q" \
	{ DLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ DLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {EN D} \
	-prevector {10} \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-prevector_pinlist {EN D} \
	-prevector {11} \
	-type combinational \
	-pinlist {EN D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type edge \
	-prevector_pinlist {EN D} \
	-prevector {10 00 01} \
	-pinlist {EN D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-prevector_pinlist {EN D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {EN D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10} \
	-pinlist {EN D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10 00} \
	-pinlist {EN D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10 00} \
	-pinlist {EN D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {11 01} \
	-pinlist {EN D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT_EN }

define_leakage \
	-pinlist {EN D Q} \
	-vector {1 0 0} \
	-when "!D*EN*!Q" \
	{ DLAT_EN }

define_leakage \
	-pinlist {EN D Q} \
	-vector {1 1 1} \
	-when "D*EN*Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {10 00} \
	-pinlist {EN D Q} \
	-vector {0 0 0} \
	-when "!D*!EN*!Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {11 01} \
	-pinlist {EN D Q} \
	-vector {0 1 1} \
	-when "D*!EN*Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {10 00 01} \
	-pinlist {EN D Q} \
	-vector {0 1 0} \
	-when "D*!EN*!Q" \
	{ DLAT_EN }

define_leakage \
	-prevector_pinlist {EN D} \
	-prevector {11 01 00} \
	-pinlist {EN D Q} \
	-vector {0 0 1} \
	-when "!D*!EN*Q" \
	{ DLAT_EN }

define_arc \
	-type combinational \
	-prevector_pinlist {E D} \
	-prevector {10} \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-prevector_pinlist {E D} \
	-prevector {11} \
	-type combinational \
	-pinlist {E D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-prevector_pinlist {E D} \
	-prevector {11 01 00} \
	-type combinational \
	-pinlist {E D Q} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-prevector_pinlist {E D} \
	-prevector {10 00 01} \
	-pinlist {E D Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {10} \
	-pinlist {E D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {11 01} \
	-pinlist {E D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {11 01} \
	-pinlist {E D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F 1} \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {10 00} \
	-pinlist {E D Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ DLAT_E }

define_leakage \
	-pinlist {E D Q} \
	-vector {1 0 0} \
	-when "!D*E*!Q" \
	{ DLAT_E }

define_leakage \
	-pinlist {E D Q} \
	-vector {1 1 1} \
	-when "D*E*Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {11 01} \
	-pinlist {E D Q} \
	-vector {0 1 1} \
	-when "D*!E*Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {10 00} \
	-pinlist {E D Q} \
	-vector {0 0 0} \
	-when "!D*!E*!Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {11 01 00} \
	-pinlist {E D Q} \
	-vector {0 0 1} \
	-when "!D*!E*Q" \
	{ DLAT_E }

define_leakage \
	-prevector_pinlist {E D} \
	-prevector {10 00 01} \
	-pinlist {E D Q} \
	-vector {0 1 0} \
	-when "D*!E*!Q" \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R 1} \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {R 1 1} \
	-pin CLK \
	{ GLAT }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ GLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ GLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ GLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 0 1} \
	-when "!CLK*!D*Q" \
	{ GLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 1 R R} \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 1 F F} \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 0} \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD $VDD" \
	-vector {0 0 F 1} \
	-pin D \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin D \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 0 0} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 1 1} \
	-when "!CLKA*CLKB*D*Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MUXLAT }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MUXLAT }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MUXLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 1 R R} \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD $VDD $VDD" \
	-vector {0 1 F F} \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {F 1 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {0 R 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {F 1 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 R 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 0} \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD 0 0" \
	-vector {F 1 0 0} \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 1 F 1} \
	-pin D \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 0 0} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 1 1} \
	-when "!CLKA*CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ MCDFF }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ MCDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {0 R R} \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-type combinational \
	-pinlist {CLK D Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 R} \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-ic "$VDD 0 0" \
	-vector {1 R 0} \
	-pin D \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin D \
	{ TCASC }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 0 0} \
	-when "!CLK*!D*!Q" \
	{ TCASC }

define_leakage \
	-pinlist {CLK D Q} \
	-vector {0 1 1} \
	-when "!CLK*D*Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 0} \
	-when "CLK*!D*!Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 1} \
	-when "CLK*D*Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 1 0} \
	-when "CLK*D*!Q" \
	{ TCASC }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 0 1} \
	-when "CLK*!D*Q" \
	{ TCASC }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD 0" \
	-vector {1 F R} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D T} \
	-ic "0 0 0" \
	-vector {0 R R} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-type edge \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F F} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type combinational \
	-pinlist {CLK D T} \
	-ic "$VDD 0 $VDD" \
	-vector {1 R F} \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {CLK D T} \
	-ic "0 0 $VDD" \
	-vector {R 0 F} \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D T} \
	-ic "$VDD $VDD 0" \
	-vector {F 1 0} \
	-pin CLK \
	{ XLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D T} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D T} \
	-vector {1 1 0} \
	-when "CLK*D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D T} \
	-vector {0 0 0} \
	-when "!CLK*!D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D T} \
	-vector {1 0 0} \
	-when "CLK*!D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D T} \
	-vector {0 1 0} \
	-when "!CLK*D*!T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D T} \
	-vector {1 1 1} \
	-when "CLK*D*T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D T} \
	-vector {1 0 1} \
	-when "CLK*!D*T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D T} \
	-vector {0 0 1} \
	-when "!CLK*!D*T" \
	{ XLAT }

define_leakage \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D T} \
	-vector {0 1 1} \
	-when "!CLK*D*T" \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 $VDD" \
	-vector {R 0 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101 001} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 $VDD 0" \
	-vector {R 0 1 R} \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 F 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110 010 110} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 F 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {F 0 0 0} \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 0} \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 0 R 0} \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {1 0 F 1} \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111} \
	-pinlist {CLKA CLKB D Q} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 F 1 1} \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000} \
	-pinlist {CLKA CLKB D Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin CLKA \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 0} \
	-when "CLKA*!CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 1} \
	-when "CLKA*!CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 0} \
	-when "!CLKA*!CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 1 0} \
	-when "CLKA*!CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 0 1} \
	-when "CLKA*!CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 1} \
	-when "!CLKA*!CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 1 1} \
	-when "!CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 0 1} \
	-when "!CLKA*!CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 1 0} \
	-when "!CLKA*!CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 0 0} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 1 0} \
	-when "!CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 0 1} \
	-when "!CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 0} \
	-when "CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110 010 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 1} \
	-when "CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 0 0} \
	-when "CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110 010 110 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 0 1} \
	-when "!CLKA*CLKB*!D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110 010 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 1 1} \
	-when "CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 1 0} \
	-when "!CLKA*CLKB*D*!Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100 110 010 110 111 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 1 1} \
	-when "!CLKA*CLKB*D*Q" \
	{ HPIPE }

define_leakage \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 010 011 111 011 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 0 0} \
	-when "!CLKA*CLKB*!D*!Q" \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 $VDD 0" \
	-vector {1 R 0 1 R} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD $VDD 0 0" \
	-vector {R 1 1 0 R} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 0 0 $VDD $VDD" \
	-vector {R 0 0 1 F} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 0 $VDD 0 $VDD" \
	-vector {0 R 1 0 F} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110} \
	-type combinational \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD $VDD $VDD 0 $VDD" \
	-vector {F 1 1 0 F} \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101} \
	-type combinational \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD $VDD 0 $VDD $VDD" \
	-vector {1 F 0 1 F} \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {F 0 1 0 1} \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {1 R 1 0 1} \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {1 0 F 0 1} \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 $VDD 0 $VDD" \
	-vector {1 0 1 R 1} \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 $VDD $VDD $VDD" \
	-vector {1 0 1 F 1} \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "$VDD 0 0 0 0" \
	-vector {1 0 R 0 0} \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {R 1 1 1 1} \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-ic "0 $VDD $VDD $VDD $VDD" \
	-vector {0 F 1 1 1} \
	-pin CLKB \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 0 1} \
	-when "CLKA*!CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 1 1} \
	-when "CLKA*!CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 0 0} \
	-when "CLKA*!CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 1 1} \
	-when "!CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 0 0} \
	-when "!CLKA*CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 1 1} \
	-when "!CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 1 0} \
	-when "CLKA*!CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 0 0} \
	-when "!CLKA*CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 1 1} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 1 1} \
	-when "!CLKA*!CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 0 0} \
	-when "CLKA*!CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 1 1} \
	-when "!CLKA*!CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 0 0} \
	-when "!CLKA*!CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 0 1} \
	-when "!CLKA*!CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 0 1} \
	-when "!CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 0 0} \
	-when "CLKA*CLKB*!DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 1 1} \
	-when "CLKA*!CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 0 1} \
	-when "!CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 1 0} \
	-when "!CLKA*!CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 0 1} \
	-when "CLKA*!CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 1 0} \
	-when "!CLKA*CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 0 0} \
	-when "!CLKA*!CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 1 0} \
	-when "CLKA*!CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 0 1} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 1 1} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 1 0} \
	-when "!CLKA*CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 0 1} \
	-when "!CLKA*!CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 0 1} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 1 1} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 1100 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 0 0} \
	-when "CLKA*CLKB*DA*!DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 1 1} \
	-when "CLKA*CLKB*DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 1 0} \
	-when "!CLKA*!CLKB*DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 1 0} \
	-when "CLKA*CLKB*!DA*DB*!Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 0 1} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 0 1} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 1 1} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001 1101 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 0 1} \
	-when "CLKA*CLKB*!DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 1110 1111 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 1 1} \
	-when "CLKA*CLKB*!DA*DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101 1100 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 0 1} \
	-when "CLKA*CLKB*DA*!DB*Q" \
	{ DCMUX }

define_leakage \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101 1101 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 1 0} \
	-when "CLKA*CLKB*DA*DB*!Q" \
	{ DCMUX }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK EN} \
	-prevector {01} \
	-pinlist {CLK EN GCLK} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11} \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00} \
	-pinlist {CLK EN GCLK} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin CLK \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00} \
	-pinlist {CLK EN GCLK} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {01} \
	-pinlist {CLK EN GCLK} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10} \
	-pinlist {CLK EN GCLK} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin CLK \
	{ ICG }

define_leakage \
	-pinlist {CLK EN GCLK} \
	-vector {0 0 0} \
	-when "!CLK*!EN*!GCLK" \
	{ ICG }

define_leakage \
	-pinlist {CLK EN GCLK} \
	-vector {0 1 0} \
	-when "!CLK*EN*!GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11} \
	-pinlist {CLK EN GCLK} \
	-vector {1 1 1} \
	-when "CLK*EN*GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10} \
	-pinlist {CLK EN GCLK} \
	-vector {1 0 0} \
	-when "CLK*!EN*!GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11 10} \
	-pinlist {CLK EN GCLK} \
	-vector {1 0 1} \
	-when "CLK*!EN*GCLK" \
	{ ICG }

define_leakage \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10 11} \
	-pinlist {CLK EN GCLK} \
	-vector {1 1 0} \
	-when "CLK*EN*!GCLK" \
	{ ICG }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 $VDD 0 $VDD 0" \
	-vector {1 R 1 0 1 R} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {R 0 0 1 0 R} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 0 $VDD 0 $VDD" \
	-vector {F 0 0 1 0 F} \
	-related_pin CLKA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 0 $VDD 0 $VDD" \
	-vector {1 0 R 1 0 F} \
	-related_pin RA \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 F 1 0 1 F} \
	-related_pin CLKB \
	-pin GCLK \
	{ ICM }

define_arc \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-type combinational \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD $VDD $VDD 0 $VDD $VDD" \
	-vector {1 1 1 R 1 F} \
	-related_pin RB \
	-pin GCLK \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {R 0 1 1 0 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 R 1 1 0 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 0 F 1 0 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 0 1 F 0 0} \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD 0 0" \
	-vector {0 0 1 1 R 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 0 $VDD $VDD $VDD 0" \
	-vector {0 0 1 1 F 0} \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 $VDD $VDD $VDD 0 0" \
	-vector {0 F 1 1 0 0} \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 $VDD $VDD 0 0" \
	-vector {F 0 1 1 0 0} \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "$VDD 0 0 $VDD 0 0" \
	-vector {1 0 R 1 0 0} \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-ic "0 $VDD $VDD 0 $VDD 0" \
	-vector {0 1 1 R 1 0} \
	-pin RB \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 1 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*RB*S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 1 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*RB*!S" \
	{ ICM }

define_leakage \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 1 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11111 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01100 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01111 01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {11110 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10110 10100 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 0 0} \
	-when "CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 10011 00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 10101 11101 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 0 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01110 01010 11010 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10111 10101 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 0 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 10011 00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 0 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 0 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 10010 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01001 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 10011 00011 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 10011 00011 10011 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 00100 01100 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 0 1 1} \
	-when "!CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 00010 00000 10000 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 0 1 1} \
	-when "CLKA*CLKB*GCLK*RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 11010 11011 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 10011 00011 10011 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 01010 01000 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01001 01000 00000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11001 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011 11010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 1 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 00101 01101 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 0} \
	-when "CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 01101 01001 01000 00000 01000 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010 10010 10011 00011 10011 10001 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 11010 11011 01011 11011 11010 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 11101 11100 10100 11100 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 00000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 00001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 00001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 0 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 1 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 0 1} \
	-when "!CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 11001 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 1 1} \
	-when "CLKA*!CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 10000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 00001 01001 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 10000 11000 01000 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00111 00101 01101 01001 01000 11000 01000 11000 10000 11000 11001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 1 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*S" \
	{ ICM }

define_leakage \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00110 00010 00000 10000 10001 11001 10001 11001 01001 11001 11000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 0 0 0 1} \
	-when "CLKA*CLKB*GCLK*!RA*!RB*!S" \
	{ ICM }

define_arc \
	-type combinational \
	-prevector_pinlist {C D} \
	-prevector {01} \
	-pinlist {C D Y} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-prevector_pinlist {C D} \
	-prevector {01 11} \
	-type combinational \
	-pinlist {C D Y} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 F} \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00} \
	-pinlist {C D Y} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin C \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00} \
	-pinlist {C D Y} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {01} \
	-pinlist {C D Y} \
	-ic "0 $VDD 0" \
	-vector {0 F 0} \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00 10} \
	-pinlist {C D Y} \
	-ic "$VDD 0 0" \
	-vector {F 0 0} \
	-pin C \
	{ GL }

define_leakage \
	-pinlist {C D Y} \
	-vector {0 0 0} \
	-when "!C*!D*!Y" \
	{ GL }

define_leakage \
	-pinlist {C D Y} \
	-vector {0 1 0} \
	-when "!C*D*!Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {01 11} \
	-pinlist {C D Y} \
	-vector {1 1 1} \
	-when "C*D*Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {00 10} \
	-pinlist {C D Y} \
	-vector {1 0 0} \
	-when "C*!D*!Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {01 11 10} \
	-pinlist {C D Y} \
	-vector {1 0 1} \
	-when "C*!D*Y" \
	{ GL }

define_leakage \
	-prevector_pinlist {C D} \
	-prevector {00 10 11} \
	-pinlist {C D Y} \
	-vector {1 1 0} \
	-when "C*D*!Y" \
	{ GL }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0001} \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {0 0 R 1 R X} \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0100} \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {R 1 0 0 X R} \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1000} \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {1 R 0 0 X R} \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1100} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {F 1 0 0 X F} \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1100} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD 0 0 0 $VDD" \
	-vector {1 F 0 0 X F} \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 $VDD $VDD $VDD 0" \
	-vector {1 0 F 1 F X} \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {R 0 0 0 0 0} \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 R 0 0 0 0} \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 0 R 0 0 0} \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 0 0 0" \
	-vector {0 0 0 R 0 0} \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001} \
	-pinlist {A B C D Y Z} \
	-ic "0 0 0 $VDD 0 0" \
	-vector {0 0 0 F 0 0} \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100} \
	-pinlist {A B C D Y Z} \
	-ic "0 $VDD 0 0 0 0" \
	-vector {0 F 0 0 0 0} \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000} \
	-pinlist {A B C D Y Z} \
	-ic "$VDD 0 0 0 0 0" \
	-vector {F 0 0 0 0 0} \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110} \
	-pinlist {A B C D Y Z} \
	-ic "$VDD $VDD $VDD 0 0 $VDD" \
	-vector {1 1 F 0 0 1} \
	-pin C \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 0 0 0 0 0} \
	-when "!A*!B*!C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 0 0 1 0 0} \
	-when "!A*!B*!C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 1 0 0 0 0} \
	-when "!A*B*!C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {0 1 0 1 0 0} \
	-when "!A*B*!C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 0 0 0 0 0} \
	-when "A*!B*!C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 0 0 1 0 0} \
	-when "A*!B*!C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 1 0 0 0 1} \
	-when "A*B*!C*!D*!Y*Z" \
	{ MIX }

define_leakage \
	-pinlist {A B C D Y Z} \
	-vector {1 1 0 1 0 1} \
	-when "A*B*!C*D*!Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 1 1 0} \
	-when "A*!B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 1 1 0} \
	-when "!A*B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 0 0 1} \
	-when "A*B*C*!D*!Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 1 1 1} \
	-when "A*B*C*D*Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 0 0 0} \
	-when "A*!B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 1 1 0} \
	-when "!A*!B*C*D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 0 0 0} \
	-when "!A*!B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 0 0 0} \
	-when "!A*B*C*!D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010 1011} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 1 0 0} \
	-when "A*!B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 0 1 0} \
	-when "!A*!B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011 1010} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 0 1 0} \
	-when "A*!B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111 1110} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 0 1 1} \
	-when "A*B*C*!D*Y*Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 1 0 0} \
	-when "!A*B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 1 0 0} \
	-when "!A*!B*C*D*!Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 0 1 0} \
	-when "!A*B*C*!D*Y*!Z" \
	{ MIX }

define_leakage \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110 1111} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 1 0 1} \
	-when "A*B*C*D*!Y*Z" \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {000} \
	-pinlist {C D E Z2} \
	-ic "0 0 0 0" \
	-vector {0 0 R R} \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {001} \
	-type combinational \
	-pinlist {C D E Z2} \
	-ic "0 0 $VDD $VDD" \
	-vector {0 0 F F} \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {010} \
	-pinlist {C D E Z2} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {F 1 0 F} \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000} \
	-pinlist {C D E Z2} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000} \
	-pinlist {C D E Z2} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010} \
	-pinlist {C D E Z2} \
	-ic "0 $VDD 0 0" \
	-vector {0 F 0 0} \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {001 101} \
	-pinlist {C D E Z2} \
	-ic "$VDD 0 $VDD $VDD" \
	-vector {F 0 1 1} \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R 1} \
	-pin E \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {011 111} \
	-pinlist {C D E Z2} \
	-ic "$VDD $VDD $VDD $VDD" \
	-vector {1 1 F 1} \
	-pin E \
	{ TRW }

define_leakage \
	-pinlist {C D E Z2} \
	-vector {0 0 0 0} \
	-when "!C*!D*!E*!Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E Z2} \
	-vector {0 0 1 1} \
	-when "!C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E Z2} \
	-vector {0 1 0 0} \
	-when "!C*D*!E*!Z2" \
	{ TRW }

define_leakage \
	-pinlist {C D E Z2} \
	-vector {0 1 1 1} \
	-when "!C*D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {001 101} \
	-pinlist {C D E Z2} \
	-vector {1 0 1 1} \
	-when "C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {000 100} \
	-pinlist {C D E Z2} \
	-vector {1 0 0 0} \
	-when "C*!D*!E*!Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-pinlist {C D E Z2} \
	-vector {1 1 0 1} \
	-when "C*D*!E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {011 111} \
	-pinlist {C D E Z2} \
	-vector {1 1 1 1} \
	-when "C*D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {011 111 101} \
	-pinlist {C D E Z2} \
	-vector {1 0 1 1} \
	-when "C*!D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {001 101 111} \
	-pinlist {C D E Z2} \
	-vector {1 1 1 1} \
	-when "C*D*E*Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {000 100 110} \
	-pinlist {C D E Z2} \
	-vector {1 1 0 0} \
	-when "C*D*!E*!Z2" \
	{ TRW }

define_leakage \
	-prevector_pinlist {C D E} \
	-prevector {010 110 100} \
	-pinlist {C D E Z2} \
	-vector {1 0 0 1} \
	-when "C*!D*!E*Z2" \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {A Q_st} \
	-prevector {00 10} \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 0" \
	-vector {1 R R} \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-prevector_pinlist {A Q_st} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "$VDD 0 $VDD" \
	-vector {F 0 F} \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-prevector_pinlist {A Q_st} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD $VDD" \
	-vector {0 F F} \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-prevector_pinlist {A Q_st} \
	-prevector {00 01} \
	-pinlist {A Q_st Q} \
	-ic "0 $VDD 0" \
	-vector {R 1 R} \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {00} \
	-pinlist {A Q_st Q} \
	-ic "0 0 0" \
	-vector {R 0 0} \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {00} \
	-pinlist {A Q_st Q} \
	-ic "0 0 0" \
	-vector {0 R 0} \
	-pin Q_st \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {11} \
	-pinlist {A Q_st Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {F 1 1} \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {11} \
	-pinlist {A Q_st Q} \
	-ic "$VDD $VDD $VDD" \
	-vector {1 F 1} \
	-pin Q_st \
	{ COLL }

define_leakage \
	-pinlist {A Q_st Q} \
	-vector {0 0 0} \
	-when "!A*!Q*!Q_st" \
	{ COLL }

define_leakage \
	-pinlist {A Q_st Q} \
	-vector {1 1 1} \
	-when "A*Q*Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {00 10} \
	-pinlist {A Q_st Q} \
	-vector {1 0 0} \
	-when "A*!Q*!Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {11 10} \
	-pinlist {A Q_st Q} \
	-vector {1 0 1} \
	-when "A*Q*!Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {11 01} \
	-pinlist {A Q_st Q} \
	-vector {0 1 1} \
	-when "!A*Q*Q_st" \
	{ COLL }

define_leakage \
	-prevector_pinlist {A Q_st} \
	-prevector {00 01} \
	-pinlist {A Q_st Q} \
	-vector {0 1 0} \
	-when "!A*!Q*Q_st" \
	{ COLL }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F F X X} \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X F X} \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD $VDD $VDD 0" \
	-vector {0 F X X R} \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 F X X} \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X F X} \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 $VDD $VDD 0" \
	-vector {F 0 X X R} \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 R X X} \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X R X} \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "0 $VDD 0 0 $VDD" \
	-vector {R 1 X X F} \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R R X X} \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X R X} \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD 0 0 0 $VDD" \
	-vector {1 R X X F} \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q Qc Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {R 0 0 0 1} \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q Qc Qn} \
	-ic "0 0 0 0 $VDD" \
	-vector {0 R 0 0 1} \
	-pin B \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD $VDD $VDD $VDD 0" \
	-vector {F 1 1 1 0} \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q Qc Qn} \
	-ic "$VDD $VDD $VDD $VDD 0" \
	-vector {1 F 1 1 0} \
	-pin B \
	{ C2P }

define_leakage \
	-pinlist {A B Q Qc Qn} \
	-vector {0 0 0 0 1} \
	-when "!A*!B*!Q*!Qc*Qn" \
	{ C2P }

define_leakage \
	-pinlist {A B Q Qc Qn} \
	-vector {1 1 1 1 0} \
	-when "A*B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {0 1 1 1 0} \
	-when "!A*B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {1 0 1 1 0} \
	-when "A*!B*Q*Qc*!Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {0 1 0 0 1} \
	-when "!A*B*!Q*!Qc*Qn" \
	{ C2P }

define_leakage \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {1 0 0 0 1} \
	-when "A*!B*!Q*!Qc*Qn" \
	{ C2P }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-type async \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD 0 $VDD" \
	-vector {1 1 R F} \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type async \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-ic "$VDD $VDD $VDD 0" \
	-vector {1 1 F R} \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {101 100} \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 0" \
	-vector {1 R 0 R} \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "$VDD 0 0 $VDD" \
	-vector {F 0 0 F} \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-type combinational \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 $VDD" \
	-vector {0 F 0 F} \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {011 010} \
	-pinlist {A B R Q} \
	-ic "0 $VDD 0 0" \
	-vector {R 1 0 R} \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {R 0 0 0} \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 R 0 0} \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-ic "0 0 0 0" \
	-vector {0 0 R 0} \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-ic "0 0 $VDD 0" \
	-vector {0 0 F 0} \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-ic "0 $VDD $VDD 0" \
	-vector {0 F 1 0} \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-ic "$VDD 0 $VDD 0" \
	-vector {F 0 1 0} \
	-pin A \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 0 0 0} \
	-when "!A*!B*!Q*!R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 0 1 0} \
	-when "!A*!B*!Q*R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {0 1 1 0} \
	-when "!A*B*!Q*R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 0 1 0} \
	-when "A*!B*!Q*R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 1 0 1} \
	-when "A*B*Q*!R" \
	{ RC2 }

define_leakage \
	-pinlist {A B R Q} \
	-vector {1 1 1 0} \
	-when "A*B*!Q*R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {101 100} \
	-pinlist {A B R Q} \
	-vector {1 0 0 0} \
	-when "A*!B*!Q*!R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-pinlist {A B R Q} \
	-vector {1 0 0 1} \
	-when "A*!B*Q*!R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-pinlist {A B R Q} \
	-vector {0 1 0 1} \
	-when "!A*B*Q*!R" \
	{ RC2 }

define_leakage \
	-prevector_pinlist {A B R} \
	-prevector {011 010} \
	-pinlist {A B R Q} \
	-vector {0 1 0 0} \
	-when "!A*B*!Q*!R" \
	{ RC2 }

