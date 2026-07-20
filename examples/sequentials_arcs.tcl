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
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type combinational \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 0} \
	-when "D*!Q" \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ DFF_NOCOLLAPSE }

define_leakage -when "!CLK*!D" { DFF_NOCOLLAPSE }
define_leakage -when "!CLK*D" { DFF_NOCOLLAPSE }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type combinational \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 0} \
	-when "D*!Q" \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin D \
	{ UCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ UCDFF }

define_leakage -when "!CLK*!D" { UCDFF }
define_leakage -when "!CLK*D" { UCDFF }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D M Q} \
	-vector {F 0 F X} \
	-when "!D" \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D M Q} \
	-vector {F 1 R X} \
	-when "D" \
	-related_pin CLK \
	-pin M \
	{ EMDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-type combinational \
	-pinlist {CLK D M Q} \
	-vector {0 F F X} \
	-when "!CLK" \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D M Q} \
	-vector {0 R R X} \
	-when "!CLK" \
	-related_pin D \
	-pin M \
	{ EMDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D M Q} \
	-vector {R 0 X F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D M Q} \
	-vector {R 1 X R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D M Q} \
	-vector {F 0 0 0} \
	-when "!D*!M*!Q" \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D M Q} \
	-vector {F 1 1 1} \
	-when "D*M*Q" \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D M Q} \
	-vector {R 0 0 0} \
	-when "!D*!M*!Q" \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D M Q} \
	-vector {R 1 1 1} \
	-when "D*M*Q" \
	-pin CLK \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D M Q} \
	-vector {1 F 0 0} \
	-when "CLK*!M*!Q" \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D M Q} \
	-vector {1 F 1 1} \
	-when "CLK*M*Q" \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D M Q} \
	-vector {1 R 0 0} \
	-when "CLK*!M*!Q" \
	-pin D \
	{ EMDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D M Q} \
	-vector {1 R 1 1} \
	-when "CLK*M*Q" \
	-pin D \
	{ EMDFF }

define_leakage -when "!CLK*!D*!M" { EMDFF }
define_leakage -when "!CLK*D*M" { EMDFF }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D Q T} \
	-vector {R 0 F X} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q T} \
	-vector {R 1 R X} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ TAPDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D Q T} \
	-vector {F 0 X F} \
	-when "!D" \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q T} \
	-vector {F 1 X R} \
	-when "D" \
	-related_pin CLK \
	-pin T \
	{ TAPDFF }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-type combinational \
	-pinlist {CLK D Q T} \
	-vector {0 F X F} \
	-when "!CLK" \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q T} \
	-vector {0 R X R} \
	-when "!CLK" \
	-related_pin D \
	-pin T \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q T} \
	-vector {F 0 0 0} \
	-when "!D*!Q*!T" \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q T} \
	-vector {F 1 1 1} \
	-when "D*Q*T" \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q T} \
	-vector {R 0 0 0} \
	-when "!D*!Q*!T" \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q T} \
	-vector {R 1 1 1} \
	-when "D*Q*T" \
	-pin CLK \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q T} \
	-vector {1 F 0 0} \
	-when "CLK*!Q*!T" \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q T} \
	-vector {1 F 1 1} \
	-when "CLK*Q*T" \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q T} \
	-vector {1 R 0 0} \
	-when "CLK*!Q*!T" \
	-pin D \
	{ TAPDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q T} \
	-vector {1 R 1 1} \
	-when "CLK*Q*T" \
	-pin D \
	{ TAPDFF }

define_leakage -when "!CLK*!D*!T" { TAPDFF }
define_leakage -when "!CLK*D*T" { TAPDFF }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-type edge \
	-pinlist {CLK D Q} \
	-vector {R 1 F} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 R} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 0} \
	-when "D*!Q" \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 0} \
	-when "D*!Q" \
	-pin CLK \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin D \
	{ IDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ IDFF }

define_leakage -when "!CLK*!D" { IDFF }
define_leakage -when "!CLK*D" { IDFF }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-vector {R 0 F X} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q Qn} \
	-vector {R 1 R X} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ XN }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-type edge \
	-pinlist {CLK D Q Qn} \
	-vector {R 1 X F} \
	-when "D" \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q Qn} \
	-vector {R 0 X R} \
	-when "!D" \
	-related_pin CLK \
	-pin Qn \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q Qn} \
	-vector {F 0 0 1} \
	-when "!D*!Q*Qn" \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q Qn} \
	-vector {F 0 1 0} \
	-when "!D*Q*!Qn" \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q Qn} \
	-vector {F 1 0 1} \
	-when "D*!Q*Qn" \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q Qn} \
	-vector {F 1 1 0} \
	-when "D*Q*!Qn" \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q Qn} \
	-vector {R 0 0 1} \
	-when "!D*!Q*Qn" \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q Qn} \
	-vector {R 1 1 0} \
	-when "D*Q*!Qn" \
	-pin CLK \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00 01} \
	-pinlist {CLK D Q Qn} \
	-vector {0 F 0 1} \
	-when "!CLK*!Q*Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01} \
	-pinlist {CLK D Q Qn} \
	-vector {0 F 1 0} \
	-when "!CLK*Q*!Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q Qn} \
	-vector {1 F 0 1} \
	-when "CLK*!Q*Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q Qn} \
	-vector {1 F 1 0} \
	-when "CLK*Q*!Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 00} \
	-pinlist {CLK D Q Qn} \
	-vector {0 R 0 1} \
	-when "!CLK*!Q*Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 01 00} \
	-pinlist {CLK D Q Qn} \
	-vector {0 R 1 0} \
	-when "!CLK*Q*!Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q Qn} \
	-vector {1 R 0 1} \
	-when "CLK*!Q*Qn" \
	-pin D \
	{ XN }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q Qn} \
	-vector {1 R 1 0} \
	-when "CLK*Q*!Qn" \
	-pin D \
	{ XN }

define_leakage -when "!CLK*!D" { XN }
define_leakage -when "!CLK*D" { XN }
define_arc \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10 00} \
	-type edge \
	-pinlist {CLK R Q} \
	-vector {R 0 F} \
	-when "!R" \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK R} \
	-prevector {01 00} \
	-pinlist {CLK R Q} \
	-vector {R 0 R} \
	-when "!R" \
	-related_pin CLK \
	-pin Q \
	{ TFF }

define_arc \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10 00} \
	-type async \
	-pinlist {CLK R Q} \
	-vector {0 R F} \
	-when "!CLK" \
	-related_pin R \
	-pin Q \
	{ TFF }

define_arc \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10} \
	-type async \
	-pinlist {CLK R Q} \
	-vector {1 R F} \
	-when "CLK" \
	-related_pin R \
	-pin Q \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {11 10} \
	-pinlist {CLK R Q} \
	-vector {F 0 0} \
	-when "!Q*!R" \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {01 00 10} \
	-pinlist {CLK R Q} \
	-vector {F 0 1} \
	-when "Q*!R" \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {11} \
	-pinlist {CLK R Q} \
	-vector {F 1 0} \
	-when "!Q*R" \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {01} \
	-pinlist {CLK R Q} \
	-vector {R 1 0} \
	-when "!Q*R" \
	-pin CLK \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {01} \
	-pinlist {CLK R Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin R \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {11} \
	-pinlist {CLK R Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin R \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {01 00} \
	-pinlist {CLK R Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin R \
	{ TFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK R} \
	-prevector {11 10} \
	-pinlist {CLK R Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin R \
	{ TFF }

define_leakage -when "!CLK*!Q*R" { TFF }
define_leakage -when "CLK*!Q*R" { TFF }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D Q} \
	-vector {F 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin D \
	{ DET }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ DET }

define_leakage -when "!CLK*!D" { DET }
define_leakage -when "!CLK*D" { DET }
define_leakage -when "CLK*!D" { DET }
define_leakage -when "CLK*D" { DET }
define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 F} \
	-when "!D*!R" \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 F} \
	-when "!D*R" \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 F} \
	-when "D*R" \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 R} \
	-when "D*!R" \
	-related_pin CLK \
	-pin Q \
	{ MOR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {1 0 R F} \
	-when "CLK*!D" \
	-related_pin R \
	-pin Q \
	{ MOR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {1 1 R F} \
	-when "CLK*D" \
	-related_pin R \
	-pin Q \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 1} \
	-when "!D*Q*!R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {F 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 0} \
	-when "D*!Q*!R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {F 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100 000} \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 1} \
	-when "!CLK*Q*R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 F 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 1} \
	-when "!CLK*Q*R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 R 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 1} \
	-when "!CLK*!D*Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 1} \
	-when "!CLK*D*Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 0 F 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 1 F 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 1} \
	-when "!CLK*!D*Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 1} \
	-when "!CLK*D*Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 R 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ MOR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 R 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ MOR }

define_leakage -when "!CLK*!D*!R" { MOR }
define_leakage -when "!CLK*!D*R" { MOR }
define_leakage -when "!CLK*D*!R" { MOR }
define_leakage -when "!CLK*D*R" { MOR }
define_leakage -when "CLK*!D*!Q*R" { MOR }
define_leakage -when "CLK*D*!Q*R" { MOR }
define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 F} \
	-when "!D*!R" \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 F} \
	-when "!D*R" \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 F} \
	-when "D*R" \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 R} \
	-when "D*!R" \
	-related_pin CLK \
	-pin Q \
	{ MORA }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-type async \
	-pinlist {CLK D R Q} \
	-vector {1 0 R F} \
	-when "CLK*!D" \
	-related_pin R \
	-pin Q \
	{ MORA }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type async \
	-pinlist {CLK D R Q} \
	-vector {1 1 R F} \
	-when "CLK*D" \
	-related_pin R \
	-pin Q \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 1} \
	-when "!D*Q*!R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {F 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 0} \
	-when "D*!Q*!R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {F 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100 000} \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 1} \
	-when "!CLK*Q*R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 F 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 1} \
	-when "!CLK*Q*R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 R 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000 001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 1} \
	-when "!CLK*!D*Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 1} \
	-when "!CLK*D*Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 0 F 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 1 F 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 1} \
	-when "!CLK*!D*Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 1} \
	-when "!CLK*D*Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {000 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 R 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ MORA }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 R 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ MORA }

define_leakage -when "!CLK*!D*!R" { MORA }
define_leakage -when "!CLK*!D*R" { MORA }
define_leakage -when "!CLK*D*!R" { MORA }
define_leakage -when "!CLK*D*R" { MORA }
define_leakage -when "CLK*!D*!Q*R" { MORA }
define_leakage -when "CLK*D*!Q*R" { MORA }
define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 F} \
	-when "!D*!R" \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 R} \
	-when "D*!R" \
	-related_pin CLK \
	-pin Q \
	{ BR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type async \
	-pinlist {CLK D R Q} \
	-vector {0 0 R F} \
	-when "!CLK*!D" \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-type async \
	-pinlist {CLK D R Q} \
	-vector {0 1 R F} \
	-when "!CLK*D" \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-type async \
	-pinlist {CLK D R Q} \
	-vector {1 0 R F} \
	-when "CLK*!D" \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type async \
	-pinlist {CLK D R Q} \
	-vector {1 1 R F} \
	-when "CLK*D" \
	-related_pin R \
	-pin Q \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 1} \
	-when "!D*Q*!R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {F 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 0} \
	-when "D*!Q*!R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {F 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 F 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 R 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 0 F 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 1 F 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 R 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ BR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 R 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ BR }

define_leakage -when "!CLK*!D*!R" { BR }
define_leakage -when "!CLK*!D*!Q*R" { BR }
define_leakage -when "!CLK*D*!R" { BR }
define_leakage -when "!CLK*D*!Q*R" { BR }
define_leakage -when "CLK*!D*!Q*R" { BR }
define_leakage -when "CLK*D*!Q*R" { BR }
define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 F} \
	-when "!D*!R" \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 R} \
	-when "D*!R" \
	-related_pin CLK \
	-pin Q \
	{ SYNCR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {0 0 R F} \
	-when "!CLK*!D" \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {0 1 R F} \
	-when "!CLK*D" \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {1 0 R F} \
	-when "CLK*!D" \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {1 1 R F} \
	-when "CLK*D" \
	-related_pin R \
	-pin Q \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 1} \
	-when "!D*Q*!R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {F 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 0} \
	-when "D*!Q*!R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {F 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 F 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 R 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 0 F 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 1 F 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 R 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ SYNCR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 R 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ SYNCR }

define_leakage -when "!CLK*!D*!R" { SYNCR }
define_leakage -when "!CLK*!D*!Q*R" { SYNCR }
define_leakage -when "!CLK*D*!R" { SYNCR }
define_leakage -when "!CLK*D*!Q*R" { SYNCR }
define_leakage -when "CLK*!D*!Q*R" { SYNCR }
define_leakage -when "CLK*D*!Q*R" { SYNCR }
define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 0 F} \
	-when "!D*!G*!R" \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 0100} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 0 R} \
	-when "D*!G*!R" \
	-related_pin CLK \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 R F} \
	-when "!CLK*!D*!R" \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 R F} \
	-when "!CLK*D*!R" \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 R F} \
	-when "CLK*!D*!R" \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 R F} \
	-when "CLK*D*!R" \
	-related_pin G \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 0 F} \
	-when "!CLK*!D*!G" \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 0 F} \
	-when "!CLK*D*!G" \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 0 F} \
	-when "CLK*!D*!G" \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 0 F} \
	-when "CLK*D*!G" \
	-related_pin R \
	-pin Q \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010 1000} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 0 0} \
	-when "!D*!G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 0 1} \
	-when "!D*!G*Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 0 0} \
	-when "!D*!G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1001} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 1 0} \
	-when "!D*G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 1 0} \
	-when "!D*G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110 1100} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 0 0} \
	-when "D*!G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 0 1} \
	-when "D*!G*Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 0 0} \
	-when "D*!G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1101} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 1 0} \
	-when "D*G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 1 0} \
	-when "D*G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 0000} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 0 0} \
	-when "!D*!G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 0 0} \
	-when "!D*!G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 1 0} \
	-when "!D*G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 1 0} \
	-when "!D*G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 0 1} \
	-when "D*!G*Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 0 0} \
	-when "D*!G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 1 0} \
	-when "D*G*!Q*!R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 1 0} \
	-when "D*G*!Q*R" \
	-pin CLK \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 0 0} \
	-when "!CLK*!G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 0 1} \
	-when "!CLK*!G*Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 0 0} \
	-when "!CLK*!G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 1 0} \
	-when "!CLK*G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 1 0} \
	-when "!CLK*G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 0 0} \
	-when "CLK*!G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 0 1} \
	-when "CLK*!G*Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 0 0} \
	-when "CLK*!G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 1 0} \
	-when "CLK*G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 1 0} \
	-when "CLK*G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 0 0} \
	-when "!CLK*!G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 0 1} \
	-when "!CLK*!G*Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 0 0} \
	-when "!CLK*!G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 1 0} \
	-when "!CLK*G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 1 0} \
	-when "!CLK*G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 0 0} \
	-when "CLK*!G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 0 1} \
	-when "CLK*!G*Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 0 0} \
	-when "CLK*!G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 1 0} \
	-when "CLK*G*!Q*!R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 1 0} \
	-when "CLK*G*!Q*R" \
	-pin D \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 F 0} \
	-when "!CLK*!D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 F 0} \
	-when "!CLK*!D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 F 0} \
	-when "!CLK*D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 F 0} \
	-when "!CLK*D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 F 0} \
	-when "CLK*!D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 F 0} \
	-when "CLK*!D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 F 0} \
	-when "CLK*D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 F 0} \
	-when "CLK*D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 R 0} \
	-when "!CLK*!D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 R 0} \
	-when "!CLK*!D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 R 0} \
	-when "!CLK*D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 R 0} \
	-when "!CLK*D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 R 0} \
	-when "CLK*!D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 R 0} \
	-when "CLK*!D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 R 0} \
	-when "CLK*D*!Q*!R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 R 0} \
	-when "CLK*D*!Q*R" \
	-pin G \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 0 0} \
	-when "!CLK*!D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 1 0} \
	-when "!CLK*!D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 0 0} \
	-when "!CLK*D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 1 0} \
	-when "!CLK*D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 0 0} \
	-when "CLK*!D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 1 0} \
	-when "CLK*!D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 0 0} \
	-when "CLK*D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 1 0} \
	-when "CLK*D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0010 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 0 0} \
	-when "!CLK*!D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 1 0} \
	-when "!CLK*!D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 0 0} \
	-when "!CLK*D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 1 0} \
	-when "!CLK*D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1010 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 0 0} \
	-when "CLK*!D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 1 0} \
	-when "CLK*!D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1110 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 0 0} \
	-when "CLK*D*!G*!Q" \
	-pin R \
	{ SYNCRG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 1 0} \
	-when "CLK*D*G*!Q" \
	-pin R \
	{ SYNCRG }

define_leakage -when "!CLK*!D*!G*!R" { SYNCRG }
define_leakage -when "!CLK*!D*!G*!Q*R" { SYNCRG }
define_leakage -when "!CLK*!D*G*!Q*!R" { SYNCRG }
define_leakage -when "!CLK*!D*G*!Q*R" { SYNCRG }
define_leakage -when "!CLK*D*!G*!R" { SYNCRG }
define_leakage -when "!CLK*D*!G*!Q*R" { SYNCRG }
define_leakage -when "!CLK*D*G*!Q*!R" { SYNCRG }
define_leakage -when "!CLK*D*G*!Q*R" { SYNCRG }
define_leakage -when "CLK*!D*!G*!Q*R" { SYNCRG }
define_leakage -when "CLK*!D*G*!Q*!R" { SYNCRG }
define_leakage -when "CLK*!D*G*!Q*R" { SYNCRG }
define_leakage -when "CLK*D*!G*!Q*R" { SYNCRG }
define_leakage -when "CLK*D*G*!Q*!R" { SYNCRG }
define_leakage -when "CLK*D*G*!Q*R" { SYNCRG }
define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 0 F} \
	-when "!D*!G*!R" \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 0 F} \
	-when "!D*!G*R" \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 1 F} \
	-when "!D*G*!R" \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 0 R} \
	-when "D*!G*!R" \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 0 R} \
	-when "D*!G*R" \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 1 R} \
	-when "D*G*!R" \
	-related_pin CLK \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 R F} \
	-when "!CLK*!D*R" \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 R F} \
	-when "!CLK*D*R" \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 R F} \
	-when "CLK*!D*R" \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 R F} \
	-when "CLK*D*R" \
	-related_pin G \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 1 F} \
	-when "!CLK*!D*G" \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 1 F} \
	-when "!CLK*D*G" \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 1 F} \
	-when "CLK*!D*G" \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-type combinational \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 1 F} \
	-when "CLK*D*G" \
	-related_pin R \
	-pin Q \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 0 0} \
	-when "!D*!G*!Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 0 1} \
	-when "!D*!G*Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 0 0} \
	-when "!D*!G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 0 1} \
	-when "!D*!G*Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 1 0} \
	-when "!D*G*!Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 1 1} \
	-when "!D*G*Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 1 0} \
	-when "!D*G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 0 0} \
	-when "D*!G*!Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 0 1} \
	-when "D*!G*Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 0 0} \
	-when "D*!G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 0 1} \
	-when "D*!G*Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 1 0} \
	-when "D*G*!Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 1 1} \
	-when "D*G*Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 1 0} \
	-when "D*G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 0 0} \
	-when "!D*!G*!Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 0 0} \
	-when "!D*!G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 1 0} \
	-when "!D*G*!Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 1 0} \
	-when "!D*G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 0 1} \
	-when "D*!G*Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 0 1} \
	-when "D*!G*Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 1 1} \
	-when "D*G*Q*!R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 1 0} \
	-when "D*G*!Q*R" \
	-pin CLK \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 0 0} \
	-when "!CLK*!G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 0 1} \
	-when "!CLK*!G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 0 0} \
	-when "!CLK*!G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 0 1} \
	-when "!CLK*!G*Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 1 0} \
	-when "!CLK*G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 1 1} \
	-when "!CLK*G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 1 0} \
	-when "!CLK*G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 0 0} \
	-when "CLK*!G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 0 1} \
	-when "CLK*!G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 0 0} \
	-when "CLK*!G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 0 1} \
	-when "CLK*!G*Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 1 0} \
	-when "CLK*G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 1 1} \
	-when "CLK*G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 1 0} \
	-when "CLK*G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 0 0} \
	-when "!CLK*!G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 0 1} \
	-when "!CLK*!G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 0 0} \
	-when "!CLK*!G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 0 1} \
	-when "!CLK*!G*Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 1 0} \
	-when "!CLK*G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 1 1} \
	-when "!CLK*G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 1 0} \
	-when "!CLK*G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 0 0} \
	-when "CLK*!G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 0 1} \
	-when "CLK*!G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 0 0} \
	-when "CLK*!G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 0 1} \
	-when "CLK*!G*Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 1 0} \
	-when "CLK*G*!Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 1 1} \
	-when "CLK*G*Q*!R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 1 0} \
	-when "CLK*G*!Q*R" \
	-pin D \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 F 0} \
	-when "!CLK*!D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 F 1} \
	-when "!CLK*!D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 F 0} \
	-when "!CLK*!D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 F 0} \
	-when "!CLK*D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 F 1} \
	-when "!CLK*D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 F 0} \
	-when "!CLK*D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 F 0} \
	-when "CLK*!D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 F 1} \
	-when "CLK*!D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 F 0} \
	-when "CLK*!D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 F 0} \
	-when "CLK*D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 F 1} \
	-when "CLK*D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 F 0} \
	-when "CLK*D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 R 0} \
	-when "!CLK*!D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 R 1} \
	-when "!CLK*!D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 R 0} \
	-when "!CLK*!D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 R 0} \
	-when "!CLK*D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 R 1} \
	-when "!CLK*D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 R 0} \
	-when "!CLK*D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 R 0} \
	-when "CLK*!D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 R 1} \
	-when "CLK*!D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 R 0} \
	-when "CLK*!D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 R 0} \
	-when "CLK*D*!Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 R 1} \
	-when "CLK*D*Q*!R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 R 0} \
	-when "CLK*D*!Q*R" \
	-pin G \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 0 0} \
	-when "!CLK*!D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 0 1} \
	-when "!CLK*!D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 1 0} \
	-when "!CLK*!D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 0 0} \
	-when "!CLK*D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 0 1} \
	-when "!CLK*D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 1 0} \
	-when "!CLK*D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 0 0} \
	-when "CLK*!D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 0 1} \
	-when "CLK*!D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 1 0} \
	-when "CLK*!D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 0 0} \
	-when "CLK*D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 0 1} \
	-when "CLK*D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 1 0} \
	-when "CLK*D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 0 0} \
	-when "!CLK*!D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 0 1} \
	-when "!CLK*!D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 1 0} \
	-when "!CLK*!D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 0 0} \
	-when "!CLK*D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 0 1} \
	-when "!CLK*D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 1 0} \
	-when "!CLK*D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 0 0} \
	-when "CLK*!D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 0 1} \
	-when "CLK*!D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 1 0} \
	-when "CLK*!D*G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 0 0} \
	-when "CLK*D*!G*!Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 0 1} \
	-when "CLK*D*!G*Q" \
	-pin R \
	{ GATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 1 0} \
	-when "CLK*D*G*!Q" \
	-pin R \
	{ GATEDR }

define_leakage -when "!CLK*!D*!G*!R" { GATEDR }
define_leakage -when "!CLK*!D*!G*R" { GATEDR }
define_leakage -when "!CLK*!D*G*!R" { GATEDR }
define_leakage -when "!CLK*!D*G*!Q*R" { GATEDR }
define_leakage -when "!CLK*D*!G*!R" { GATEDR }
define_leakage -when "!CLK*D*!G*R" { GATEDR }
define_leakage -when "!CLK*D*G*!R" { GATEDR }
define_leakage -when "!CLK*D*G*!Q*R" { GATEDR }
define_leakage -when "CLK*!D*G*!Q*R" { GATEDR }
define_leakage -when "CLK*D*G*!Q*R" { GATEDR }
define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 0 F} \
	-when "!D*!G*!R" \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 0 F} \
	-when "!D*!G*R" \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-type edge \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 1 F} \
	-when "!D*G*!R" \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 0 R} \
	-when "D*!G*!R" \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 0 R} \
	-when "D*!G*R" \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 1 R} \
	-when "D*G*!R" \
	-related_pin CLK \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 R F} \
	-when "!CLK*!D*R" \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 R F} \
	-when "!CLK*D*R" \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 R F} \
	-when "CLK*!D*R" \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 R F} \
	-when "CLK*D*R" \
	-related_pin G \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 1 F} \
	-when "!CLK*!D*G" \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 1 F} \
	-when "!CLK*D*G" \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 1 F} \
	-when "CLK*!D*G" \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-type async \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 1 F} \
	-when "CLK*D*G" \
	-related_pin R \
	-pin Q \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 0 0} \
	-when "!D*!G*!Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 0 1} \
	-when "!D*!G*Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 0 0} \
	-when "!D*!G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 0 1} \
	-when "!D*!G*Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 1 0} \
	-when "!D*G*!Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 0 1 1} \
	-when "!D*G*Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {F 0 1 1 0} \
	-when "!D*G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 0 0} \
	-when "D*!G*!Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 0 1} \
	-when "D*!G*Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 0 0} \
	-when "D*!G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 0 1} \
	-when "D*!G*Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 1 0} \
	-when "D*G*!Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 0 1 1} \
	-when "D*G*Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {F 1 1 1 0} \
	-when "D*G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 0 0} \
	-when "!D*!G*!Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 0 0} \
	-when "!D*!G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 0 1 0} \
	-when "!D*G*!Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {R 0 1 1 0} \
	-when "!D*G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 0 1} \
	-when "D*!G*Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 0 1} \
	-when "D*!G*Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 0 1 1} \
	-when "D*G*Q*!R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {R 1 1 1 0} \
	-when "D*G*!Q*R" \
	-pin CLK \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 0 0} \
	-when "!CLK*!G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 0 1} \
	-when "!CLK*!G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 0 0} \
	-when "!CLK*!G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 0 1} \
	-when "!CLK*!G*Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 1 0} \
	-when "!CLK*G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 0 1 1} \
	-when "!CLK*G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 F 1 1 0} \
	-when "!CLK*G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 0 0} \
	-when "CLK*!G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 0 1} \
	-when "CLK*!G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 0 0} \
	-when "CLK*!G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 0 1} \
	-when "CLK*!G*Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 1 0} \
	-when "CLK*G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 0 1 1} \
	-when "CLK*G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 F 1 1 0} \
	-when "CLK*G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 0 0} \
	-when "!CLK*!G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 0 1} \
	-when "!CLK*!G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 0 0} \
	-when "!CLK*!G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 0 1} \
	-when "!CLK*!G*Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 1 0} \
	-when "!CLK*G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 0 1 1} \
	-when "!CLK*G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 R 1 1 0} \
	-when "!CLK*G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 0 0} \
	-when "CLK*!G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 0 1} \
	-when "CLK*!G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 0 0} \
	-when "CLK*!G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 0 1} \
	-when "CLK*!G*Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 1 0} \
	-when "CLK*G*!Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 0 1 1} \
	-when "CLK*G*Q*!R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 R 1 1 0} \
	-when "CLK*G*!Q*R" \
	-pin D \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 F 0} \
	-when "!CLK*!D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 F 1} \
	-when "!CLK*!D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 F 0} \
	-when "!CLK*!D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 F 0} \
	-when "!CLK*D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 F 1} \
	-when "!CLK*D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 F 0} \
	-when "!CLK*D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 F 0} \
	-when "CLK*!D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 F 1} \
	-when "CLK*!D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 F 0} \
	-when "CLK*!D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 F 0} \
	-when "CLK*D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0101 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 F 1} \
	-when "CLK*D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 F 0} \
	-when "CLK*D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 R 0} \
	-when "!CLK*!D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 0 R 1} \
	-when "!CLK*!D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 1 R 0} \
	-when "!CLK*!D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 R 0} \
	-when "!CLK*D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 0 R 1} \
	-when "!CLK*D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 1 R 0} \
	-when "!CLK*D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 R 0} \
	-when "CLK*!D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 0 R 1} \
	-when "CLK*!D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 1 R 0} \
	-when "CLK*!D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 R 0} \
	-when "CLK*D*!Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 0 R 1} \
	-when "CLK*D*Q*!R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 1 R 0} \
	-when "CLK*D*!Q*R" \
	-pin G \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 0 0} \
	-when "!CLK*!D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110 0010} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 0 1} \
	-when "!CLK*!D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 F 1 0} \
	-when "!CLK*!D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 0 0} \
	-when "!CLK*D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 0110} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 0 1} \
	-when "!CLK*D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 F 1 0} \
	-when "!CLK*D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 0 0} \
	-when "CLK*!D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110 1010} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 0 1} \
	-when "CLK*!D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 F 1 0} \
	-when "CLK*!D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 0 0} \
	-when "CLK*D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0110 1110} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 0 1} \
	-when "CLK*D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 F 1 0} \
	-when "CLK*D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 0 0} \
	-when "!CLK*!D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100 0000} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 0 1} \
	-when "!CLK*!D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0011 0001} \
	-pinlist {CLK D R G Q} \
	-vector {0 0 R 1 0} \
	-when "!CLK*!D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 0 0} \
	-when "!CLK*D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 0100} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 0 1} \
	-when "!CLK*D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0111 0101} \
	-pinlist {CLK D R G Q} \
	-vector {0 1 R 1 0} \
	-when "!CLK*D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0000 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 0 0} \
	-when "CLK*!D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100 1000} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 0 1} \
	-when "CLK*!D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1011 1001} \
	-pinlist {CLK D R G Q} \
	-vector {1 0 R 1 0} \
	-when "CLK*!D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 0 0} \
	-when "CLK*D*!G*!Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {0100 1100} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 0 1} \
	-when "CLK*D*!G*Q" \
	-pin R \
	{ AGATEDR }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R G} \
	-prevector {1111 1101} \
	-pinlist {CLK D R G Q} \
	-vector {1 1 R 1 0} \
	-when "CLK*D*G*!Q" \
	-pin R \
	{ AGATEDR }

define_leakage -when "!CLK*!D*!G*!R" { AGATEDR }
define_leakage -when "!CLK*!D*!G*R" { AGATEDR }
define_leakage -when "!CLK*!D*G*!R" { AGATEDR }
define_leakage -when "!CLK*!D*G*!Q*R" { AGATEDR }
define_leakage -when "!CLK*D*!G*!R" { AGATEDR }
define_leakage -when "!CLK*D*!G*R" { AGATEDR }
define_leakage -when "!CLK*D*G*!R" { AGATEDR }
define_leakage -when "!CLK*D*G*!Q*R" { AGATEDR }
define_leakage -when "CLK*!D*G*!Q*R" { AGATEDR }
define_leakage -when "CLK*D*G*!Q*R" { AGATEDR }
define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type edge \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 F} \
	-when "!D*!R" \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 R} \
	-when "D*!R" \
	-related_pin CLK \
	-pin Q \
	{ RDFF }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {0 0 R F} \
	-when "!CLK*!D" \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {0 1 R F} \
	-when "!CLK*D" \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {1 0 R F} \
	-when "CLK*!D" \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {CLK D R Q} \
	-vector {1 1 R F} \
	-when "CLK*D" \
	-related_pin R \
	-pin Q \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {F 0 0 1} \
	-when "!D*Q*!R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {F 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 0} \
	-when "D*!Q*!R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {F 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {F 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {R 0 0 0} \
	-when "!D*!Q*!R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {R 0 1 0} \
	-when "!D*!Q*R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {R 1 0 1} \
	-when "D*Q*!R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {R 1 1 0} \
	-when "D*!Q*R" \
	-pin CLK \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010} \
	-pinlist {CLK D R Q} \
	-vector {0 F 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {0 F 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110} \
	-pinlist {CLK D R Q} \
	-vector {1 F 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 F 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 0} \
	-when "!CLK*!Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 010 000} \
	-pinlist {CLK D R Q} \
	-vector {0 R 0 1} \
	-when "!CLK*Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {0 R 1 0} \
	-when "!CLK*!Q*R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 0} \
	-when "CLK*!Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {010 110 100} \
	-pinlist {CLK D R Q} \
	-vector {1 R 0 1} \
	-when "CLK*Q*!R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 R 1 0} \
	-when "CLK*!Q*R" \
	-pin D \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001} \
	-pinlist {CLK D R Q} \
	-vector {0 0 F 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011} \
	-pinlist {CLK D R Q} \
	-vector {0 1 F 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101} \
	-pinlist {CLK D R Q} \
	-vector {1 0 F 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111} \
	-pinlist {CLK D R Q} \
	-vector {1 1 F 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {001 000} \
	-pinlist {CLK D R Q} \
	-vector {0 0 R 0} \
	-when "!CLK*!D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {011 010} \
	-pinlist {CLK D R Q} \
	-vector {0 1 R 0} \
	-when "!CLK*D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {101 100} \
	-pinlist {CLK D R Q} \
	-vector {1 0 R 0} \
	-when "CLK*!D*!Q" \
	-pin R \
	{ RDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D R} \
	-prevector {111 110} \
	-pinlist {CLK D R Q} \
	-vector {1 1 R 0} \
	-when "CLK*D*!Q" \
	-pin R \
	{ RDFF }

define_leakage -when "!CLK*!D*!R" { RDFF }
define_leakage -when "!CLK*!D*!Q*R" { RDFF }
define_leakage -when "!CLK*D*!R" { RDFF }
define_leakage -when "!CLK*D*!Q*R" { RDFF }
define_leakage -when "CLK*!D*!Q*R" { RDFF }
define_leakage -when "CLK*D*!Q*R" { RDFF }
define_arc \
	-type combinational \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 R 0 R} \
	-when "!CLK*!D*!R" \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 R 0 R} \
	-when "!CLK*D*!R" \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 R 0 R} \
	-when "CLK*!D*!R" \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 R 0 R} \
	-when "CLK*D*!R" \
	-related_pin B \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-type edge \
	-pinlist {CLK D B R Q} \
	-vector {R 0 0 0 F} \
	-when "!B*!D*!R" \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-pinlist {CLK D B R Q} \
	-vector {R 1 0 0 R} \
	-when "!B*D*!R" \
	-related_pin CLK \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 R F} \
	-when "!B*!CLK*!D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110 0100} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 R F} \
	-when "!B*!CLK*D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010 1000} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 R F} \
	-when "!B*CLK*!D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110 1100} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 R F} \
	-when "!B*CLK*D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {0 0 1 R F} \
	-when "B*!CLK*!D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {0 1 1 R F} \
	-when "B*!CLK*D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {1 0 1 R F} \
	-when "B*CLK*!D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110} \
	-type async \
	-pinlist {CLK D B R Q} \
	-vector {1 1 1 R F} \
	-when "B*CLK*D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-prevector_pinlist {CLK D B R} \
	-prevector {0011} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 1 F R} \
	-when "B*!CLK*!D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-prevector_pinlist {CLK D B R} \
	-prevector {0111} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 1 F R} \
	-when "B*!CLK*D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-prevector_pinlist {CLK D B R} \
	-prevector {1011} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 1 F R} \
	-when "B*CLK*!D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type async \
	-prevector_pinlist {CLK D B R} \
	-prevector {1111} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 1 F R} \
	-when "B*CLK*D" \
	-related_pin R \
	-pin Q \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 F 0 1} \
	-when "!CLK*!D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0011} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 F 1 0} \
	-when "!CLK*!D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 F 0 1} \
	-when "!CLK*D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0111} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 F 1 0} \
	-when "!CLK*D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 F 0 1} \
	-when "CLK*!D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1011} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 F 1 0} \
	-when "CLK*!D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 F 0 1} \
	-when "CLK*D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1111} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 F 1 0} \
	-when "CLK*D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 R 0 1} \
	-when "!CLK*!D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 R 1 0} \
	-when "!CLK*!D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 R 0 1} \
	-when "!CLK*D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 R 1 0} \
	-when "!CLK*D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 R 0 1} \
	-when "CLK*!D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 R 1 0} \
	-when "CLK*!D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 R 0 1} \
	-when "CLK*D*Q*!R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 R 1 0} \
	-when "CLK*D*!Q*R" \
	-pin B \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001 1000} \
	-pinlist {CLK D B R Q} \
	-vector {F 0 0 0 0} \
	-when "!B*!D*!Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010 1000} \
	-pinlist {CLK D B R Q} \
	-vector {F 0 0 0 1} \
	-when "!B*!D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001} \
	-pinlist {CLK D B R Q} \
	-vector {F 0 0 1 0} \
	-when "!B*!D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101 1100} \
	-pinlist {CLK D B R Q} \
	-vector {F 1 0 0 0} \
	-when "!B*D*!Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110 1100} \
	-pinlist {CLK D B R Q} \
	-vector {F 1 0 0 1} \
	-when "!B*D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101} \
	-pinlist {CLK D B R Q} \
	-vector {F 1 0 1 0} \
	-when "!B*D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010} \
	-pinlist {CLK D B R Q} \
	-vector {F 0 1 0 1} \
	-when "B*!D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1011} \
	-pinlist {CLK D B R Q} \
	-vector {F 0 1 1 0} \
	-when "B*!D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110} \
	-pinlist {CLK D B R Q} \
	-vector {F 1 1 0 1} \
	-when "B*D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1111} \
	-pinlist {CLK D B R Q} \
	-vector {F 1 1 1 0} \
	-when "B*D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001 0000} \
	-pinlist {CLK D B R Q} \
	-vector {R 0 0 0 0} \
	-when "!B*!D*!Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-vector {R 0 0 1 0} \
	-when "!B*!D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110 0100} \
	-pinlist {CLK D B R Q} \
	-vector {R 1 0 0 1} \
	-when "!B*D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101} \
	-pinlist {CLK D B R Q} \
	-vector {R 1 0 1 0} \
	-when "!B*D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010} \
	-pinlist {CLK D B R Q} \
	-vector {R 0 1 0 1} \
	-when "B*!D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0011} \
	-pinlist {CLK D B R Q} \
	-vector {R 0 1 1 0} \
	-when "B*!D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110} \
	-pinlist {CLK D B R Q} \
	-vector {R 1 1 0 1} \
	-when "B*D*Q*!R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0111} \
	-pinlist {CLK D B R Q} \
	-vector {R 1 1 1 0} \
	-when "B*D*!Q*R" \
	-pin CLK \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 F 0 0 0} \
	-when "!B*!CLK*!Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 F 0 0 1} \
	-when "!B*!CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101} \
	-pinlist {CLK D B R Q} \
	-vector {0 F 0 1 0} \
	-when "!B*!CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 F 0 0 0} \
	-when "!B*CLK*!Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 F 0 0 1} \
	-when "!B*CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101} \
	-pinlist {CLK D B R Q} \
	-vector {1 F 0 1 0} \
	-when "!B*CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0110} \
	-pinlist {CLK D B R Q} \
	-vector {0 F 1 0 1} \
	-when "B*!CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0111} \
	-pinlist {CLK D B R Q} \
	-vector {0 F 1 1 0} \
	-when "B*!CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1110} \
	-pinlist {CLK D B R Q} \
	-vector {1 F 1 0 1} \
	-when "B*CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1111} \
	-pinlist {CLK D B R Q} \
	-vector {1 F 1 1 0} \
	-when "B*CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 R 0 0 0} \
	-when "!B*!CLK*!Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 R 0 0 1} \
	-when "!B*!CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-vector {0 R 0 1 0} \
	-when "!B*!CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 R 0 0 0} \
	-when "!B*CLK*!Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 R 0 0 1} \
	-when "!B*CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001} \
	-pinlist {CLK D B R Q} \
	-vector {1 R 0 1 0} \
	-when "!B*CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0010} \
	-pinlist {CLK D B R Q} \
	-vector {0 R 1 0 1} \
	-when "B*!CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0011} \
	-pinlist {CLK D B R Q} \
	-vector {0 R 1 1 0} \
	-when "B*!CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1010} \
	-pinlist {CLK D B R Q} \
	-vector {1 R 1 0 1} \
	-when "B*CLK*Q*!R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1011} \
	-pinlist {CLK D B R Q} \
	-vector {1 R 1 1 0} \
	-when "B*CLK*!Q*R" \
	-pin D \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 F 0} \
	-when "!B*!CLK*!D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 F 0} \
	-when "!B*!CLK*D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 F 0} \
	-when "!B*CLK*!D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 F 0} \
	-when "!B*CLK*D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0001 0000} \
	-pinlist {CLK D B R Q} \
	-vector {0 0 0 R 0} \
	-when "!B*!CLK*!D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {0101 0100} \
	-pinlist {CLK D B R Q} \
	-vector {0 1 0 R 0} \
	-when "!B*!CLK*D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1001 1000} \
	-pinlist {CLK D B R Q} \
	-vector {1 0 0 R 0} \
	-when "!B*CLK*!D*!Q" \
	-pin R \
	{ COEX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D B R} \
	-prevector {1101 1100} \
	-pinlist {CLK D B R Q} \
	-vector {1 1 0 R 0} \
	-when "!B*CLK*D*!Q" \
	-pin R \
	{ COEX }

define_leakage -when "!B*!CLK*!D*!R" { COEX }
define_leakage -when "!B*!CLK*!D*!Q*R" { COEX }
define_leakage -when "!B*!CLK*D*!R" { COEX }
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
define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-type edge \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 0 0 0 F} \
	-when "!CLR*!D*!PRE" \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 1 0 0 R} \
	-when "!CLR*D*!PRE" \
	-related_pin CLK \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 R F} \
	-when "!CLK*!D*!PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 1 R F} \
	-when "!CLK*!D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110 0100} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 R F} \
	-when "!CLK*D*!PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 1 R F} \
	-when "!CLK*D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010 1000} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 R F} \
	-when "CLK*!D*!PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 1 R F} \
	-when "CLK*!D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110 1100} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 R F} \
	-when "CLK*D*!PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110} \
	-type async \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 1 R F} \
	-when "CLK*D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 1 F R} \
	-when "!CLK*!D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 1 F R} \
	-when "!CLK*D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 1 F R} \
	-when "CLK*!D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 1 F R} \
	-when "CLK*D*PRE" \
	-related_pin CLR \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 R 0 R} \
	-when "!CLK*!CLR*!D" \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 R 0 R} \
	-when "!CLK*!CLR*D" \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 R 0 R} \
	-when "CLK*!CLR*!D" \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-type async \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 R 0 R} \
	-when "CLK*!CLR*D" \
	-related_pin PRE \
	-pin Q \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 0 0 0 0} \
	-when "!CLR*!D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 0 0 0 1} \
	-when "!CLR*!D*!PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 0 1 0 1} \
	-when "!CLR*!D*PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 1 0 0 0} \
	-when "!CLR*D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 1 0 0 1} \
	-when "!CLR*D*!PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 1 1 0 1} \
	-when "!CLR*D*PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 0 0 1 0} \
	-when "CLR*!D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 0 1 1 0} \
	-when "CLR*!D*PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 1 0 1 0} \
	-when "CLR*D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {F 1 1 1 0} \
	-when "CLR*D*PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 0 0 0 0} \
	-when "!CLR*!D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 0 1 0 1} \
	-when "!CLR*!D*PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 1 0 0 1} \
	-when "!CLR*D*!PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 1 1 0 1} \
	-when "!CLR*D*PRE*Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 0 0 1 0} \
	-when "CLR*!D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 0 1 1 0} \
	-when "CLR*!D*PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 1 0 1 0} \
	-when "CLR*D*!PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {R 1 1 1 0} \
	-when "CLR*D*PRE*!Q" \
	-pin CLK \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 F 0} \
	-when "!CLK*!D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 F 0} \
	-when "!CLK*D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 F 0} \
	-when "CLK*!D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 F 0} \
	-when "CLK*D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 0 R 0} \
	-when "!CLK*!D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 0 R 0} \
	-when "!CLK*D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 0 R 0} \
	-when "CLK*!D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 0 R 0} \
	-when "CLK*D*!PRE*!Q" \
	-pin CLR \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 F 0 0 0} \
	-when "!CLK*!CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 F 0 0 1} \
	-when "!CLK*!CLR*!PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 F 1 0 1} \
	-when "!CLK*!CLR*PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 F 0 1 0} \
	-when "!CLK*CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 F 1 1 0} \
	-when "!CLK*CLR*PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 F 0 0 0} \
	-when "CLK*!CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 F 0 0 1} \
	-when "CLK*!CLR*!PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 F 1 0 1} \
	-when "CLK*!CLR*PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 F 0 1 0} \
	-when "CLK*CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 F 1 1 0} \
	-when "CLK*CLR*PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 R 0 0 0} \
	-when "!CLK*!CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 R 0 0 1} \
	-when "!CLK*!CLR*!PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 R 1 0 1} \
	-when "!CLK*!CLR*PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 R 0 1 0} \
	-when "!CLK*CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 R 1 1 0} \
	-when "!CLK*CLR*PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 R 0 0 0} \
	-when "CLK*!CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 R 0 0 1} \
	-when "CLK*!CLR*!PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 R 1 0 1} \
	-when "CLK*!CLR*PRE*Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 R 0 1 0} \
	-when "CLK*CLR*!PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 R 1 1 0} \
	-when "CLK*CLR*PRE*!Q" \
	-pin D \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 F 0 1} \
	-when "!CLK*!CLR*!D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 F 0 1} \
	-when "!CLK*!CLR*D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 F 1 0} \
	-when "!CLK*CLR*!D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 F 1 0} \
	-when "!CLK*CLR*D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 F 0 1} \
	-when "CLK*!CLR*!D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 F 0 1} \
	-when "CLK*!CLR*D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1011} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 F 1 0} \
	-when "CLK*CLR*!D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1111} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 F 1 0} \
	-when "CLK*CLR*D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0010 0000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 R 0 1} \
	-when "!CLK*!CLR*!D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0110 0100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 R 0 1} \
	-when "!CLK*!CLR*D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 0 R 1 0} \
	-when "!CLK*CLR*!D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {0101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {0 1 R 1 0} \
	-when "!CLK*CLR*D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1010 1000} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 R 0 1} \
	-when "CLK*!CLR*!D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1110 1100} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 R 0 1} \
	-when "CLK*!CLR*D*Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1001} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 0 R 1 0} \
	-when "CLK*CLR*!D*!Q" \
	-pin PRE \
	{ CAFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D PRE CLR} \
	-prevector {1101} \
	-pinlist {CLK D PRE CLR Q} \
	-vector {1 1 R 1 0} \
	-when "CLK*CLR*D*!Q" \
	-pin PRE \
	{ CAFF }

define_leakage -when "!CLK*!CLR*!D*!PRE" { CAFF }
define_leakage -when "!CLK*!CLR*!D*PRE*Q" { CAFF }
define_leakage -when "!CLK*!CLR*D*!PRE" { CAFF }
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
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {CLK D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ DLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-type combinational \
	-pinlist {CLK D Q} \
	-vector {1 F F} \
	-when "CLK" \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {10} \
	-pinlist {CLK D Q} \
	-vector {1 R R} \
	-when "CLK" \
	-related_pin D \
	-pin Q \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 0} \
	-when "!CLK*!Q" \
	-pin D \
	{ DLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ DLAT }

define_leakage -when "CLK*!D*!Q" { DLAT }
define_leakage -when "CLK*D*Q" { DLAT }
define_arc \
	-prevector_pinlist {EN D} \
	-prevector {11} \
	-type combinational \
	-pinlist {EN D Q} \
	-vector {1 F F} \
	-when "EN" \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type combinational \
	-prevector_pinlist {EN D} \
	-prevector {10} \
	-pinlist {EN D Q} \
	-vector {1 R R} \
	-when "EN" \
	-related_pin D \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-prevector_pinlist {EN D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {EN D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type edge \
	-prevector_pinlist {EN D} \
	-prevector {10 00 01} \
	-pinlist {EN D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin EN \
	-pin Q \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10 00 01} \
	-pinlist {EN D Q} \
	-vector {0 F 0} \
	-when "!EN*!Q" \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {11 01} \
	-pinlist {EN D Q} \
	-vector {0 F 1} \
	-when "!EN*Q" \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10 00} \
	-pinlist {EN D Q} \
	-vector {0 R 0} \
	-when "!EN*!Q" \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {11 01 00} \
	-pinlist {EN D Q} \
	-vector {0 R 1} \
	-when "!EN*Q" \
	-pin D \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10} \
	-pinlist {EN D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {11} \
	-pinlist {EN D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {10 00} \
	-pinlist {EN D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin EN \
	{ DLAT_EN }

define_arc \
	-type hidden \
	-prevector_pinlist {EN D} \
	-prevector {11 01} \
	-pinlist {EN D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin EN \
	{ DLAT_EN }

define_leakage -when "!D*EN*!Q" { DLAT_EN }
define_leakage -when "D*EN*Q" { DLAT_EN }
define_arc \
	-prevector_pinlist {E D} \
	-prevector {11} \
	-type combinational \
	-pinlist {E D Q} \
	-vector {1 F F} \
	-when "E" \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-prevector_pinlist {E D} \
	-prevector {10} \
	-pinlist {E D Q} \
	-vector {1 R R} \
	-when "E" \
	-related_pin D \
	-pin Q \
	{ DLAT_E }

define_arc \
	-prevector_pinlist {E D} \
	-prevector {11 01 00} \
	-type combinational \
	-pinlist {E D Q} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type combinational \
	-prevector_pinlist {E D} \
	-prevector {10 00 01} \
	-pinlist {E D Q} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin E \
	-pin Q \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {10 00 01} \
	-pinlist {E D Q} \
	-vector {0 F 0} \
	-when "!E*!Q" \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {11 01} \
	-pinlist {E D Q} \
	-vector {0 F 1} \
	-when "!E*Q" \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {10 00} \
	-pinlist {E D Q} \
	-vector {0 R 0} \
	-when "!E*!Q" \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {11 01 00} \
	-pinlist {E D Q} \
	-vector {0 R 1} \
	-when "!E*Q" \
	-pin D \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {10} \
	-pinlist {E D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {11} \
	-pinlist {E D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {10 00} \
	-pinlist {E D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin E \
	{ DLAT_E }

define_arc \
	-type hidden \
	-prevector_pinlist {E D} \
	-prevector {11 01} \
	-pinlist {E D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin E \
	{ DLAT_E }

define_leakage -when "!D*E*!Q" { DLAT_E }
define_leakage -when "D*E*Q" { DLAT_E }
define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {R 0 1} \
	-when "!D*Q" \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D Q} \
	-vector {0 F 1} \
	-when "!CLK*Q" \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-pinlist {CLK D Q} \
	-vector {0 R 1} \
	-when "!CLK*Q" \
	-pin D \
	{ GLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ GLAT }

define_leakage -when "CLK*D*Q" { GLAT }
define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 0 F} \
	-when "!CLKB*!D" \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 1 R} \
	-when "!CLKB*D" \
	-related_pin CLKA \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 0 F} \
	-when "!CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 1 R} \
	-when "!CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 F F} \
	-when "!CLKA*CLKB" \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101} \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 F F} \
	-when "CLKA*!CLKB" \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {111} \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 F F} \
	-when "CLKA*CLKB" \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 R R} \
	-when "!CLKA*CLKB" \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 R R} \
	-when "CLKA*!CLKB" \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 R R} \
	-when "CLKA*CLKB" \
	-related_pin D \
	-pin Q \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 0 0} \
	-when "!CLKB*!D*!Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 1 1} \
	-when "!CLKB*D*Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 0 0} \
	-when "CLKB*!D*!Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 1 1} \
	-when "CLKB*D*Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 0 0} \
	-when "!CLKB*!D*!Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 1 1} \
	-when "!CLKB*D*Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 0 0} \
	-when "CLKB*!D*!Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 1 1} \
	-when "CLKB*D*Q" \
	-pin CLKA \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 0 0} \
	-when "!CLKA*!D*!Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 1 1} \
	-when "!CLKA*D*Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 0} \
	-when "CLKA*!D*!Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 1} \
	-when "CLKA*D*Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 0 0} \
	-when "!CLKA*!D*!Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 1 1} \
	-when "!CLKA*D*Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 0} \
	-when "CLKA*!D*!Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 1} \
	-when "CLKA*D*Q" \
	-pin CLKB \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 F 0} \
	-when "!CLKA*!CLKB*!Q" \
	-pin D \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 F 1} \
	-when "!CLKA*!CLKB*Q" \
	-pin D \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 R 0} \
	-when "!CLKA*!CLKB*!Q" \
	-pin D \
	{ MUXLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 R 1} \
	-when "!CLKA*!CLKB*Q" \
	-pin D \
	{ MUXLAT }

define_leakage -when "!CLKA*CLKB*!D*!Q" { MUXLAT }
define_leakage -when "!CLKA*CLKB*D*Q" { MUXLAT }
define_leakage -when "CLKA*!CLKB*!D*!Q" { MUXLAT }
define_leakage -when "CLKA*!CLKB*D*Q" { MUXLAT }
define_leakage -when "CLKA*CLKB*!D*!Q" { MUXLAT }
define_leakage -when "CLKA*CLKB*D*Q" { MUXLAT }
define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 0 F} \
	-when "CLKB*!D" \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 1 R} \
	-when "CLKB*D" \
	-related_pin CLKA \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 0 F} \
	-when "!CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 F} \
	-when "CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000 100 101} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 F} \
	-when "CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 1 R} \
	-when "!CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 R} \
	-when "CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 R} \
	-when "CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ MCDFF }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-type combinational \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 F F} \
	-when "!CLKA*CLKB" \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-type combinational \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 R R} \
	-when "!CLKA*CLKB" \
	-related_pin D \
	-pin Q \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 0 0} \
	-when "!CLKB*!D*!Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 0 1} \
	-when "!CLKB*!D*Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 1 0} \
	-when "!CLKB*D*!Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 1 1} \
	-when "!CLKB*D*Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 0 0} \
	-when "CLKB*!D*!Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 1 1} \
	-when "CLKB*D*Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 0 0} \
	-when "!CLKB*!D*!Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 0 1} \
	-when "!CLKB*!D*Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 1 0} \
	-when "!CLKB*D*!Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 1 1} \
	-when "!CLKB*D*Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 0 0} \
	-when "CLKB*!D*!Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 1 1} \
	-when "CLKB*D*Q" \
	-pin CLKA \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 0 0} \
	-when "!CLKA*!D*!Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 1 1} \
	-when "!CLKA*D*Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 0} \
	-when "CLKA*!D*!Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 1} \
	-when "CLKA*!D*Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 0} \
	-when "CLKA*D*!Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 1} \
	-when "CLKA*D*Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 0 0} \
	-when "!CLKA*!D*!Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 1 1} \
	-when "!CLKA*D*Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 0} \
	-when "CLKA*!D*!Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 1} \
	-when "CLKA*!D*Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 0} \
	-when "CLKA*D*!Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 1} \
	-when "CLKA*D*Q" \
	-pin CLKB \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 F 0} \
	-when "!CLKA*!CLKB*!Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 F 1} \
	-when "!CLKA*!CLKB*Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 F 0} \
	-when "CLKA*!CLKB*!Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 F 1} \
	-when "CLKA*!CLKB*Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 F 0} \
	-when "CLKA*CLKB*!Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 F 1} \
	-when "CLKA*CLKB*Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 R 0} \
	-when "!CLKA*!CLKB*!Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 R 1} \
	-when "!CLKA*!CLKB*Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 R 0} \
	-when "CLKA*!CLKB*!Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 R 1} \
	-when "CLKA*!CLKB*Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {010 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 R 0} \
	-when "CLKA*CLKB*!Q" \
	-pin D \
	{ MCDFF }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {011 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 R 1} \
	-when "CLKA*CLKB*Q" \
	-pin D \
	{ MCDFF }

define_leakage -when "!CLKA*!CLKB*!D" { MCDFF }
define_leakage -when "!CLKA*!CLKB*D" { MCDFF }
define_leakage -when "!CLKA*CLKB*!D*!Q" { MCDFF }
define_leakage -when "!CLKA*CLKB*D*Q" { MCDFF }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D Q} \
	-vector {F 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-type edge \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 R} \
	-when "D" \
	-related_pin CLK \
	-pin Q \
	{ TCASC }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-type combinational \
	-pinlist {CLK D Q} \
	-vector {0 F F} \
	-when "!CLK" \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q} \
	-vector {0 R R} \
	-when "!CLK" \
	-related_pin D \
	-pin Q \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {F 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {F 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00} \
	-pinlist {CLK D Q} \
	-vector {R 0 0} \
	-when "!D*!Q" \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01} \
	-pinlist {CLK D Q} \
	-vector {R 1 1} \
	-when "D*Q" \
	-pin CLK \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 0} \
	-when "CLK*!Q" \
	-pin D \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D Q} \
	-vector {1 F 1} \
	-when "CLK*Q" \
	-pin D \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 0} \
	-when "CLK*!Q" \
	-pin D \
	{ TCASC }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-pinlist {CLK D Q} \
	-vector {1 R 1} \
	-when "CLK*Q" \
	-pin D \
	{ TCASC }

define_leakage -when "!CLK*!D*!Q" { TCASC }
define_leakage -when "!CLK*D*Q" { TCASC }
define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type edge \
	-pinlist {CLK D T} \
	-vector {R 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-type edge \
	-pinlist {CLK D T} \
	-vector {R 1 F} \
	-when "D" \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type edge \
	-pinlist {CLK D T} \
	-vector {F 0 F} \
	-when "!D" \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-type edge \
	-pinlist {CLK D T} \
	-vector {F 1 F} \
	-when "D" \
	-related_pin CLK \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {11 01 00} \
	-type combinational \
	-pinlist {CLK D T} \
	-vector {0 R F} \
	-when "!CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {10 00 01} \
	-type combinational \
	-pinlist {CLK D T} \
	-vector {0 F F} \
	-when "!CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {01 11 10} \
	-type combinational \
	-pinlist {CLK D T} \
	-vector {1 R F} \
	-when "CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-prevector_pinlist {CLK D} \
	-prevector {00 10 11} \
	-type combinational \
	-pinlist {CLK D T} \
	-vector {1 F F} \
	-when "CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D T} \
	-vector {0 R R} \
	-when "!CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D T} \
	-vector {0 F R} \
	-when "!CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D T} \
	-vector {1 R R} \
	-when "CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D T} \
	-vector {1 F R} \
	-when "CLK" \
	-related_pin D \
	-pin T \
	{ XLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {00 10} \
	-pinlist {CLK D T} \
	-vector {F 0 0} \
	-when "!D*!T" \
	-pin CLK \
	{ XLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {01 11} \
	-pinlist {CLK D T} \
	-vector {F 1 0} \
	-when "D*!T" \
	-pin CLK \
	{ XLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {10 00} \
	-pinlist {CLK D T} \
	-vector {R 0 0} \
	-when "!D*!T" \
	-pin CLK \
	{ XLAT }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK D} \
	-prevector {11 01} \
	-pinlist {CLK D T} \
	-vector {R 1 0} \
	-when "D*!T" \
	-pin CLK \
	{ XLAT }

define_leakage -when "!CLK*!D" { XLAT }
define_leakage -when "!CLK*D" { XLAT }
define_leakage -when "CLK*!D" { XLAT }
define_leakage -when "CLK*D" { XLAT }
define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 000} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 0 F} \
	-when "!CLKB*!D" \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 1 R} \
	-when "!CLKB*D" \
	-related_pin CLKA \
	-pin Q \
	{ HPIPE }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010 110 010} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 0 F} \
	-when "!CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010 110 010 011} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 1 F} \
	-when "!CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010 110} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 F} \
	-when "CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010 110 111} \
	-type edge \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 F} \
	-when "CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011 111 011 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 0 R} \
	-when "!CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011 111 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 1 R} \
	-when "!CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 R} \
	-when "CLKA*!D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 R} \
	-when "CLKA*D" \
	-related_pin CLKB \
	-pin Q \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 0 0} \
	-when "!CLKB*!D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 0 1} \
	-when "!CLKB*!D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 1 0} \
	-when "!CLKB*D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 0 1 1} \
	-when "!CLKB*D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 0 0} \
	-when "CLKB*!D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 0 1} \
	-when "CLKB*!D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 1 0} \
	-when "CLKB*D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {F 1 1 1} \
	-when "CLKB*D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 0 0} \
	-when "!CLKB*!D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 0 1 1} \
	-when "!CLKB*D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 0 0} \
	-when "CLKB*!D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 0 1} \
	-when "CLKB*!D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 1 0} \
	-when "CLKB*D*!Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {R 1 1 1} \
	-when "CLKB*D*Q" \
	-pin CLKA \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 0 0} \
	-when "!CLKA*!D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 0 1} \
	-when "!CLKA*!D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 1 0} \
	-when "!CLKA*D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 F 1 1} \
	-when "!CLKA*D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 0} \
	-when "CLKA*!D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 0 1} \
	-when "CLKA*!D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 0} \
	-when "CLKA*D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 F 1 1} \
	-when "CLKA*D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 0 0} \
	-when "!CLKA*!D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 0 1} \
	-when "!CLKA*!D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 1 0} \
	-when "!CLKA*D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 R 1 1} \
	-when "!CLKA*D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 0} \
	-when "CLKA*!D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 0 1} \
	-when "CLKA*!D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 0} \
	-when "CLKA*D*!Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 R 1 1} \
	-when "CLKA*D*Q" \
	-pin CLKB \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 F 0} \
	-when "!CLKA*!CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 F 1} \
	-when "!CLKA*!CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 F 0} \
	-when "!CLKA*CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 F 1} \
	-when "!CLKA*CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 F 0} \
	-when "CLKA*!CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 F 1} \
	-when "CLKA*!CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 F 0} \
	-when "CLKA*CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 F 1} \
	-when "CLKA*CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 R 0} \
	-when "!CLKA*!CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 000} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 0 R 1} \
	-when "!CLKA*!CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 000 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 R 0} \
	-when "!CLKA*CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 001 011 010} \
	-pinlist {CLKA CLKB D Q} \
	-vector {0 1 R 1} \
	-when "!CLKA*CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 R 0} \
	-when "CLKA*!CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 100} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 0 R 1} \
	-when "CLKA*!CLKB*Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {000 100 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 R 0} \
	-when "CLKA*CLKB*!Q" \
	-pin D \
	{ HPIPE }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB D} \
	-prevector {001 101 111 110} \
	-pinlist {CLKA CLKB D Q} \
	-vector {1 1 R 1} \
	-when "CLKA*CLKB*Q" \
	-pin D \
	{ HPIPE }

define_leakage -when "!CLKA*!CLKB*!D" { HPIPE }
define_leakage -when "!CLKA*!CLKB*D" { HPIPE }
define_leakage -when "!CLKA*CLKB*!D" { HPIPE }
define_leakage -when "!CLKA*CLKB*D" { HPIPE }
define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001 0000} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 0 0 F} \
	-when "!CLKB*!DA*!DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 0 1 F} \
	-when "!CLKB*!DA*DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1110 1100} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 0 0 F} \
	-when "CLKB*!DA*!DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1110 1100 1101} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 0 1 F} \
	-when "CLKB*!DA*DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1110} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 1 0 F} \
	-when "CLKB*DA*!DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1110 1111} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 1 1 F} \
	-when "CLKB*DA*DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 1 0 R} \
	-when "!CLKB*DA*!DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 1 1 R} \
	-when "!CLKB*DA*DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 1 0 R} \
	-when "CLKB*DA*!DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 1 1 R} \
	-when "CLKB*DA*DB" \
	-related_pin CLKA \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001 0000} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 0 0 F} \
	-when "!CLKA*!DA*!DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 1 0 F} \
	-when "!CLKA*DA*!DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101 1100} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 0 0 F} \
	-when "CLKA*!DA*!DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 0 1 F} \
	-when "CLKA*!DA*DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101 1111 1110} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 1 0 F} \
	-when "CLKA*DA*!DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1101 1111} \
	-type edge \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 1 1 F} \
	-when "CLKA*DA*DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 0 1 R} \
	-when "!CLKA*!DA*DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 1 1 R} \
	-when "!CLKA*DA*DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 0 1 R} \
	-when "CLKA*!DA*DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type edge \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 1 1 R} \
	-when "CLKA*DA*DB" \
	-related_pin CLKB \
	-pin Q \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 0 0 0} \
	-when "!CLKB*!DA*!DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 0 0 1} \
	-when "!CLKB*!DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 0 1 0} \
	-when "!CLKB*!DA*DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 0 1 1} \
	-when "!CLKB*!DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 1 0 0} \
	-when "!CLKB*DA*!DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 1 0 1} \
	-when "!CLKB*DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 1 1 0} \
	-when "!CLKB*DA*DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 0 1 1 1} \
	-when "!CLKB*DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 0 0 0} \
	-when "CLKB*!DA*!DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {1001 1101 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 0 0 1} \
	-when "CLKB*!DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 0 1 0} \
	-when "CLKB*!DA*DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {1001 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 0 1 1} \
	-when "CLKB*!DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 1 0 0} \
	-when "CLKB*DA*!DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {1011 1111 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 1 0 1} \
	-when "CLKB*DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 1 1 0} \
	-when "CLKB*DA*DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {1011 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {F 1 1 1 1} \
	-when "CLKB*DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 0 0 0} \
	-when "!CLKB*!DA*!DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 0 1 0} \
	-when "!CLKB*!DA*DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 1 0 1} \
	-when "!CLKB*DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 0 1 1 1} \
	-when "!CLKB*DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 0 0 0} \
	-when "CLKB*!DA*!DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 0 0 1} \
	-when "CLKB*!DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 0 1 0} \
	-when "CLKB*!DA*DB*!Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 0 1 1} \
	-when "CLKB*!DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 1 0 1} \
	-when "CLKB*DA*!DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {R 1 1 1 1} \
	-when "CLKB*DA*DB*Q" \
	-pin CLKA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 0 0 0} \
	-when "!CLKA*!DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 0 0 1} \
	-when "!CLKA*!DA*!DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 0 1 0} \
	-when "!CLKA*!DA*DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 0 1 1} \
	-when "!CLKA*!DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 1 0 0} \
	-when "!CLKA*DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 1 0 1} \
	-when "!CLKA*DA*!DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 1 1 0} \
	-when "!CLKA*DA*DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 F 1 1 1} \
	-when "!CLKA*DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 0 0 0} \
	-when "CLKA*!DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0110 1110 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 0 0 1} \
	-when "CLKA*!DA*!DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 0 1 0} \
	-when "CLKA*!DA*DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0111 1111 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 0 1 1} \
	-when "CLKA*!DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 1 0 0} \
	-when "CLKA*DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0110 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 1 0 1} \
	-when "CLKA*DA*!DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 1 1 0} \
	-when "CLKA*DA*DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0111 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 F 1 1 1} \
	-when "CLKA*DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 0 0 0} \
	-when "!CLKA*!DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 0 1 1} \
	-when "!CLKA*!DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 1 0 0} \
	-when "!CLKA*DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 R 1 1 1} \
	-when "!CLKA*DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 0 0 0} \
	-when "CLKA*!DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 0 0 1} \
	-when "CLKA*!DA*!DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 0 1 1} \
	-when "CLKA*!DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 1 0 0} \
	-when "CLKA*DA*!DB*!Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 1 0 1} \
	-when "CLKA*DA*!DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 R 1 1 1} \
	-when "CLKA*DA*DB*Q" \
	-pin CLKB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 F 0 0} \
	-when "!CLKA*!CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 F 0 1} \
	-when "!CLKA*!CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 F 1 0} \
	-when "!CLKA*!CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 F 1 1} \
	-when "!CLKA*!CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 F 0 0} \
	-when "!CLKA*CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 F 0 1} \
	-when "!CLKA*CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 F 1 0} \
	-when "!CLKA*CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 F 1 1} \
	-when "!CLKA*CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 F 0 0} \
	-when "CLKA*!CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 F 0 1} \
	-when "CLKA*!CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 F 1 0} \
	-when "CLKA*!CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 F 1 1} \
	-when "CLKA*!CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 F 0 0} \
	-when "CLKA*CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0110 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 F 0 1} \
	-when "CLKA*CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 F 1 0} \
	-when "CLKA*CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0111 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 F 1 1} \
	-when "CLKA*CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 R 0 0} \
	-when "!CLKA*!CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 R 0 1} \
	-when "!CLKA*!CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 R 1 0} \
	-when "!CLKA*!CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 R 1 1} \
	-when "!CLKA*!CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 R 0 0} \
	-when "!CLKA*CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 R 0 1} \
	-when "!CLKA*CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 R 1 0} \
	-when "!CLKA*CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 R 1 1} \
	-when "!CLKA*CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 R 0 0} \
	-when "CLKA*!CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 R 0 1} \
	-when "CLKA*!CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 R 1 0} \
	-when "CLKA*!CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 R 1 1} \
	-when "CLKA*!CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 R 0 0} \
	-when "CLKA*CLKB*!DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0110 1110 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 R 0 1} \
	-when "CLKA*CLKB*!DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 R 1 0} \
	-when "CLKA*CLKB*DB*!Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {1001 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 R 1 1} \
	-when "CLKA*CLKB*DB*Q" \
	-pin DA \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 F 0} \
	-when "!CLKA*!CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 F 1} \
	-when "!CLKA*!CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 0001 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 F 0} \
	-when "!CLKA*!CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 0011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 F 1} \
	-when "!CLKA*!CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 F 0} \
	-when "!CLKA*CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 F 1} \
	-when "!CLKA*CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 F 0} \
	-when "!CLKA*CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 F 1} \
	-when "!CLKA*CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 F 0} \
	-when "CLKA*!CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011 1001} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 F 1} \
	-when "CLKA*!CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 1001 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 F 0} \
	-when "CLKA*!CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 1011} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 F 1} \
	-when "CLKA*!CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 F 0} \
	-when "CLKA*CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {1001 1101} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 F 1} \
	-when "CLKA*CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 F 0} \
	-when "CLKA*CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0111 1111} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 F 1} \
	-when "CLKA*CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 R 0} \
	-when "!CLKA*!CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0001 0000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 0 R 1} \
	-when "!CLKA*!CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 R 0} \
	-when "!CLKA*!CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 0010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 0 1 R 1} \
	-when "!CLKA*!CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 R 0} \
	-when "!CLKA*CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0001 0101 0100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 0 R 1} \
	-when "!CLKA*CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 R 0} \
	-when "!CLKA*CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0011 0111 0110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {0 1 1 R 1} \
	-when "!CLKA*CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 R 0} \
	-when "CLKA*!CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010 1000} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 0 R 1} \
	-when "CLKA*!CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 R 0} \
	-when "CLKA*!CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0010 1010} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 0 1 R 1} \
	-when "CLKA*!CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 R 0} \
	-when "CLKA*CLKB*!DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0110 1110 1100} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 0 R 1} \
	-when "CLKA*CLKB*!DA*Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0000 1000 1100 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 R 0} \
	-when "CLKA*CLKB*DA*!Q" \
	-pin DB \
	{ DCMUX }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB DA DB} \
	-prevector {0110 1110} \
	-pinlist {CLKA CLKB DA DB Q} \
	-vector {1 1 1 R 1} \
	-when "CLKA*CLKB*DA*Q" \
	-pin DB \
	{ DCMUX }

define_leakage -when "!CLKA*!CLKB*!DA*!DB" { DCMUX }
define_leakage -when "!CLKA*!CLKB*!DA*DB" { DCMUX }
define_leakage -when "!CLKA*!CLKB*DA*!DB" { DCMUX }
define_leakage -when "!CLKA*!CLKB*DA*DB" { DCMUX }
define_leakage -when "!CLKA*CLKB*!DA*!DB" { DCMUX }
define_leakage -when "!CLKA*CLKB*!DA*DB" { DCMUX }
define_leakage -when "!CLKA*CLKB*DA*!DB" { DCMUX }
define_leakage -when "!CLKA*CLKB*DA*DB" { DCMUX }
define_leakage -when "CLKA*!CLKB*!DA*!DB" { DCMUX }
define_leakage -when "CLKA*!CLKB*!DA*DB" { DCMUX }
define_leakage -when "CLKA*!CLKB*DA*!DB" { DCMUX }
define_leakage -when "CLKA*!CLKB*DA*DB" { DCMUX }
define_arc \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11 10} \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-vector {F 0 F} \
	-when "!EN" \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11} \
	-type combinational \
	-pinlist {CLK EN GCLK} \
	-vector {F 1 F} \
	-when "EN" \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type combinational \
	-prevector_pinlist {CLK EN} \
	-prevector {01} \
	-pinlist {CLK EN GCLK} \
	-vector {R 1 R} \
	-when "EN" \
	-related_pin CLK \
	-pin GCLK \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10} \
	-pinlist {CLK EN GCLK} \
	-vector {F 0 0} \
	-when "!EN*!GCLK" \
	-pin CLK \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10 11} \
	-pinlist {CLK EN GCLK} \
	-vector {F 1 0} \
	-when "EN*!GCLK" \
	-pin CLK \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00} \
	-pinlist {CLK EN GCLK} \
	-vector {R 0 0} \
	-when "!EN*!GCLK" \
	-pin CLK \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {01} \
	-pinlist {CLK EN GCLK} \
	-vector {0 F 0} \
	-when "!CLK*!GCLK" \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10 11} \
	-pinlist {CLK EN GCLK} \
	-vector {1 F 0} \
	-when "CLK*!GCLK" \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11} \
	-pinlist {CLK EN GCLK} \
	-vector {1 F 1} \
	-when "CLK*GCLK" \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00} \
	-pinlist {CLK EN GCLK} \
	-vector {0 R 0} \
	-when "!CLK*!GCLK" \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {00 10} \
	-pinlist {CLK EN GCLK} \
	-vector {1 R 0} \
	-when "CLK*!GCLK" \
	-pin EN \
	{ ICG }

define_arc \
	-type hidden \
	-prevector_pinlist {CLK EN} \
	-prevector {01 11 10} \
	-pinlist {CLK EN GCLK} \
	-vector {1 R 1} \
	-when "CLK*GCLK" \
	-pin EN \
	{ ICG }

define_leakage -when "!CLK*!EN*!GCLK" { ICG }
define_leakage -when "!CLK*EN*!GCLK" { ICG }
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
	-prevector {10100 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 10001} \
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
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 0 1 1 0} \
	-when "!CLKB*!GCLK*!RA*RB*S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {F 0 1 0 0 0} \
	-when "!CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101} \
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
	-prevector {10101 11101 10101 11101 11001 11000} \
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
	-prevector {10101 11101 10101 11101 11001} \
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
	-prevector {11111 11011} \
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
	-prevector {00100 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 0 0 0 0} \
	-when "!CLKB*!GCLK*!RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101 00001} \
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
	-prevector {00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {R 0 1 0 0 0} \
	-when "!CLKB*!GCLK*RA*!RB*!S" \
	-pin CLKA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101} \
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
	-prevector {00101 01101 00101 01101 01001 01000} \
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
	-prevector {00101 01101 00101 01101 01001} \
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
	-prevector {01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 F 0 1 0 0} \
	-when "!CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01011} \
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
	-prevector {01111 01101} \
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
	-prevector {01010 11010 01010 11010 11000} \
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
	-prevector {01010 11010 01010 11010 11000 11001} \
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
	-prevector {11111 11101} \
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
	-prevector {00010 00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 0 0} \
	-when "!CLKA*!GCLK*!RA*!RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011 00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 0 1 0} \
	-when "!CLKA*!GCLK*!RA*!RB*S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 R 0 1 0 0} \
	-when "!CLKA*!GCLK*!RA*RB*!S" \
	-pin CLKB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011} \
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
	-prevector {00010 10010 00010 10010 10000} \
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
	-prevector {00010 10010 00010 10010 10000 10001} \
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
	-prevector {00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 F 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101} \
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
	-prevector {10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 F 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101} \
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
	-prevector {00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 0 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 R 1 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*!S" \
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
	-prevector {01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 0 1 0} \
	-when "!CLKA*CLKB*!GCLK*!RB*S" \
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
	-prevector {01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 1 0 0} \
	-when "!CLKA*CLKB*!GCLK*RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 R 1 1 0} \
	-when "!CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10100 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 R 0 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RB*!S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101 10001} \
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
	-prevector {10111 10011} \
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
	-prevector {10101 11101 10101 11101 11001 11000} \
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
	-prevector {10101 11101 10101 11101 11001} \
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
	-prevector {11111 11011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 R 1 1 0} \
	-when "CLKA*CLKB*!GCLK*RB*S" \
	-pin RA \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 F 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011} \
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
	-prevector {01010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 F 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01011} \
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
	-prevector {00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 0 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 1 R 1 0} \
	-when "!CLKA*!CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 R 0 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {01011 01001} \
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
	-prevector {01111 01101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 1 R 1 0} \
	-when "!CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10100 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!S" \
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
	-prevector {10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 R 1 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*S" \
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
	-prevector {10100} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 1 R 0 0} \
	-when "CLKA*!CLKB*!GCLK*RA*!S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {10101} \
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
	-prevector {01010 11010 01010 11010 11000} \
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
	-prevector {01010 11010 01010 11010 11000 11001} \
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
	-prevector {11111 11101} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 1 1 R 1 0} \
	-when "CLKA*CLKB*!GCLK*RA*S" \
	-pin RB \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 F 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00101} \
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
	-prevector {01011 01001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 F 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
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
	-prevector {01011} \
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
	-prevector {10101 10001} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
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
	-prevector {10111 10011} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 F 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB" \
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
	-prevector {10101} \
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
	-prevector {10101 11101 10101 11101 11001} \
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
	-prevector {00000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 0 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*!RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 0 0 1 R 0} \
	-when "!CLKA*!CLKB*!GCLK*!RA*RB" \
	-pin S \
	{ ICM }

define_arc \
	-type hidden \
	-prevector_pinlist {CLKA CLKB RA RB S} \
	-prevector {00100} \
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
	-prevector {01010 01000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {0 1 0 0 R 0} \
	-when "!CLKA*CLKB*!GCLK*!RA*!RB" \
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
	-prevector {01010} \
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
	-prevector {10100 10000} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 0 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*!RB" \
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
	-prevector {10110 10010} \
	-pinlist {CLKA CLKB RA RB S GCLK} \
	-vector {1 0 0 1 R 0} \
	-when "CLKA*!CLKB*!GCLK*!RA*RB" \
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
	-prevector {10100} \
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
	-prevector {01010 11010 01010 11010 11000} \
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
	-prevector_pinlist {C D} \
	-prevector {01 11 10} \
	-type combinational \
	-pinlist {C D Y} \
	-vector {F 0 F} \
	-when "!D" \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-prevector_pinlist {C D} \
	-prevector {01 11} \
	-type combinational \
	-pinlist {C D Y} \
	-vector {F 1 F} \
	-when "D" \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type combinational \
	-prevector_pinlist {C D} \
	-prevector {01} \
	-pinlist {C D Y} \
	-vector {R 1 R} \
	-when "D" \
	-related_pin C \
	-pin Y \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00 10} \
	-pinlist {C D Y} \
	-vector {F 0 0} \
	-when "!D*!Y" \
	-pin C \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00 10 11} \
	-pinlist {C D Y} \
	-vector {F 1 0} \
	-when "D*!Y" \
	-pin C \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00} \
	-pinlist {C D Y} \
	-vector {R 0 0} \
	-when "!D*!Y" \
	-pin C \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {01} \
	-pinlist {C D Y} \
	-vector {0 F 0} \
	-when "!C*!Y" \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00 10 11} \
	-pinlist {C D Y} \
	-vector {1 F 0} \
	-when "C*!Y" \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {01 11} \
	-pinlist {C D Y} \
	-vector {1 F 1} \
	-when "C*Y" \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00} \
	-pinlist {C D Y} \
	-vector {0 R 0} \
	-when "!C*!Y" \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {00 10} \
	-pinlist {C D Y} \
	-vector {1 R 0} \
	-when "C*!Y" \
	-pin D \
	{ GL }

define_arc \
	-type hidden \
	-prevector_pinlist {C D} \
	-prevector {01 11 10} \
	-pinlist {C D Y} \
	-vector {1 R 1} \
	-when "C*Y" \
	-pin D \
	{ GL }

define_leakage -when "!C*!D*!Y" { GL }
define_leakage -when "!C*D*!Y" { GL }
define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011 0010} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {0 0 F 0 F X} \
	-when "!A*!B*!D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {0 0 F 1 F X} \
	-when "!A*!B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111 0110} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {0 1 F 0 F X} \
	-when "!A*B*!D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {0 1 F 1 F X} \
	-when "!A*B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011 1010} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 0 F 0 F X} \
	-when "A*!B*!D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 0 F 1 F X} \
	-when "A*!B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111 1110} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 1 F 0 F X} \
	-when "A*B*!D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 1 F 1 F X} \
	-when "A*B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0001} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 R 1 R X} \
	-when "!A*!B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0101} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 R 1 R X} \
	-when "!A*B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1001} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 R 1 R X} \
	-when "A*!B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1101} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 R 1 R X} \
	-when "A*B*D" \
	-related_pin C \
	-pin Y \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1100} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {F 1 0 0 X F} \
	-when "B*!C*!D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1101} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {F 1 0 1 X F} \
	-when "B*!C*D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1110} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {F 1 1 0 X F} \
	-when "B*C*!D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1111} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {F 1 1 1 X F} \
	-when "B*C*D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0100} \
	-pinlist {A B C D Y Z} \
	-vector {R 1 0 0 X R} \
	-when "B*!C*!D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0101} \
	-pinlist {A B C D Y Z} \
	-vector {R 1 0 1 X R} \
	-when "B*!C*D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0110} \
	-pinlist {A B C D Y Z} \
	-vector {R 1 1 0 X R} \
	-when "B*C*!D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {0111} \
	-pinlist {A B C D Y Z} \
	-vector {R 1 1 1 X R} \
	-when "B*C*D" \
	-related_pin A \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1100} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 F 0 0 X F} \
	-when "A*!C*!D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1101} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 F 0 1 X F} \
	-when "A*!C*D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1110} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 F 1 0 X F} \
	-when "A*C*!D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-prevector_pinlist {A B C D} \
	-prevector {1111} \
	-type combinational \
	-pinlist {A B C D Y Z} \
	-vector {1 F 1 1 X F} \
	-when "A*C*D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1000} \
	-pinlist {A B C D Y Z} \
	-vector {1 R 0 0 X R} \
	-when "A*!C*!D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1001} \
	-pinlist {A B C D Y Z} \
	-vector {1 R 0 1 X R} \
	-when "A*!C*D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1010} \
	-pinlist {A B C D Y Z} \
	-vector {1 R 1 0 X R} \
	-when "A*C*!D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type combinational \
	-prevector_pinlist {A B C D} \
	-prevector {1011} \
	-pinlist {A B C D Y Z} \
	-vector {1 R 1 1 X R} \
	-when "A*C*D" \
	-related_pin B \
	-pin Z \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000} \
	-pinlist {A B C D Y Z} \
	-vector {F 0 0 0 0 0} \
	-when "!B*!C*!D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1001} \
	-pinlist {A B C D Y Z} \
	-vector {F 0 0 1 0 0} \
	-when "!B*!C*D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010} \
	-pinlist {A B C D Y Z} \
	-vector {F 0 1 0 0 0} \
	-when "!B*C*!D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011 1010} \
	-pinlist {A B C D Y Z} \
	-vector {F 0 1 0 1 0} \
	-when "!B*C*!D*Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010 1011} \
	-pinlist {A B C D Y Z} \
	-vector {F 0 1 1 0 0} \
	-when "!B*C*D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011} \
	-pinlist {A B C D Y Z} \
	-vector {F 0 1 1 1 0} \
	-when "!B*C*D*Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-vector {R 0 0 0 0 0} \
	-when "!B*!C*!D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001} \
	-pinlist {A B C D Y Z} \
	-vector {R 0 0 1 0 0} \
	-when "!B*!C*D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010} \
	-pinlist {A B C D Y Z} \
	-vector {R 0 1 0 0 0} \
	-when "!B*C*!D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011 0010} \
	-pinlist {A B C D Y Z} \
	-vector {R 0 1 0 1 0} \
	-when "!B*C*!D*Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010 0011} \
	-pinlist {A B C D Y Z} \
	-vector {R 0 1 1 0 0} \
	-when "!B*C*D*!Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011} \
	-pinlist {A B C D Y Z} \
	-vector {R 0 1 1 1 0} \
	-when "!B*C*D*Y*!Z" \
	-pin A \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100} \
	-pinlist {A B C D Y Z} \
	-vector {0 F 0 0 0 0} \
	-when "!A*!C*!D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0101} \
	-pinlist {A B C D Y Z} \
	-vector {0 F 0 1 0 0} \
	-when "!A*!C*D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 F 1 0 0 0} \
	-when "!A*C*!D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 F 1 0 1 0} \
	-when "!A*C*!D*Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 F 1 1 0 0} \
	-when "!A*C*D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 F 1 1 1 0} \
	-when "!A*C*D*Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-vector {0 R 0 0 0 0} \
	-when "!A*!C*!D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001} \
	-pinlist {A B C D Y Z} \
	-vector {0 R 0 1 0 0} \
	-when "!A*!C*D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 R 1 0 0 0} \
	-when "!A*C*!D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 R 1 0 1 0} \
	-when "!A*C*!D*Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 R 1 1 0 0} \
	-when "!A*C*D*!Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 R 1 1 1 0} \
	-when "!A*C*D*Y*!Z" \
	-pin B \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 F 0 0 0} \
	-when "!A*!B*!D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 F 1 0 0} \
	-when "!A*!B*D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 F 0 0 0} \
	-when "!A*B*!D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 F 1 0 0} \
	-when "!A*B*D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 F 0 0 0} \
	-when "A*!B*!D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010 1011} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 F 1 0 0} \
	-when "A*!B*D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 F 0 0 1} \
	-when "A*B*!D*!Y*Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110 1111} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 F 1 0 1} \
	-when "A*B*D*!Y*Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 R 0 0 0} \
	-when "!A*!B*!D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 R 0 0 0} \
	-when "!A*B*!D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 R 0 0 0} \
	-when "A*!B*!D*!Y*!Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 R 0 0 1} \
	-when "A*B*!D*!Y*Z" \
	-pin C \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 0 F 0 0} \
	-when "!A*!B*!C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 F 0 0} \
	-when "!A*!B*C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 F 1 0} \
	-when "!A*!B*C*Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0101} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 0 F 0 0} \
	-when "!A*B*!C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 F 0 0} \
	-when "!A*B*C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 F 1 0} \
	-when "!A*B*C*Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1001} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 0 F 0 0} \
	-when "A*!B*!C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010 1011} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 F 0 0} \
	-when "A*!B*C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 F 1 0} \
	-when "A*!B*C*Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1101} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 0 F 0 1} \
	-when "A*B*!C*!Y*Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110 1111} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 F 0 1} \
	-when "A*B*C*!Y*Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 F 1 1} \
	-when "A*B*C*Y*Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 0 R 0 0} \
	-when "!A*!B*!C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0000 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 R 0 0} \
	-when "!A*!B*C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0001 0011 0010} \
	-pinlist {A B C D Y Z} \
	-vector {0 0 1 R 1 0} \
	-when "!A*!B*C*Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 0 R 0 0} \
	-when "!A*B*!C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0100 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 R 0 0} \
	-when "!A*B*C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {0101 0111 0110} \
	-pinlist {A B C D Y Z} \
	-vector {0 1 1 R 1 0} \
	-when "!A*B*C*Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 0 R 0 0} \
	-when "A*!B*!C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1000 1010} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 R 0 0} \
	-when "A*!B*C*!Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1001 1011 1010} \
	-pinlist {A B C D Y Z} \
	-vector {1 0 1 R 1 0} \
	-when "A*!B*C*Y*!Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 0 R 0 1} \
	-when "A*B*!C*!Y*Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1100 1110} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 R 0 1} \
	-when "A*B*C*!Y*Z" \
	-pin D \
	{ MIX }

define_arc \
	-type hidden \
	-prevector_pinlist {A B C D} \
	-prevector {1101 1111 1110} \
	-pinlist {A B C D Y Z} \
	-vector {1 1 1 R 1 1} \
	-when "A*B*C*Y*Z" \
	-pin D \
	{ MIX }

define_leakage -when "!A*!B*!C*!D*!Y*!Z" { MIX }
define_leakage -when "!A*!B*!C*D*!Y*!Z" { MIX }
define_leakage -when "!A*!B*C*!D*!Z" { MIX }
define_leakage -when "!A*!B*C*D*!Z" { MIX }
define_leakage -when "!A*B*!C*!D*!Y*!Z" { MIX }
define_leakage -when "!A*B*!C*D*!Y*!Z" { MIX }
define_leakage -when "!A*B*C*!D*!Z" { MIX }
define_leakage -when "!A*B*C*D*!Z" { MIX }
define_leakage -when "A*!B*!C*!D*!Y*!Z" { MIX }
define_leakage -when "A*!B*!C*D*!Y*!Z" { MIX }
define_leakage -when "A*!B*C*!D*!Z" { MIX }
define_leakage -when "A*!B*C*D*!Z" { MIX }
define_leakage -when "A*B*!C*!D*!Y*Z" { MIX }
define_leakage -when "A*B*!C*D*!Y*Z" { MIX }
define_leakage -when "A*B*C*!D*Z" { MIX }
define_leakage -when "A*B*C*D*Z" { MIX }
define_arc \
	-prevector_pinlist {C D E} \
	-prevector {010 110 100} \
	-type combinational \
	-pinlist {C D E Z2} \
	-vector {F 0 0 F} \
	-when "!D*!E" \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-type combinational \
	-pinlist {C D E Z2} \
	-vector {F 1 0 F} \
	-when "D*!E" \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {010} \
	-pinlist {C D E Z2} \
	-vector {R 1 0 R} \
	-when "D*!E" \
	-related_pin C \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {001} \
	-type combinational \
	-pinlist {C D E Z2} \
	-vector {0 0 F F} \
	-when "!C*!D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {011} \
	-type combinational \
	-pinlist {C D E Z2} \
	-vector {0 1 F F} \
	-when "!C*D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {001 101} \
	-type combinational \
	-pinlist {C D E Z2} \
	-vector {1 0 F F} \
	-when "C*!D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-prevector_pinlist {C D E} \
	-prevector {001 101 111} \
	-type combinational \
	-pinlist {C D E Z2} \
	-vector {1 1 F F} \
	-when "C*D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {000} \
	-pinlist {C D E Z2} \
	-vector {0 0 R R} \
	-when "!C*!D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {010} \
	-pinlist {C D E Z2} \
	-vector {0 1 R R} \
	-when "!C*D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {000 100} \
	-pinlist {C D E Z2} \
	-vector {1 0 R R} \
	-when "C*!D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type combinational \
	-prevector_pinlist {C D E} \
	-prevector {000 100 110} \
	-pinlist {C D E Z2} \
	-vector {1 1 R R} \
	-when "C*D" \
	-related_pin E \
	-pin Z2 \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000 100} \
	-pinlist {C D E Z2} \
	-vector {F 0 0 0} \
	-when "!D*!E*!Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {101} \
	-pinlist {C D E Z2} \
	-vector {F 0 1 1} \
	-when "!D*E*Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000 100 110} \
	-pinlist {C D E Z2} \
	-vector {F 1 0 0} \
	-when "D*!E*!Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {111} \
	-pinlist {C D E Z2} \
	-vector {F 1 1 1} \
	-when "D*E*Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000} \
	-pinlist {C D E Z2} \
	-vector {R 0 0 0} \
	-when "!D*!E*!Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {001} \
	-pinlist {C D E Z2} \
	-vector {R 0 1 1} \
	-when "!D*E*Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {011} \
	-pinlist {C D E Z2} \
	-vector {R 1 1 1} \
	-when "D*E*Z2" \
	-pin C \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010} \
	-pinlist {C D E Z2} \
	-vector {0 F 0 0} \
	-when "!C*!E*!Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {011} \
	-pinlist {C D E Z2} \
	-vector {0 F 1 1} \
	-when "!C*E*Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000 100 110} \
	-pinlist {C D E Z2} \
	-vector {1 F 0 0} \
	-when "C*!E*!Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-pinlist {C D E Z2} \
	-vector {1 F 0 1} \
	-when "C*!E*Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {111} \
	-pinlist {C D E Z2} \
	-vector {1 F 1 1} \
	-when "C*E*Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000} \
	-pinlist {C D E Z2} \
	-vector {0 R 0 0} \
	-when "!C*!E*!Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {001} \
	-pinlist {C D E Z2} \
	-vector {0 R 1 1} \
	-when "!C*E*Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {000 100} \
	-pinlist {C D E Z2} \
	-vector {1 R 0 0} \
	-when "C*!E*!Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010 110 100} \
	-pinlist {C D E Z2} \
	-vector {1 R 0 1} \
	-when "C*!E*Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {101} \
	-pinlist {C D E Z2} \
	-vector {1 R 1 1} \
	-when "C*E*Z2" \
	-pin D \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {011 111 101} \
	-pinlist {C D E Z2} \
	-vector {1 0 F 1} \
	-when "C*!D*Z2" \
	-pin E \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {011 111} \
	-pinlist {C D E Z2} \
	-vector {1 1 F 1} \
	-when "C*D*Z2" \
	-pin E \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010 110 100} \
	-pinlist {C D E Z2} \
	-vector {1 0 R 1} \
	-when "C*!D*Z2" \
	-pin E \
	{ TRW }

define_arc \
	-type hidden \
	-prevector_pinlist {C D E} \
	-prevector {010 110} \
	-pinlist {C D E Z2} \
	-vector {1 1 R 1} \
	-when "C*D*Z2" \
	-pin E \
	{ TRW }

define_leakage -when "!C*!D*!E*!Z2" { TRW }
define_leakage -when "!C*!D*E*Z2" { TRW }
define_leakage -when "!C*D*!E*!Z2" { TRW }
define_leakage -when "!C*D*E*Z2" { TRW }
define_leakage -when "C*!D*E*Z2" { TRW }
define_leakage -when "C*D*E*Z2" { TRW }
define_arc \
	-prevector_pinlist {A Q_st} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A Q_st Q} \
	-vector {F 0 F} \
	-when "!Q_st" \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-prevector_pinlist {A Q_st} \
	-prevector {00 01} \
	-pinlist {A Q_st Q} \
	-vector {R 1 R} \
	-when "Q_st" \
	-related_pin A \
	-pin Q \
	{ COLL }

define_arc \
	-prevector_pinlist {A Q_st} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A Q_st Q} \
	-vector {0 F F} \
	-when "!A" \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type combinational \
	-prevector_pinlist {A Q_st} \
	-prevector {00 10} \
	-pinlist {A Q_st Q} \
	-vector {1 R R} \
	-when "A" \
	-related_pin Q_st \
	-pin Q \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {00 10} \
	-pinlist {A Q_st Q} \
	-vector {F 0 0} \
	-when "!Q*!Q_st" \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {11} \
	-pinlist {A Q_st Q} \
	-vector {F 1 1} \
	-when "Q*Q_st" \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {00} \
	-pinlist {A Q_st Q} \
	-vector {R 0 0} \
	-when "!Q*!Q_st" \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {11 01} \
	-pinlist {A Q_st Q} \
	-vector {R 1 1} \
	-when "Q*Q_st" \
	-pin A \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {00 01} \
	-pinlist {A Q_st Q} \
	-vector {0 F 0} \
	-when "!A*!Q" \
	-pin Q_st \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {11} \
	-pinlist {A Q_st Q} \
	-vector {1 F 1} \
	-when "A*Q" \
	-pin Q_st \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {00} \
	-pinlist {A Q_st Q} \
	-vector {0 R 0} \
	-when "!A*!Q" \
	-pin Q_st \
	{ COLL }

define_arc \
	-type hidden \
	-prevector_pinlist {A Q_st} \
	-prevector {11 10} \
	-pinlist {A Q_st Q} \
	-vector {1 R 1} \
	-when "A*Q" \
	-pin Q_st \
	{ COLL }

define_leakage -when "!A*!Q*!Q_st" { COLL }
define_leakage -when "A*Q*Q_st" { COLL }
define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-vector {F 0 F X X} \
	-when "!B" \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {R 1 R X X} \
	-when "B" \
	-related_pin A \
	-pin Q \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-vector {0 F F X X} \
	-when "!A" \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {1 R R X X} \
	-when "A" \
	-related_pin B \
	-pin Q \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-vector {F 0 X F X} \
	-when "!B" \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {R 1 X R X} \
	-when "B" \
	-related_pin A \
	-pin Qc \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-vector {0 F X F X} \
	-when "!A" \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {1 R X R X} \
	-when "A" \
	-related_pin B \
	-pin Qc \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-vector {R 1 X X F} \
	-when "B" \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {F 0 X X R} \
	-when "!B" \
	-related_pin A \
	-pin Qn \
	{ C2P }

define_arc \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-type combinational \
	-pinlist {A B Q Qc Qn} \
	-vector {1 R X X F} \
	-when "A" \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type combinational \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {0 F X X R} \
	-when "!A" \
	-related_pin B \
	-pin Qn \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {F 0 0 0 1} \
	-when "!B*!Q*!Qc*Qn" \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q Qc Qn} \
	-vector {F 1 1 1 0} \
	-when "B*Q*Qc*!Qn" \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q Qc Qn} \
	-vector {R 0 0 0 1} \
	-when "!B*!Q*!Qc*Qn" \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {R 1 1 1 0} \
	-when "B*Q*Qc*!Qn" \
	-pin A \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00 01} \
	-pinlist {A B Q Qc Qn} \
	-vector {0 F 0 0 1} \
	-when "!A*!Q*!Qc*Qn" \
	-pin B \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11} \
	-pinlist {A B Q Qc Qn} \
	-vector {1 F 1 1 0} \
	-when "A*Q*Qc*!Qn" \
	-pin B \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {00} \
	-pinlist {A B Q Qc Qn} \
	-vector {0 R 0 0 1} \
	-when "!A*!Q*!Qc*Qn" \
	-pin B \
	{ C2P }

define_arc \
	-type hidden \
	-prevector_pinlist {A B} \
	-prevector {11 10} \
	-pinlist {A B Q Qc Qn} \
	-vector {1 R 1 1 0} \
	-when "A*Q*Qc*!Qn" \
	-pin B \
	{ C2P }

define_leakage -when "!A*!B*!Q*!Qc*Qn" { C2P }
define_leakage -when "A*B*Q*Qc*!Qn" { C2P }
define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {F 0 0 F} \
	-when "!B*!R" \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {R 1 0 R} \
	-when "B*!R" \
	-related_pin A \
	-pin Q \
	{ RC2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-type combinational \
	-pinlist {A B R Q} \
	-vector {0 F 0 F} \
	-when "!A*!R" \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-type combinational \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {1 R 0 R} \
	-when "A*!R" \
	-related_pin B \
	-pin Q \
	{ RC2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-type async \
	-pinlist {A B R Q} \
	-vector {0 1 R F} \
	-when "!A*B" \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-type async \
	-pinlist {A B R Q} \
	-vector {1 0 R F} \
	-when "A*!B" \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-type async \
	-pinlist {A B R Q} \
	-vector {1 1 R F} \
	-when "A*B" \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type async \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 1 F R} \
	-when "A*B" \
	-related_pin R \
	-pin Q \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {F 0 0 0} \
	-when "!B*!Q*!R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {F 0 1 0} \
	-when "!B*!Q*R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-pinlist {A B R Q} \
	-vector {F 1 0 1} \
	-when "B*Q*!R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {F 1 1 0} \
	-when "B*!Q*R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {R 0 0 0} \
	-when "!B*!Q*!R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {R 0 1 0} \
	-when "!B*!Q*R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110 010} \
	-pinlist {A B R Q} \
	-vector {R 1 0 1} \
	-when "B*Q*!R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {R 1 1 0} \
	-when "B*!Q*R" \
	-pin A \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {0 F 0 0} \
	-when "!A*!Q*!R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {0 F 1 0} \
	-when "!A*!Q*R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110} \
	-pinlist {A B R Q} \
	-vector {1 F 0 1} \
	-when "A*Q*!R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {111} \
	-pinlist {A B R Q} \
	-vector {1 F 1 0} \
	-when "A*!Q*R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {0 R 0 0} \
	-when "!A*!Q*!R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {0 R 1 0} \
	-when "!A*!Q*R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {110 100} \
	-pinlist {A B R Q} \
	-vector {1 R 0 1} \
	-when "A*Q*!R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {1 R 1 0} \
	-when "A*!Q*R" \
	-pin B \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {001} \
	-pinlist {A B R Q} \
	-vector {0 0 F 0} \
	-when "!A*!B*!Q" \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {011} \
	-pinlist {A B R Q} \
	-vector {0 1 F 0} \
	-when "!A*B*!Q" \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {101} \
	-pinlist {A B R Q} \
	-vector {1 0 F 0} \
	-when "A*!B*!Q" \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000} \
	-pinlist {A B R Q} \
	-vector {0 0 R 0} \
	-when "!A*!B*!Q" \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 010} \
	-pinlist {A B R Q} \
	-vector {0 1 R 0} \
	-when "!A*B*!Q" \
	-pin R \
	{ RC2 }

define_arc \
	-type hidden \
	-prevector_pinlist {A B R} \
	-prevector {000 100} \
	-pinlist {A B R Q} \
	-vector {1 0 R 0} \
	-when "A*!B*!Q" \
	-pin R \
	{ RC2 }

define_leakage -when "!A*!B*!Q*!R" { RC2 }
define_leakage -when "!A*!B*!Q*R" { RC2 }
define_leakage -when "!A*B*!Q*R" { RC2 }
define_leakage -when "A*!B*!Q*R" { RC2 }
define_leakage -when "A*B*Q*!R" { RC2 }
define_leakage -when "A*B*!Q*R" { RC2 }
